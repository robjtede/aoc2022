use std::collections::HashSet;

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));
static INPUT_TEST_LARGE: &str = include_str!(concat!("./", module_path!(), "_test_large.txt"));

/// Returns mutable reference to both `i` and `i+1` positions in `arr`.
fn pair_mut(arr: &mut [(i64, i64)], i: usize) -> (&mut (i64, i64), &mut (i64, i64)) {
    let (h, t) = arr[i..=(i + 1)].split_at_mut(1);
    (&mut h[0], &mut t[0])
}

/// Returns pythagorean distance between `h` and `t`.
fn distance((hx, hy): (i64, i64), (tx, ty): (i64, i64)) -> f64 {
    (((hx - tx) as f64).abs().powi(2) + ((hy - ty) as f64).abs().powi(2)).sqrt()
}

/// Returns 1 if `b` is true, otherwise returns -1.
fn bool_to_int(b: bool) -> i64 {
    b as u8 as i64 * 2 - 1
}

/// Moves `h` in direction `dir`.
fn move_h(dir: &str, (hx, hy): &mut (i64, i64)) {
    match dir {
        "R" => *hx += 1,
        "U" => *hy += 1,
        "L" => *hx -= 1,
        "D" => *hy -= 1,
        _ => panic!("invalid direction {dir}"),
    }
}

/// Moves `t` to follow `h` assuming that `t` actually needs to move.
fn move_t((hx, hy): (i64, i64), (tx, ty): &mut (i64, i64)) {
    let dx = hx - *tx;
    let dy = hy - *ty;

    if dx == 0 {
        // vertical move
        *ty += dy.signum();
    } else if dy == 0 {
        // horizontal move
        *tx += dx.signum();
    } else {
        // diagonal move
        *tx += bool_to_int(hx > *tx);
        *ty += bool_to_int(hy > *ty);
    }
}

/// Parses input line into direction and distance.
fn parse_line(line: &str) -> (&str, i64) {
    let (dir, dist) = line.split_once(' ').unwrap();
    (dir, dist.parse::<i64>().unwrap())
}

/// Simulate a `LEN` segment rope moving according to `input` instructions, returning a list of the
/// tail's positions after each instruction.
fn follow_rope<const LEN: usize>(input: &str) -> HashSet<(i64, i64)> {
    let mut rope = [(0_i64, 0_i64); LEN];
    let mut t_locations = HashSet::with_capacity(4096);

    // each line is an instruction of how H should move
    for line in input.lines() {
        let (dir, dist) = parse_line(line);

        // move H in direction {dir} by {dist} steps
        for _ in 0..dist {
            // move H according to the input
            move_h(dir, &mut rope[0]);

            // move each subsequent rope segment (if necessary)
            for i in 0..(rope.len() - 1) {
                let (h, t) = pair_mut(&mut rope, i);

                // t needs to move if its pythag distance is larger than sqrt(2)
                if distance(*h, *t) > 1.5 {
                    move_t(*h, t);
                }
            }

            // store updated location for rope tail
            t_locations.insert(*rope.last_mut().unwrap());
        }
    }

    t_locations
}

fn main() {
    // let mut print_path = false;

    let input = match std::env::args().skip(1).next() {
        Some(flag) if flag == "--test" => INPUT_TEST,
        Some(flag) if flag == "--test-large" => INPUT_TEST_LARGE,
        // Some(flag) if flag == "--print" => {
        //     print_path = true;
        //     INPUT
        // }
        _ => INPUT,
    };

    // problem A
    let t_locations = follow_rope::<2>(input);
    // println!("T locations of part A:");
    // if print_path { print_t_locations(&t_locations); }
    // let solution_a = BTreeSet::from_iter(t_locations.clone()).len();
    let solution_a = t_locations.len();
    println!("solution A = {solution_a:?}");

    println!();

    // problem B
    let t_locations = follow_rope::<10>(input);
    // println!("T locations of part B:");
    // if print_path { print_t_locations(&t_locations); }
    // let solution_a = BTreeSet::from_iter(t_locations.clone()).len();
    let solution_b = t_locations.len();
    println!("solution B = {solution_b:?}");
}

// visualisation helpers

fn print_t_locations(t_set: &[(i64, i64)]) {
    for y in (-20..20).rev() {
        let mut line = String::new();
        for x in -40..40 {
            line.push(if t_set.contains(&(x, y)) { 'X' } else { '.' });
        }
        println!("{line}");
    }
}

#[allow(dead_code)]
fn print_board(h: (i64, i64), t: (i64, i64)) {
    for y in (-8..8).rev() {
        let mut line = String::new();
        for x in -8..8 {
            if h == (x, y) {
                line.push('H');
            } else if t == (x, y) {
                line.push('T');
            } else {
                line.push('.');
            }
        }
        println!("{line}");
    }

    println!();
}
