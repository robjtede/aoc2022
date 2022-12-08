static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

fn parse_int(num_str: &str) -> u64 {
    num_str
        .parse()
        .unwrap_or_else(|_| panic!("{num_str} is not a string"))
}

fn main() {
    let test = std::env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--test");

    let input = if test { INPUT_TEST } else { INPUT };

    let mut group_sums = input
        .trim()
        .split("\n\n")
        .map(|group| group.split("\n").map(parse_int).sum::<u64>())
        .collect::<Vec<_>>();
    group_sums.sort();

    // part A

    let solution_a = group_sums.last().unwrap();
    println!("solution A = {solution_a}");

    // part B

    let solution_b = group_sums[(group_sums.len() - 3)..].iter().sum::<u64>();
    println!("solution B = {solution_b}");
}
