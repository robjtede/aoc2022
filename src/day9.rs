use std::collections::BTreeSet;

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

fn dir_move(dir: &str, pos: &mut (i64, i64)) {
    match dir {
        "R" => pos.0 += 1,
        "U" => pos.1 += 1,
        "L" => pos.0 -= 1,
        "D" => pos.1 -= 1,
        _ => panic!("invalid dir {dir}"),
    }
}

fn distance((hx, hy): (i64, i64), (tx, ty): (i64, i64)) -> f64 {
    (((hx - tx) as f64).abs().powi(2) + ((hy - ty) as f64).abs().powi(2)).sqrt()
}

#[derive(Debug)]
enum MoveType {
    None,
    Linear,
    Diag,
}

fn move_type(h: (i64, i64), t: (i64, i64)) -> MoveType {
    let dist = distance(h, t);

    // values chosen between square roots of 2, 3, and 4

    if dist > 2.01 {
        MoveType::Diag
    } else if dist > 1.5 {
        MoveType::Linear
    } else {
        MoveType::None
    }
}

fn main() {
    let test = std::env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--test");

    let input = if test { INPUT_TEST } else { INPUT };

    // problem A

    let mut h = (0_i64, 0_i64);
    let mut t = (0_i64, 0_i64);

    let mut t_set = Vec::<(i64, i64)>::new();

    for line in input.lines() {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dist = dist.parse::<i64>().unwrap();

        for _ in 0..dist {
            let h_prev = h;

            // move H according to the input
            dir_move(dir, &mut h);

            match move_type(h, t) {
                // H has not moved enough to move T
                MoveType::None => {}

                // H has moved linearly away from T, move it the same
                MoveType::Linear => dir_move(dir, &mut t),

                // H is not in line with T, move T to previous H location
                MoveType::Diag => t = h_prev,
            }

            t_set.push(t);
        }
    }

    let solution_a = BTreeSet::from_iter(t_set).len();
    println!("solution A = {solution_a:?}");

    // problem B

    let solution_b = "TODO";
    println!("solution B = {solution_b}");
}
