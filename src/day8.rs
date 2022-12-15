use std::collections::BTreeSet;

use nalgebra::DMatrix;

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

    let width = input.split_once('\n').unwrap().0.len();
    let height = input.lines().count();

    println!();
    println!("width={width} height={height}");

    let grid = input
        .lines()
        .flat_map(|row| row.chars())
        .collect::<Vec<_>>();

    let forest = DMatrix::from_row_slice(height, width, &grid);

    let mut best_scenic_score = ((0, 0), 0);

    for (y, row) in forest.row_iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let ss = scenic_score(&forest, x, y);

            if ss > best_scenic_score.1 {
                best_scenic_score = ((x, y), ss);
            }
        }
    }

    let ((x, y), solution_b) = best_scenic_score;
    println!("best scenic score at {x},{y}");
    println!("solution B = {solution_b}");
}

fn scenic_score(forest: &DMatrix<char>, x: usize, y: usize) -> usize {
    let height = *forest.get((y, x)).unwrap();

    let l = forest.row(y);
    let l = l.iter().take(x).rev();
    let l = fold_view2(height, l);

    let r = forest.row(y);
    let r = r.iter().skip(x + 1);
    let r = fold_view2(height, r);

    let d = forest.column(x);
    let d = d.iter().skip(y + 1);
    let d = fold_view2(height, d);

    let u = forest.column(x);
    let u = u.iter().take(y).rev();
    let u = fold_view2(height, u);

    // println!("u={u}, l={l}, r={r}, d={d}");

    r * l * u * d
}

fn fold_view2<'a>(height: char, trees: impl Iterator<Item = &'a char>) -> usize {
    trees
        .fold((0, false), |(l, blocked), tree| {
            if blocked {
                // view already blocked
                (l, true)
            } else if *tree >= height {
                // same height or higher tree, view beyond blocked
                (l + 1, true)
            } else {
                // lower tree, continue looking
                (l + 1, false)
            }
        })
        .0
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
