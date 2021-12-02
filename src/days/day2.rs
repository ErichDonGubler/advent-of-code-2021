use std::str::FromStr;

use anyhow::{anyhow, bail, Context};

pub const EXAMPLE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Part1Submarine {
    horizontal_pos: u32,
    depth: u32,
}

impl Part1Submarine {
    pub fn new() -> Self {
        Self {
            horizontal_pos: 0,
            depth: 0,
        }
    }

    pub fn exec_cmd(&mut self, cmd: SubmarineCommand) {
        let Self {
            horizontal_pos,
            depth,
        } = self;
        match cmd {
            SubmarineCommand::Forward(value) => {
                *horizontal_pos = horizontal_pos.checked_add(value.into()).unwrap()
            }
            SubmarineCommand::Up(value) => *depth = depth.checked_sub(value.into()).unwrap(),
            SubmarineCommand::Down(value) => *depth = depth.checked_add(value.into()).unwrap(),
        }
    }
}

#[test]
fn part1_example() {
    let mut submarine = Part1Submarine::new();
    SubmarineCommand::iter_from_lines(EXAMPLE).for_each(|cmd| submarine.exec_cmd(cmd));
    assert_eq!(
        submarine,
        Part1Submarine {
            horizontal_pos: 15,
            depth: 10,
        }
    );
}

pub enum SubmarineCommand {
    Forward(u8),
    Up(u8),
    Down(u8),
}

impl SubmarineCommand {
    pub fn iter_from_lines(input: &str) -> impl Iterator<Item = SubmarineCommand> + '_ {
        input
            .lines()
            .map(|l| l.trim())
            .enumerate()
            .filter(|(_idx, l)| !l.is_empty())
            .map(|(idx, l)| (idx, l.parse()))
            .map(|(idx, res)| {
                res.with_context(|| anyhow!("failed to parse line {}", idx))
                    .unwrap()
            })
    }
}

impl FromStr for SubmarineCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_discriminant, raw_value) = s
            .split_once(' ')
            .context("no space found to split discriminant and value")?;

        let value = || {
            raw_value
                .parse()
                .with_context(|| anyhow!("failed to parse {:?} as value", raw_value))
        };

        let cmd = match raw_discriminant {
            "forward" => Self::Forward(value()?),
            "down" => Self::Down(value()?),
            "up" => Self::Up(value()?),
            unrecognized => bail!("unrecognized discriminant {:?}", unrecognized),
        };

        Ok(cmd)
    }
}

pub const INPUT: &str = include_str!("day2_input.txt");

#[test]
fn part1() {
    let mut submarine = Part1Submarine::new();
    SubmarineCommand::iter_from_lines(INPUT).for_each(|cmd| submarine.exec_cmd(cmd));
    assert_eq!(
        submarine,
        Part1Submarine {
            horizontal_pos: 1965,
            depth: 1182,
        }
    );

    let Part1Submarine {
        horizontal_pos,
        depth,
    } = submarine;
    assert_eq!(horizontal_pos.checked_mul(depth).unwrap(), 2322630)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Part2Submarine {
    aim: u32,

    horizontal_pos: u32,
    depth: u32,
}

impl Part2Submarine {
    pub fn new() -> Self {
        Self {
            aim: 0,
            horizontal_pos: 0,
            depth: 0,
        }
    }

    pub fn exec_cmd(&mut self, cmd: SubmarineCommand) {
        let Self {
            aim,
            horizontal_pos,
            depth,
        } = self;
        match cmd {
            SubmarineCommand::Forward(value) => {
                *horizontal_pos = horizontal_pos.checked_add(value.into()).unwrap();
                *depth = depth
                    .checked_add(aim.checked_mul(value.into()).unwrap())
                    .unwrap();
            }
            SubmarineCommand::Up(value) => *aim = aim.checked_sub(value.into()).unwrap(),
            SubmarineCommand::Down(value) => *aim = aim.checked_add(value.into()).unwrap(),
        }
    }
}

#[test]
fn part2_example() {
    let mut submarine = Part2Submarine::new();
    SubmarineCommand::iter_from_lines(EXAMPLE).for_each(|cmd| submarine.exec_cmd(cmd));

    assert_eq!(
        submarine,
        Part2Submarine {
            aim: 10,
            horizontal_pos: 15,
            depth: 60,
        }
    );
}

#[test]
fn part2() {
    let mut submarine = Part2Submarine::new();
    SubmarineCommand::iter_from_lines(INPUT).for_each(|cmd| submarine.exec_cmd(cmd));

    let Part2Submarine {
        aim: _,
        horizontal_pos,
        depth,
    } = submarine;
    assert_eq!(horizontal_pos.checked_mul(depth).unwrap(), 2105273490);
}
