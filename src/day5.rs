static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

fn main() {
    let test = std::env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--test");

    let input = if test { INPUT_TEST } else { INPUT };

    // part A

    let solution_a = "TODO";
    println!("solution A = {solution_a}");

    // part B

    let solution_b = "TODO";
    println!("solution B = {solution_b}");
}
