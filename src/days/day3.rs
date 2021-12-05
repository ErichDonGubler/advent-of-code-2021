use anyhow::Context;
use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    mem::size_of,
    ops::{Not, Shl},
};
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

pub fn parse_diagnostic_report(input: &str) -> impl Clone + Iterator<Item = Sample> + '_ {
    let lines = input.lines().map(|l| l.trim());

    let line_len = lines
        .clone()
        .next()
        .context("y u no have line")
        .unwrap()
        .len();
    Sample::check_width(line_len).unwrap();

    lines
        .inspect(move |l| assert_eq!(l.len(), line_len))
        .map(move |l| {
            let data = l.chars().fold(0, |acc, c| {
                (acc << 1)
                    | match c {
                        '0' => 0,
                        '1' => 1,
                        c => panic!("blarg invalid character {:?}", c),
                    }
            });
            Sample {
                data,
                width: line_len,
            }
        })
}

pub struct SampleBitsStats {
    counts_of_0s_and_1s: Vec<i32>,
}

impl SampleBitsStats {
    pub fn new(iter: impl Iterator<Item = Sample>) -> Option<Self> {
        let mut iter = iter.peekable();

        let sample_width = iter.peek()?.width();

        let mut counts_of_0s_and_1s = vec![0i32; sample_width];

        iter.for_each(|sample| {
            (0..sample_width).for_each(|idx| {
                counts_of_0s_and_1s[idx] = counts_of_0s_and_1s[idx]
                    .checked_add(if sample.is_bit_set(idx) { 1 } else { -1 })
                    .expect("wat, {under,over}flow?");
            });
        });

        Some(Self {
            counts_of_0s_and_1s,
        })
    }

    pub fn counts_of_0s_and_1s(&self) -> &[i32] {
        &self.counts_of_0s_and_1s
    }

    pub fn sample_width(&self) -> usize {
        self.counts_of_0s_and_1s.len()
    }
}

pub fn most_common_bits_part1(bit_stats: &SampleBitsStats) -> Sample {
    let most_common_bits =
        bit_stats
            .counts_of_0s_and_1s()
            .iter()
            .fold(0, |most_common_bits, &bucket| {
                (most_common_bits << 1)
                    | match bucket.cmp(&0) {
                        Ordering::Equal => panic!("IDK what to do with this man"),
                        Ordering::Greater => 1,
                        Ordering::Less => 0,
                    }
            });

    Sample::new(most_common_bits, bit_stats.sample_width())
}

pub fn most_common_bits_part2(bit_stats: &SampleBitsStats) -> Sample {
    let most_common_bits =
        bit_stats
            .counts_of_0s_and_1s()
            .iter()
            .fold(0, |most_common_bits, &bucket| {
                (most_common_bits << 1)
                    | match bucket.cmp(&0) {
                        Ordering::Equal => 1,
                        Ordering::Greater => 1,
                        Ordering::Less => 0,
                    }
            });

    Sample::new(most_common_bits, bit_stats.sample_width())
}

pub fn least_common_bits_part2(bit_stats: &SampleBitsStats) -> Sample {
    let most_common_bits =
        bit_stats
            .counts_of_0s_and_1s()
            .iter()
            .fold(0, |most_common_bits, &bucket| {
                (most_common_bits << 1)
                    | match bucket.cmp(&0) {
                        Ordering::Equal => 0,
                        Ordering::Greater => 0,
                        Ordering::Less => 1,
                    }
            });

    Sample::new(most_common_bits, bit_stats.sample_width())
}

pub fn gamma(most_common_bits: Sample) -> Sample {
    most_common_bits
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

impl Display for Sample {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let &Self { data, width } = self;
        write!(f, "{:0width$b}", data, width = width)
    }
}

impl Not for Sample {
    type Output = Self;

    fn not(self) -> Self::Output {
        let Self { data, width } = self;
        Self::masked(!data, width)
    }
}

impl Shl<usize> for Sample {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let Self { data, width } = self;
        Self::masked(data << rhs, width)
    }
}

#[derive(Debug, ThisError)]
#[error("{checked} is too wide for signal representation with max size of {max}")]
struct SignalWidthError {
    checked: usize,
    max: usize,
}

impl Sample {
    fn mask(width: usize) -> u32 {
        !(!0 << width)
    }

    pub fn bitmask(&self) -> u32 {
        Self::mask(self.width)
    }

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

    pub fn masked(data: u32, width: usize) -> Self {
        Self {
            data: data & Self::mask(width),
            width,
        }
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

    pub fn is_bit_set(&self, idx: usize) -> bool {
        let &Self { data, width } = self;
        assert!(idx < width);

        data & (1 << width - 1 - idx) != 0
    }
}

#[cfg(test)]
fn part1_gamma_and_epsilon(input: &str) -> (Sample, Sample) {
    let report_samples_iter = parse_diagnostic_report(input);
    let samples_bits_stats = SampleBitsStats::new(report_samples_iter).unwrap();
    let most_common_bits = most_common_bits_part1(&samples_bits_stats);
    let gamma = gamma(most_common_bits);
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
    let (gamma, epsilon) = part1_gamma_and_epsilon(INPUT);
    assert_eq!(power_consumption(gamma, epsilon), 2003336);
}

pub fn o2_generator_rating(report_samples_iter: impl Iterator<Item = Sample>) -> (usize, Sample) {
    exactly_one_for_bit_criteria(report_samples_iter, most_common_bits_part2).unwrap()
}

pub fn co2_scrubber_rating(report_samples_iter: impl Iterator<Item = Sample>) -> (usize, Sample) {
    exactly_one_for_bit_criteria(report_samples_iter, least_common_bits_part2).unwrap()
}

fn exactly_one_for_bit_criteria(
    report_samples_iter: impl Iterator<Item = Sample>,
    mut next_bits_selection_gen: impl FnMut(&SampleBitsStats) -> Sample,
) -> Result<(usize, Sample), BitCriteriaSelectionError> {
    let mut report_samples_iter = report_samples_iter.peekable();

    let sample_width = report_samples_iter
        .peek()
        .ok_or(BitCriteriaSelectionError::NoSamplesProvided)?
        .width();

    let mut report_samples = report_samples_iter.enumerate().collect::<Vec<_>>();
    for idx in 0..sample_width {
        let bit_stats_mask =
            SampleBitsStats::new(report_samples.iter().map(|(_idx, sample)| sample).cloned())
                .unwrap();
        let this_bit_set = next_bits_selection_gen(&bit_stats_mask).is_bit_set(idx);
        report_samples.retain(|&(_idx, ref sample)| sample.is_bit_set(idx) == this_bit_set);

        match report_samples.len() {
            0 => return Err(BitCriteriaSelectionError::AllCandidatesEliminated { after: idx }),
            1 => return Ok(report_samples.pop().unwrap()),
            _ => (),
        }
    }

    Err(BitCriteriaSelectionError::TooManyLeft {
        remaining: report_samples,
    })
}

pub fn life_support_rating(o2_generator_rating: Sample, co2_scrubber_rating: Sample) -> u64 {
    o2_generator_rating
        .checked_mul(&co2_scrubber_rating)
        .unwrap()
}

#[derive(Debug, ThisError)]
pub enum BitCriteriaSelectionError {
    #[error("no samples were provided")]
    NoSamplesProvided,
    #[error("not enough candidates eliminated")]
    TooManyLeft { remaining: Vec<(usize, Sample)> },
    #[error(
        "no single candidate remaining after {after} analysis iterations (?!), re-run w/ \
        debuggingg for more info"
    )]
    AllCandidatesEliminated { after: usize },
}

#[cfg(test)]
fn part2_o2_and_co2_ratings(input: &str) -> ((usize, Sample), (usize, Sample)) {
    let report_samples_iter = parse_diagnostic_report(input);
    let o2_generator_rating = o2_generator_rating(report_samples_iter.clone());
    let co2_scrubber_rating = co2_scrubber_rating(report_samples_iter);
    (o2_generator_rating, co2_scrubber_rating)
}

#[test]
fn part2_example() {
    let (o2_generator_rating, co2_scrubber_rating) = part2_o2_and_co2_ratings(EXAMPLE);
    assert_eq!(o2_generator_rating, (3, Sample::new(0b10111, 5)));
    assert_eq!(co2_scrubber_rating, (11, Sample::new(0b01010, 5)));

    let (_idx, o2_generator_rating) = o2_generator_rating;
    let (_idx, co2_scrubber_rating) = co2_scrubber_rating;

    assert_eq!(
        life_support_rating(o2_generator_rating, co2_scrubber_rating),
        230
    );
}

#[test]
fn part2() {
    let ((_, o2_generator_rating), (_, co2_scrubber_rating)) =
        part2_o2_and_co2_ratings(INPUT);

    assert_eq!(
        life_support_rating(o2_generator_rating, co2_scrubber_rating),
        1877139
    );
}
