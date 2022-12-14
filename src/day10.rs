use std::collections::VecDeque;

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));
static INPUT_TEST_LARGE: &str = include_str!(concat!("./", module_path!(), "_test_large.txt"));

fn main() {
    let input = match std::env::args().nth(2) {
        Some(flag) if flag == "--test" => INPUT_TEST,
        Some(flag) if flag == "--test-large" => INPUT_TEST_LARGE,
        _ => INPUT,
    };

    enum State {
        Noop,
        AddX(usize, i64),
    }

    let mut instructions = input
        .lines()
        .map(|line| match &line[..4] {
            "noop" => State::Noop,
            "addx" => State::AddX(1, line[5..].parse().unwrap()),
            inst => panic!("invalid instruction: {inst}"),
        })
        .collect::<VecDeque<_>>();

    let mut x = 1_i64;
    let mut cycle = 0_i64;

    // sum of signal strengths
    let mut ss_sum = 0;

    let mut crt_px = 0_i64;
    let mut crt_out = String::with_capacity(40 * 6);

    loop {
        let Some(instruction) = instructions.pop_front() else { break; };

        cycle += 1;

        // update signal strength on certain cycles
        if (cycle - 20) % 40 == 0 {
            let ss = cycle * x;
            println!("cycle {cycle} signal strength: {ss}");
            ss_sum += ss;
        }

        // update CRT screen
        if (x - (crt_px % 40)).abs() <= 1 {
            crt_out.push('#');
        } else {
            crt_out.push(' ');
        }

        // add line breaks after "scan line" reaches end of CRT
        if cycle % 40 == 0 {
            crt_out.push('\n');
        }

        match instruction {
            State::Noop => {}
            State::AddX(1, n) => instructions.push_front(State::AddX(0, n)),
            State::AddX(0, n) => x += n,
            State::AddX(..) => unreachable!(),
        }

        crt_px += 1;
    }

    // problem A
    println!("solution A = {ss_sum}");

    // problem B
    println!("solution B = ");
    println!("{crt_out}");
}
