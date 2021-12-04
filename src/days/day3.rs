use anyhow::Context;
use std::{cmp::Ordering, marker::PhantomData, mem::size_of, ops::Not};
use thiserror::Error as ThisError;

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

pub struct DiagnosticReport<'input, I>
where
    I: Clone + Iterator<Item = (usize, char)> + 'input,
{
    sample_width: usize,
    iter: I,
    _phantom_input: PhantomData<&'input str>,
}

pub fn parse_diagnostic_report(
    input: &str,
) -> DiagnosticReport<'_, impl Clone + Iterator<Item = (usize, char)> + '_> {
    let lines = input.lines().map(|l| {
        l.trim().chars().map(|c| {
            if !matches!(c, '0' | '1') {
                panic!("blarg invalid character");
            }
            c
        })
    });

    let line_len = lines
        .clone()
        .next()
        .context("y u no have line")
        .unwrap()
        .count();
    Sample::check_width(line_len).unwrap();

    let mut last_seen_idx = line_len - 1;
    DiagnosticReport {
        _phantom_input: PhantomData,
        sample_width: line_len,
        iter: lines
            .flat_map(|cs| cs.enumerate())
            .inspect(move |&(idx, _c)| {
                if idx == 0 && last_seen_idx != (line_len - 1) {
                    panic!("lines varied in length, boo");
                }
                last_seen_idx = idx;
            }),
    }
}

impl<'input, I> DiagnosticReport<'input, I>
where
    I: Clone + Iterator<Item = (usize, char)> + 'input,
{
    pub fn iter(&self) -> I {
        self.iter.clone()
    }

    pub fn sample_width(&self) -> usize {
        self.sample_width
    }

    pub fn common_bits(&self) -> Sample {
        let mut running_avgs = vec![0i32; self.sample_width()];

        self.iter().for_each(|(idx, c)| {
            running_avgs[idx] = running_avgs[idx]
                .checked_add(match c {
                    '0' => -1,
                    '1' => 1,
                    c => panic!("blarg invalid character {:?}", c),
                })
                .expect("wat, {under,over}flow?");
        });

        let gamma = running_avgs.iter().fold(0, |gamma, &bucket| {
            (gamma << 1)
                | match bucket.cmp(&0) {
                    Ordering::Equal => panic!("IDK what to do with this man"),
                    Ordering::Greater => 1,
                    Ordering::Less => 0,
                }
        });

        Sample {
            width: self.sample_width(),
            data: gamma,
        }
    }
}

pub fn gamma(common_bits: Sample) -> Sample {
    common_bits
}

pub fn epsilon(gamma: Sample) -> Sample {
    !gamma
}

pub fn power_consumption(gamma: Sample, epsilon: Sample) -> u64 {
    gamma.checked_mul(&epsilon).unwrap()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sample {
    data: u32,
    width: usize,
}

impl Not for Sample {
    type Output = Self;

    fn not(self) -> Self::Output {
        let Self { data, width } = self;
        let mask = !(!0 << width);
        Self {
            data: !data & mask,
            width,
        }
    }
}

#[derive(Debug, ThisError)]
#[error("{checked} is too wide for signal representation with max size of {max}")]
struct SignalWidthError {
    checked: usize,
    max: usize,
}

impl Sample {
    fn max_width() -> usize {
        size_of::<u32>() * 8
    }

    fn check_width(width: usize) -> Result<(), SignalWidthError> {
        if width < Self::max_width() {
            Ok(())
        } else {
            Err(SignalWidthError {
                checked: width,
                max: Self::max_width(),
            })
        }
    }

    pub fn new(data: u32, width: usize) -> Self {
        Self::check_width(width).unwrap();
        Self { data, width }
    }

    pub fn into_inner(self) -> u32 {
        let Self { data, width: _ } = self;
        data
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn assert_compatible(&self, other: &Self) -> (u32, u32) {
        let &Self {
            data: data1,
            width: width1,
        } = self;
        let &Self {
            data: data2,
            width: width2,
        } = other;

        assert_eq!(width1, width2);

        (data1, data2)
    }

    pub fn checked_mul(&self, other: &Self) -> Option<u64> {
        let (data1, data2) = self.assert_compatible(other);

        Some(u64::from(data1).checked_mul(u64::from(data2))?)
    }
}

#[cfg(test)]
fn part1_gamma_and_epsilon(input: &str) -> (Sample, Sample) {
    let report = parse_diagnostic_report(input);
    let common_bits = report.common_bits();
    let gamma = gamma(common_bits);
    let epsilon = epsilon(gamma.clone());
    (gamma, epsilon)
}

#[test]
fn part1_example() {
    let (gamma, epsilon) = part1_gamma_and_epsilon(EXAMPLE);
    assert_eq!(gamma, Sample::new(0b10110, 5));
    assert_eq!(power_consumption(gamma, epsilon), 198);
}

pub const INPUT: &str = include_str!("day3_input.txt");

#[test]
fn part1() {
    let (gamma, epsilon) = part1_gamma_and_epsilon(EXAMPLE);
    assert_eq!(power_consumption(gamma, epsilon), 2003336);
}
