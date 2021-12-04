use std::{cmp::Ordering, mem::size_of};

use anyhow::Context;

pub const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

pub fn calculate_gamma(input: &str) -> (u32, usize) {
    let lines = input.lines().map(|l| l.trim().chars().map(|c| { if !matches!(c, '0' | '1') {
        panic!("blarg invalid character");
    } c }));

    let line_len = lines.clone().next().context("y u no have line").unwrap().count();

    assert!(line_len <= size_of::<u32>() * 8);
    let mut running_avgs = vec![0i32; line_len];

    lines.flat_map(|cs| cs.enumerate()).fold(line_len - 1, |last_idx, (idx, c)| {
        if idx == 0 && last_idx != (line_len - 1) {
            panic!("lines varied in length, boo");
        }

        running_avgs[idx] = running_avgs[idx].checked_add(match c {
            '0' => -1,
            '1' => 1,
            c => panic!("blarg invalid character {:?}", c),
        }).expect("wat, {under,over}flow?");

        idx
    });

    let gamma = running_avgs.iter().fold(0, |gamma, &bucket| (gamma << 1) | match bucket.cmp(&0) {
        Ordering::Equal => panic!("IDK what to do with this man"),
        Ordering::Greater => 1,
        Ordering::Less => 0,
    });
    (gamma, line_len)
}

pub fn calculate_submarine_power_consumption(gamma: u32, gamma_width: usize) -> u64 {
    let gamma: u64 = gamma as u64;
    let mask = !(!0 << gamma_width);
    gamma.checked_mul(!gamma & mask).unwrap()
}

#[test]
fn part1_example() {
    let (gamma, gamma_width) = calculate_gamma(EXAMPLE);
    assert_eq!(gamma, 0b10110);
    assert_eq!(gamma_width, 5);
    let consumption = calculate_submarine_power_consumption(gamma, gamma_width);
    assert_eq!(consumption, 198);
}

pub const INPUT: &str = include_str!("day3_input.txt");

#[test]
fn part1() {
    let (gamma, gamma_width) = calculate_gamma(INPUT);
    assert_eq!(calculate_submarine_power_consumption(gamma, gamma_width), 2003336);
}
