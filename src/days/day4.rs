use std::{collections::HashSet, str::FromStr};

use anyhow::Context;
use itertools::izip;

const EXAMPLE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

#[derive(Clone, Debug, Eq, PartialEq)]
struct BingoGame {
    numbers_drawn: Vec<u8>,
    player_boards: Vec<[[u8; 5]; 5]>,
}

impl FromStr for BingoGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().map(|s| s.trim());
        let numbers_drawn = lines
            .by_ref()
            .take(1)
            .next()
            .context("no lines, you suck")?
            .split(',')
            .map(|raw_num| raw_num.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        let mut player_boards = Vec::new();

        loop {
            match lines.next() {
                Some(l) if l.is_empty() => (),
                Some(l) => panic!("WTF is this line doing yo: {:?}", l),
                None => break,
            }
            let next_board: [[u8; 5]; 5] = lines
                .by_ref()
                .take(5)
                .map(|l| {
                    <[u8; 5]>::try_from(
                        dbg!(l)
                            .split_whitespace()
                            .map(|t| t.parse().unwrap())
                            .collect::<Vec<_>>(),
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            player_boards.push(next_board);
        }

        Ok(Self {
            numbers_drawn,
            player_boards,
        })
    }
}

#[test]
fn part1_example() {
    let bingo_game = BingoGame::from_str(EXAMPLE).unwrap();
    assert_eq!(
        bingo_game,
        BingoGame {
            numbers_drawn: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            player_boards: vec![
                [
                    [22, 13, 17, 11, 0],
                    [8, 2, 23, 4, 24],
                    [21, 9, 14, 16, 7],
                    [6, 10, 3, 18, 5],
                    [1, 12, 20, 15, 19],
                ],
                [
                    [3, 15, 0, 2, 22],
                    [9, 18, 13, 17, 5],
                    [19, 8, 7, 25, 23],
                    [20, 11, 10, 24, 4],
                    [14, 21, 16, 12, 6],
                ],
                [
                    [14, 21, 17, 24, 4],
                    [10, 16, 15, 9, 19],
                    [18, 8, 23, 26, 20],
                    [22, 11, 13, 6, 5],
                    [2, 0, 12, 3, 7],
                ],
            ],
        }
    );

    assert_eq!(
        winners(&bingo_game),
        Some(Winners {
            number_idx: 11,
            winners: vec![((2, vec![(2, "row", 0, [14, 21, 17, 24, 4,])]), 188)],
        }),
    );
}

#[derive(Debug, Eq, PartialEq)]
struct Winners {
    number_idx: usize,
    winners: Vec<((usize, Vec<(usize, &'static str, usize, [u8; 5])>), u32)>,
}

fn winners(bingo_game: &BingoGame) -> Option<Winners> {
    let BingoGame {
        numbers_drawn: next_numbers,
        player_boards,
    } = bingo_game;

    let mut numbers_drawn = HashSet::new();
    for (number_idx, number) in next_numbers.into_iter().enumerate() {
        numbers_drawn.insert(number);

        let winners = player_boards
            .iter()
            .enumerate()
            .filter_map(|(player_idx, board)| {
                // check rows
                let winning_rows = board
                    .iter()
                    .enumerate()
                    .filter(|(_idx, row)| row.iter().all(|number| numbers_drawn.contains(number)));

                // check columns
                let [one, two, three, four, five] = &board;
                let winning_columns = izip!(one, two, three, four, five)
                    .map(|(&one, &two, &three, &four, &five)| [one, two, three, four, five])
                    .enumerate()
                    .filter(|(_idx, col)| col.iter().all(|number| numbers_drawn.contains(number)));

                let winning_triggers = winning_rows
                    .map(|(idx, row)| (player_idx, "row", idx, row.clone()))
                    .chain(winning_columns.map(|(idx, col)| (player_idx, "column", idx, col)))
                    .collect::<Vec<_>>();

                (!winning_triggers.is_empty())
                    .then(|| (player_idx, winning_triggers))
                    .map(|stuff| {
                        (
                            stuff,
                            board
                                .iter()
                                .flat_map(|row| row.iter().copied())
                                .filter(|n| !numbers_drawn.contains(n))
                                .fold(0u32, |acc, n| acc + u32::from(n)),
                        )
                    })
            })
            .collect::<Vec<_>>();

        if !winners.is_empty() {
            return Some(Winners {
                number_idx,
                winners,
            });
        }
    }
    None
}

const INPUT: &str = include_str!("./day4_input.txt");

#[test]
fn part1() {
    let bingo_game = INPUT.parse::<BingoGame>().unwrap();
    let winners = winners(&bingo_game).unwrap();
    assert_eq!(
        winners,
        Winners {
            number_idx: 16,
            winners: vec![((45, vec![(45, "column", 2, [49, 0, 13, 69, 57])]), 919)]
        }
    );

    assert_eq!(
        u32::from(bingo_game.numbers_drawn[winners.number_idx])
            .checked_mul(winners.winners[0].1)
            .unwrap(),
        45031,
    );
}
