use std::ops::RangeInclusive;

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

fn is_range_total_overlap(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r1.contains(&r2.start()) && r1.contains(&r2.end())
        || r2.contains(&r1.start()) && r2.contains(&r1.end())
}

fn is_range_any_overlap(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r1.contains(&r2.start())
        || r1.contains(&r2.end())
        || r2.contains(&r1.start())
        || r2.contains(&r1.end())
}

fn main() {
    let test = std::env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--test");

    let input = if test { INPUT_TEST } else { INPUT };

    let range_pairs = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(r1, r2)| {
            let (r1s, r1e) = r1.split_once('-').unwrap();
            let (r1s, r1e) = (r1s.parse().unwrap(), r1e.parse().unwrap());

            let (r2s, r2e) = r2.split_once('-').unwrap();
            let (r2s, r2e) = (r2s.parse().unwrap(), r2e.parse().unwrap());

            (r1s..=r1e, r2s..=r2e)
        })
        .collect::<Vec<_>>();

    // part A

    let solution_a = range_pairs
        .iter()
        .filter(|(r1, r2)| is_range_total_overlap(r1, r2))
        .count();
    println!("solution A = {solution_a}");

    // part B

    let solution_b = range_pairs
        .iter()
        .filter(|(r1, r2)| is_range_any_overlap(r1, r2))
        .count();
    println!("solution B = {solution_b}");
}
