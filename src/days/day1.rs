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
        .map(|(idx, l)| (idx, l.parse().map(|n| n)))
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

#[test]
fn part_1_example() {
    assert_eq!(
        iter_increasing_measurements(
            "\
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
                "
        )
        .collect::<Vec<_>>(),
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

#[test]
fn part_1() {
    assert_eq!(
        iter_increasing_measurements(include_str!("day1_part1.txt")).count(),
        20,
    );
}
