use std::collections::BTreeSet;

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

fn main() {
    let input = match std::env::args().skip(1).next() {
        Some(flag) if flag == "--test" => INPUT_TEST,
        _ => INPUT,
    };

    let mut forest_lr = Vec::with_capacity(99);

    for row in input.lines() {
        let row = row.chars().collect::<Vec<_>>();
        forest_lr.push(row)
    }

    let forest_tb = transpose(forest_lr.clone());

    // print_forest(&forest);

    // problem A

    let mut visible = BTreeSet::new();

    let _sweep_lr = forest_lr.iter().enumerate().for_each(|(y, row)| {
        // space is less than any ASCII digit
        let mut max = ' ';

        for (x, tree) in row.iter().enumerate() {
            if *tree > max {
                visible.insert((x, y));
                max = *tree;
            }
        }
    });

    let _sweep_rl = forest_lr.iter().enumerate().for_each(|(y, row)| {
        // space is less than any ASCII digit
        let mut max = ' ';

        for (x, tree) in row.iter().enumerate().rev() {
            if *tree > max {
                visible.insert((x, y));
                max = *tree;
            }
        }
    });

    let _sweep_tb = forest_tb.iter().enumerate().for_each(|(x, row)| {
        // space is less than any ASCII digit
        let mut max = ' ';

        for (y, tree) in row.iter().enumerate() {
            if *tree > max {
                visible.insert((x, y));
                max = *tree;
            }
        }
    });

    let _sweep_bt = forest_tb.iter().enumerate().for_each(|(x, row)| {
        // space is less than any ASCII digit
        let mut max = ' ';

        for (y, tree) in row.iter().enumerate().rev() {
            if *tree > max {
                visible.insert((x, y));
                max = *tree;
            }
        }
    });

    println!("solution A = {}", visible.len());

    // problem B
    let solution_b = "TODO";
    println!("solution B = {solution_b}");
}

// from https://stackoverflow.com/a/64499219/1743162
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters = v.into_iter().map(|n| n.into_iter()).collect::<Vec<_>>();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
