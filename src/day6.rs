use std::collections::HashSet;

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

fn distinct_n_until(line: &str, n: usize) -> usize {
    n + line
        .as_bytes()
        .windows(n)
        .take_while(|&window| HashSet::<&u8>::from_iter(window).len() < n)
        .count()
}

fn main() {
    let input = match std::env::args().skip(1).next() {
        Some(flag) if flag == "--test" => INPUT_TEST,
        _ => INPUT,
    };

    for line in input.lines() {
        // problem A

        let solution_a = distinct_n_until(line, 4);
        println!("solution A = {solution_a}");

        // problem B

        let solution_b = distinct_n_until(line, 14);
        println!("solution B = {solution_b}");

        println!();
    }
}
