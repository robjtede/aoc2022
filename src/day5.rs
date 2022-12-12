use std::collections::VecDeque;

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

fn parse_layout(layout: &str) -> Vec<Vec<char>> {
    let mut stacks = vec![VecDeque::with_capacity(8); 9];

    for stack in layout.lines() {
        for (n, boite) in stack.chars().enumerate() {
            if boite != ' ' {
                stacks[n].push_front(boite);
            }
        }
    }

    stacks.into_iter().map(Vec::from).collect()
}

fn parse_movement(line: &str) -> (usize, usize, usize) {
    let mut split = line.splitn(3, ' ');
    let num = split.next().unwrap().parse::<usize>().unwrap();
    let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
    let to = split.next().unwrap().parse::<usize>().unwrap() - 1;
    (num, from, to)
}

fn main() {
    let input = match std::env::args().skip(1).next() {
        Some(flag) if flag == "--test" => INPUT_TEST,
        _ => INPUT,
    };

    let (layout, moves) = input.split_once("\n\n").unwrap();

    // part A
    {
        let mut stacks = parse_layout(layout);

        print_stacks(&stacks);

        for (num, from, to) in moves.lines().map(parse_movement) {
            println!("move {num} from {from} to {to}");

            let len = stacks[from].len();
            let a = stacks[from].drain(len - num..).rev().collect::<Vec<_>>();
            stacks[to].extend(a);

            println!();
            print_stacks(&stacks);
        }

        let solution_a = stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect::<String>();
        println!("solution A = {solution_a}");
    }

    // part B
    {
        let mut stacks = parse_layout(layout);

        print_stacks(&stacks);

        for (num, from, to) in moves.lines().map(parse_movement) {
            println!("move {num} from {from} to {to}");

            let len = stacks[from].len();
            let a = stacks[from].drain(len - num..).collect::<Vec<_>>();
            stacks[to].extend(a);

            println!();
            print_stacks(&stacks);
        }

        let solution_b = stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect::<String>();

        println!("solution B = {solution_b}");
    }
}

// visualization helpers

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for (n, stack) in stacks.iter().enumerate() {
        let mut line = String::new();
        line.push_str(&format!("{n} "));
        for &boite in stack {
            line.push(boite);
        }
        println!("{line}");
    }
    println!()
}
