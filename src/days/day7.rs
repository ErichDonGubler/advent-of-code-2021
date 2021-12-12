use std::cmp::Ordering;

fn parse_crab_sub_horiz_pos(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.trim().split(',').map(|l| l.parse().unwrap())
}

const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

fn cheapest_crab_sub_alignment(
    input: &str,
    mut cost_fn: impl FnMut(u32, u32) -> u64,
) -> (u64, Vec<u32>) {
    let crab_sub_horiz_poses = parse_crab_sub_horiz_pos(input).collect::<Vec<_>>();
    let mut cheapest_fuel_consumption = u64::MAX;
    let mut cheapest_indices = Vec::with_capacity(1);
    (0..=crab_sub_horiz_poses.iter().copied().max().unwrap()).for_each(|pos| {
        let fuel_consumption = crab_sub_horiz_poses
            .iter()
            .copied()
            .map(|crab_pos| cost_fn(crab_pos, pos))
            .fold(0u64, |acc, fuel_cost| {
                acc.checked_add(fuel_cost.into()).unwrap()
            });

        match fuel_consumption.cmp(&cheapest_fuel_consumption) {
            Ordering::Less => {
                cheapest_fuel_consumption = fuel_consumption;
                cheapest_indices.clear();
                cheapest_indices.push(pos);
            }
            Ordering::Equal => {
                cheapest_indices.push(pos);
            }
            Ordering::Greater => (),
        }
    });

    (cheapest_fuel_consumption, cheapest_indices)
}

fn abs_diff(x: u32, y: u32) -> u32 {
    if x < y {
        y.wrapping_sub(x)
    } else {
        x.wrapping_sub(y)
    }
}

fn cheapest_crab_sub_alignment_part1(input: &str) -> (u64, Vec<u32>) {
    cheapest_crab_sub_alignment(input, |x, y| abs_diff(x, y).into())
}

#[test]
fn part1_example() {
    let (cheapest_fuel_consumption, cheapest_indices) = cheapest_crab_sub_alignment_part1(EXAMPLE);
    assert_eq!(cheapest_fuel_consumption, 37);
    assert_eq!(cheapest_indices, [2])
}

const INPUT: &str = include_str!("./day7_input.txt");

#[test]
fn part1() {
    let (cheapest_fuel_consumption, cheapest_indices) = cheapest_crab_sub_alignment_part1(INPUT);
    assert_eq!(cheapest_fuel_consumption, 364898);
    assert_eq!(cheapest_indices, [361])
}

fn cheapest_crab_sub_alignment_part2(input: &str) -> (u64, Vec<u32>) {
    let triangular_sequence = |n: u32| n.checked_mul(n + 1).unwrap() / 2;
    cheapest_crab_sub_alignment(input, |x, y| triangular_sequence(abs_diff(x, y)).into())
}

#[test]
fn part2_example() {
    let (cheapest_fuel_consumption, cheapest_indices) = cheapest_crab_sub_alignment_part2(EXAMPLE);
    assert_eq!(cheapest_fuel_consumption, 168);
    assert_eq!(cheapest_indices, [5])
}

#[test]
fn part2() {
    let (cheapest_fuel_consumption, cheapest_indices) = cheapest_crab_sub_alignment_part2(INPUT);
    assert_eq!(cheapest_fuel_consumption, 104149091);
    assert_eq!(cheapest_indices, [500])
}
