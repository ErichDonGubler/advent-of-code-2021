use anyhow::{anyhow, Context};
use std::str::FromStr;

fn parse_measurements(
    input: &str,
) -> impl Iterator<Item = (usize, Result<u16, <u16 as FromStr>::Err>)> + '_ {
    input
        .lines()
        .enumerate()
        .map(|(idx, l)| (idx, l.trim()))
        .filter(|(_idx, l)| !l.is_empty())
        .map(|(idx, l)| (idx, l.parse()))
}

pub fn iter_increasing_measurements(input: &str) -> impl Iterator<Item = (usize, u16)> + '_ {
    let mut measurements = parse_measurements(input).map(|(idx, res)| {
        (
            idx,
            res.with_context(|| anyhow!("line {} sux", idx)).unwrap(),
        )
    });
    let (_idx, mut last) = measurements.next().context("y u no measurements").unwrap();
    measurements.filter(move |&(_idx, next)| {
        let is_increasing = next > last;
        last = next;
        is_increasing
    })
}

pub const EXAMPLE: &str = "\
199
200
208
210
200
207
240
269
260
263
";

#[test]
fn part_1_example() {
    assert_eq!(
        iter_increasing_measurements(EXAMPLE).collect::<Vec<_>>(),
        &[
            (1, 200),
            (2, 208),
            (3, 210),
            (5, 207),
            (6, 240),
            (7, 269),
            (9, 263),
        ],
    );
}

pub const INPUT: &str = include_str!("day1_part1.txt");

#[test]
fn part_1() {
    assert_eq!(iter_increasing_measurements(INPUT).count(), 1288);
}

pub fn iter_increasing_3_window_sums(input: &str) -> impl Iterator<Item = (usize, u16)> + '_ {
    let measurements = parse_measurements(input)
        .map(|(idx, res)| {
            (
                idx,
                res.with_context(|| anyhow!("line {} sux", idx)).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut windows = measurements.windows(3);

    let calc_sum = |&[(window_start_idx, n_1), (_, n_2), (_, n_3)]: &[(usize, u16); 3]| {
        let context = || anyhow!("ugh, addition of window {} blew up", window_start_idx);
        (
            window_start_idx,
            n_1.checked_add(n_2)
                .with_context(context)
                .unwrap()
                .checked_add(n_3)
                .with_context(context)
                .unwrap(),
        )
    };

    let (_idx, mut last_sum) = calc_sum(
        windows
            .next()
            .context("y u no measurements window")
            .unwrap()
            .try_into()
            .unwrap(),
    );
    windows
        .filter_map(move |window| {
            let (idx, sum) = calc_sum(window.try_into().unwrap());
            let ret = (sum > last_sum).then(|| (idx, sum));
            last_sum = sum;
            ret
        })
        .collect::<Vec<_>>()
        .into_iter()
}

#[test]
fn part_2_example() {
    assert_eq!(
        iter_increasing_3_window_sums(EXAMPLE).collect::<Vec<_>>(),
        &[(1, 618), (4, 647), (5, 716), (6, 769), (7, 792)],
    )
}

#[test]
fn part_2() {
    assert_eq!(iter_increasing_3_window_sums(INPUT).count(), 1311);
}
