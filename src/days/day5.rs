use anyhow::{anyhow, Context};
use std::{
    cmp::Ordering,
    ops::{Index, IndexMut},
    str::FromStr,
};

const EXAMPLE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

#[derive(Clone, Debug, Eq, PartialEq)]
struct Coordinate(usize, usize);

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').context("no comma found")?;

        Ok(Self(
            x.parse().context("failed to parse `x`")?,
            y.parse().context("failed to parse `y`")?,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct VentLine {
    from: Coordinate,
    to: Coordinate,
}

impl FromStr for VentLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s
            .split_once(" -> ")
            .context("missing line operator ` -> `")?;
        Ok(Self {
            from: from.parse().context("failed to parse `from` coordinate")?,
            to: to.parse().context("failed to parse `to` coordinate")?,
        })
    }
}

fn parse_vent_lines(input: &str) -> impl Iterator<Item = VentLine> + '_ {
    input.lines().map(|l| l.trim()).enumerate().map(|(idx, l)| {
        l.parse()
            .with_context(|| anyhow!("failed to parse line {} as a vent line", idx))
            .unwrap()
    })
}

struct Map<T> {
    num_columns: usize,
    num_rows: usize,
    tiles: Vec<T>,
}

impl<T> Map<T> {
    pub fn new(num_columns: usize, num_rows: usize, init: T) -> Self
    where
        T: Clone,
    {
        let tiles = vec![init.clone(); num_columns.checked_mul(num_rows).unwrap()];

        Self {
            num_columns,
            num_rows,
            tiles,
        }
    }

    fn idx_to_coord(num_columns: usize, num_rows: usize, idx: usize) -> Coordinate {
        assert!(idx < num_columns * num_rows);
        Coordinate(idx % num_columns, idx / num_columns)
    }

    fn coord_to_idx(num_columns: usize, num_rows: usize, coord: Coordinate) -> usize {
        let Coordinate(x, y) = coord;
        assert!(num_columns > x);
        assert!(num_rows > y);
        y * num_columns + x // no need for overflow checking because we've already allocated this
    }

    pub fn into_iter(self) -> impl Iterator<Item = (Coordinate, T)> {
        let Self {
            num_rows,
            num_columns,
            tiles,
        } = self;

        tiles
            .into_iter()
            .enumerate()
            .map(move |(idx, t)| (Self::idx_to_coord(num_columns, num_rows, idx), t))
    }
}

impl<T> Index<Coordinate> for Map<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        let &Self {
            num_columns,
            num_rows,
            ref tiles,
        } = self;
        &tiles[Self::coord_to_idx(num_columns, num_rows, index)]
    }
}

impl<T> IndexMut<Coordinate> for Map<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        let &mut Self {
            num_columns,
            num_rows,
            ref mut tiles,
        } = self;
        &mut tiles[Self::coord_to_idx(num_columns, num_rows, index)]
    }
}

fn map_danger_levels_part1<I>(vent_lines: I) -> impl Iterator<Item = (Coordinate, u32)>
where
    I: IntoIterator<Item = VentLine>,
    I::IntoIter: Clone,
{
    let vent_lines = vent_lines.into_iter();

    let mut map = {
        let (map_x, map_y) = vent_lines
            .clone()
            .fold((0, 0), |(x, y), VentLine { from, to }| {
                (x.max(from.0).max(to.0), y.max(from.1).max(to.1))
            });
        Map::new(
            map_x.checked_add(1).unwrap(),
            map_y.checked_add(1).unwrap(),
            0u32,
        )
    };

    // only work with orthogonal lines for now
    vent_lines.for_each(|VentLine { from, to }| {
        if from.0 == to.0 {
            let y_iter = if from.1 > to.1 {
                to.1..=from.1
            } else {
                from.1..=to.1
            };
            y_iter.for_each(|y| {
                let tile = &mut map[Coordinate(from.0, y)];
                *tile = tile.checked_add(1).unwrap();
            })
        } else if from.1 == to.1 {
            let x_iter = if from.0 > to.0 {
                to.0..=from.0
            } else {
                from.0..=to.0
            };
            x_iter.for_each(|x| {
                let tile = &mut map[Coordinate(x, from.1)];
                *tile = tile.checked_add(1).unwrap();
            })
        }
    });

    map.into_iter().filter(|&(ref _idx, t)| (t >= 2))
}

#[test]
fn part1_example() {
    let vent_lines = parse_vent_lines(EXAMPLE).collect::<Vec<VentLine>>();
    assert_eq!(
        vent_lines,
        [
            VentLine {
                from: Coordinate(0, 9),
                to: Coordinate(5, 9),
            },
            VentLine {
                from: Coordinate(8, 0),
                to: Coordinate(0, 8),
            },
            VentLine {
                from: Coordinate(9, 4),
                to: Coordinate(3, 4),
            },
            VentLine {
                from: Coordinate(2, 2),
                to: Coordinate(2, 1),
            },
            VentLine {
                from: Coordinate(7, 0),
                to: Coordinate(7, 4),
            },
            VentLine {
                from: Coordinate(6, 4),
                to: Coordinate(2, 0),
            },
            VentLine {
                from: Coordinate(0, 9),
                to: Coordinate(2, 9),
            },
            VentLine {
                from: Coordinate(3, 4),
                to: Coordinate(1, 4),
            },
            VentLine {
                from: Coordinate(0, 0),
                to: Coordinate(8, 8),
            },
            VentLine {
                from: Coordinate(5, 5),
                to: Coordinate(8, 2),
            },
        ]
    );

    assert_eq!(
        map_danger_levels_part1(vent_lines).collect::<Vec<_>>(),
        [
            (Coordinate(3, 4), 2),
            (Coordinate(7, 4), 2),
            (Coordinate(0, 9), 2),
            (Coordinate(1, 9), 2),
            (Coordinate(2, 9), 2),
        ]
    );
}

const INPUT: &str = include_str!("./day5_input.txt");

#[test]
fn part1() {
    assert_eq!(
        map_danger_levels_part1(parse_vent_lines(INPUT).collect::<Vec<_>>()).count(),
        4728,
    );
}

fn map_danger_levels_part2<I>(vent_lines: I) -> impl Iterator<Item = (Coordinate, u32)>
where
    I: IntoIterator<Item = VentLine>,
    I::IntoIter: Clone,
{
    let vent_lines = vent_lines.into_iter();

    let mut map = {
        let (map_x, map_y) = vent_lines
            .clone()
            .fold((0, 0), |(x, y), VentLine { from, to }| {
                (x.max(from.0).max(to.0), y.max(from.1).max(to.1))
            });
        Map::new(
            map_x.checked_add(1).unwrap(),
            map_y.checked_add(1).unwrap(),
            0u32,
        )
    };

    // only work with orthogonal lines for now
    vent_lines.for_each(|VentLine { from, to }| {
        let abs_diff_and_increment = |x: usize, y| -> (usize, isize) {
            match x.cmp(&y) {
                Ordering::Equal => (0, 0),
                Ordering::Greater => (x.wrapping_sub(y), -1),
                Ordering::Less => (y.wrapping_sub(x), 1),
            }
        };
        let (abs_diff_x, inc_x) = abs_diff_and_increment(from.0, to.0);
        let (abs_diff_y, inc_y) = abs_diff_and_increment(from.1, to.1);
        assert!(
            abs_diff_x == 0 || abs_diff_y == 0 || (abs_diff_x == abs_diff_y),
            "non-line found",
        );
        let num_tiles = isize::try_from(abs_diff_x.max(abs_diff_y))
            .expect("diff is greater than range of `isize`");

        (0..=num_tiles).for_each(|idx| {
            let coord = Coordinate(
                ((from.0 as isize) + (idx * inc_x)) as usize,
                ((from.1 as isize) + (idx * inc_y)) as usize,
            );
            map[coord] = map[coord.clone()].checked_add(1).unwrap();
        });
    });

    map.into_iter().filter(|&(ref _idx, t)| (t >= 2))
}

#[test]
fn part2_example() {
    assert_eq!(
        map_danger_levels_part2(parse_vent_lines(EXAMPLE).collect::<Vec<_>>()).collect::<Vec<_>>(),
        [
            (Coordinate(7, 1), 2),
            (Coordinate(2, 2), 2),
            (Coordinate(5, 3), 2),
            (Coordinate(7, 3), 2),
            (Coordinate(3, 4), 2),
            (Coordinate(4, 4), 3),
            (Coordinate(6, 4), 3),
            (Coordinate(7, 4), 2),
            (Coordinate(5, 5), 2),
            (Coordinate(0, 9), 2),
            (Coordinate(1, 9), 2),
            (Coordinate(2, 9), 2),
        ],
    );
}

#[test]
fn part2() {
    assert_eq!(
        map_danger_levels_part2(parse_vent_lines(INPUT).collect::<Vec<_>>()).count(),
        17717,
    );
}
