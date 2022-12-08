use std::collections::BTreeSet;

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

fn to_priority(c: char) -> u64 {
    let p = match c {
        c @ 'a'..='z' => (c as u8) - ('a' as u8) + 1,
        c @ 'A'..='Z' => (c as u8) - ('A' as u8) + 27,
        _ => panic!("char {c} out of range"),
    };

    p as u64
}

fn main() {
    let test = std::env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--test");

    let input = if test { INPUT_TEST } else { INPUT };

    // part A

    let common_sum = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| {
            debug_assert_eq!(first.len(), second.len());

            let mut common = BTreeSet::new();

            for c in first.chars() {
                if second.contains(c) {
                    common.insert(c);
                }
            }

            common.into_iter().map(to_priority).sum::<u64>()
        })
        .sum::<u64>();

    let solution_a = common_sum;
    println!("solution A = {solution_a}");

    // part B

    let common_sum = input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|group| {
            group
                .iter()
                .map(|s| BTreeSet::from_iter(s.chars()))
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .unwrap()
                .into_iter()
                .map(to_priority)
                .sum::<u64>()
        })
        .sum::<u64>();

    let solution_b = common_sum;
    println!("solution B = {solution_b}");
}
