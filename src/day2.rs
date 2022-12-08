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

    // part A

    let score = input
        .lines()
        .map(|line| match line.split_once(' ').unwrap() {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3 + 0,

            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,

            ("C", "X") => 1 + 6,
            ("C", "Y") => 2 + 0,
            ("C", "Z") => 3 + 3,

            _ => panic!("invalid line {line}"),
        })
        .sum::<u64>();

    let solution_a = score;
    println!("solution A = {solution_a}");

    // part B

    let score = input
        .lines()
        .map(|line| match line.split_once(' ').unwrap() {
            ("A", "X") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("C", "X") => 2 + 0,

            ("A", "Y") => 1 + 3,
            ("B", "Y") => 2 + 3,
            ("C", "Y") => 3 + 3,

            ("A", "Z") => 2 + 6,
            ("B", "Z") => 3 + 6,
            ("C", "Z") => 1 + 6,

            _ => panic!("invalid line {line}"),
        })
        .sum::<u64>();

    let solution_b = score;
    println!("solution B = {solution_b}");
}
