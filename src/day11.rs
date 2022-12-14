use std::collections::VecDeque;

use itertools::Itertools;

fn example_case_part_a() {
    let rounds = 20;

    let mut monkeys = [
        VecDeque::from([79, 98]),
        VecDeque::from([54, 65, 75, 74]),
        VecDeque::from([79, 60, 97]),
        VecDeque::from([74]),
    ];

    let ops: Vec<Box<dyn Fn(&mut u64)>> = vec![
        Box::new(|n| *n *= 19),
        Box::new(|n| *n += 6),
        Box::new(|n| *n *= *n),
        Box::new(|n| *n += 3),
    ];

    let tests = [23, 19, 13, 17];
    let targets = [[3, 2], [0, 2], [3, 1], [1, 0]];
    let mut inspections = [0; 4];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for mut worry in monkeys[i].drain(..).collect::<Vec<_>>() {
                // increase monkey's inspections
                inspections[i] += 1;

                // operation
                ops[i](&mut worry);

                // easing
                worry /= 3;

                // test
                let test = worry % tests[i] == 0;

                // move item to target monkey
                let target = targets[i][test as usize];
                monkeys[target].push_back(worry);
            }
        }

        print_monkeys(monkeys.iter());
    }

    println!("{inspections:?}");

    inspections.sort();
    inspections.reverse();
    println!("{}", inspections[..2].iter().product::<u64>());
}

fn real_deal_part_a() {
    let rounds = 20;

    let mut monkeys = [
        VecDeque::from([66, 59, 64, 51]),
        VecDeque::from([67, 61]),
        VecDeque::from([86, 93, 80, 70, 71, 81, 56]),
        VecDeque::from([94]),
        VecDeque::from([71, 92, 64]),
        VecDeque::from([58, 81, 92, 75, 56]),
        VecDeque::from([82, 98, 77, 94, 86, 81]),
        VecDeque::from([54, 95, 70, 93, 88, 93, 63, 50]),
    ];

    let ops: [Box<dyn Fn(&mut u64)>; 8] = [
        Box::new(|n| *n *= 3),
        Box::new(|n| *n *= 19),
        Box::new(|n| *n += 2),
        Box::new(|n| *n *= *n),
        Box::new(|n| *n += 8),
        Box::new(|n| *n += 6),
        Box::new(|n| *n += 7),
        Box::new(|n| *n += 4),
    ];

    let tests = [2, 7, 11, 19, 3, 5, 17, 13];
    let targets = [
        [4, 1],
        [5, 3],
        [0, 4],
        [6, 7],
        [1, 5],
        [6, 3],
        [2, 7],
        [0, 2],
    ];
    let mut inspections = [0; 8];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for mut worry in monkeys[i].drain(..).collect::<Vec<_>>() {
                // increase monkey's inspections
                inspections[i] += 1;

                // operation
                ops[i](&mut worry);

                // easing
                worry /= 3;

                // test
                let test = worry % tests[i] == 0;

                // move item to target monkey
                let target = targets[i][test as usize];
                monkeys[target].push_back(worry);
            }
        }

        print_monkeys(monkeys.iter());
    }

    println!("{inspections:?}");

    inspections.sort();
    inspections.reverse();
    println!("{}", inspections[..2].iter().product::<u64>());
}

fn example_case_part_b() {}

fn real_deal_part_b() {}

fn main() {
    match std::env::args().nth(1) {
        Some(flag) if flag == "--test-a" => example_case_part_a(),
        Some(flag) if flag == "--test-b" => example_case_part_b(),
        Some(flag) if flag == "--real-a" => real_deal_part_a(),
        Some(flag) if flag == "--real-b" => real_deal_part_b(),
        _ => println!("Pass one of the flags: `--test-a`, `--test-b`, `--real-a`, or `--real-b`."),
    }
}

// visualization helpers

fn print_monkeys<'a, I>(monkeys: I)
where
    I: Iterator<Item = &'a VecDeque<u64>>,
{
    // print result
    for (i, items) in monkeys.enumerate() {
        let items =
            Itertools::intersperse(items.into_iter().map(|w| w.to_string()), ", ".to_owned())
                .collect::<String>();
        println!("Monkey {i}: {items}");
    }
    println!();
}
