const EXAMPLE: &str = "3,4,3,1,2";

#[derive(Clone, Debug)]
struct ReproductionTrain {
    ready: usize,
    still_baby: usize,
}

impl ReproductionTrain {
    pub fn empty() -> Self {
        Self {
            ready: 0,
            still_baby: 0,
        }
    }
}

fn run_simulation(input: &str, days: usize) -> usize {
    const ADULT_REPRODUCTION_CYCLE_DAYS: usize = 7;

    let initial_pool = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap() as usize)
        .collect::<Vec<_>>();
    let mut num_lanternfish = initial_pool.len();

    let mut reproduction_slots =
        [(); ADULT_REPRODUCTION_CYCLE_DAYS].map(|()| ReproductionTrain::empty());
    initial_pool
        .iter()
        .cloned()
        .for_each(|days_left_to_reproduce| match days_left_to_reproduce {
            adult if adult < reproduction_slots.len() => reproduction_slots[adult].ready += 1,
            still_a_minor => panic!("Yo, {} isn't an adult, get outta here.", still_a_minor),
        });

    (0..days).for_each(|day| {
        let today_slot_idx = day % reproduction_slots.len();
        let ReproductionTrain { ready, still_baby } = &mut reproduction_slots[today_slot_idx];

        let new_babies = *ready;
        num_lanternfish = num_lanternfish.checked_add(new_babies).unwrap();

        *ready = ready.checked_add(*still_baby).unwrap();
        *still_baby = 0;

        let baby_repro_slot = (today_slot_idx + 2) % reproduction_slots.len(); // simulate `day + ADULT_REPRODUCTION_CYCLE_DAYS + 2`
        let still_baby = &mut reproduction_slots[baby_repro_slot].still_baby;
        *still_baby = still_baby.checked_add(new_babies).unwrap();
    });

    num_lanternfish
}

#[test]
fn part1_example() {
    assert_eq!(run_simulation(EXAMPLE, 80), 5934);
}

const INPUT: &str = include_str!("./day6_input.txt");

#[test]
fn part1() {
    assert_eq!(run_simulation(INPUT, 80), 372984);
}

#[test]
fn part2_example() {
    assert_eq!(run_simulation(EXAMPLE, 256), 26984457539);
}

#[test]
fn part2() {
    assert_eq!(run_simulation(INPUT, 256), 1681503251694);
}
