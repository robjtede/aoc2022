use std::cmp;

use itertools::{EitherOrBoth, Itertools as _};

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Single(u64),
    List(Box<Vec<Item>>),
}

impl Item {
    fn to_list(&self) -> Self {
        match self {
            Item::Single(num) => Self::List(Box::new(vec![Item::Single(*num)])),
            Item::List(_) => panic!("already a list"),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        eprintln!("  comparing {self:?} vs {other:?}");
        match (self, other) {
            (Self::Single(l), Self::Single(r)) => dbg!(l.partial_cmp(r)),
            (Self::List(l), Self::List(r)) => {
                if dbg!(l == r) {
                    return Some(cmp::Ordering::Equal);
                }

                for zip in l.iter().zip_longest(r.iter()) {
                    match dbg!(zip) {
                        // If both values are lists, compare the first value of each list, then the
                        // second value, and so on.
                        EitherOrBoth::Both(l, r) => dbg!(match l.partial_cmp(r)? {
                            cmp::Ordering::Less => return Some(cmp::Ordering::Less),
                            cmp::Ordering::Equal => {}
                            cmp::Ordering::Greater => return Some(cmp::Ordering::Greater),
                        }),

                        // If the right list runs out of items first, the inputs are not in the
                        // right order.
                        EitherOrBoth::Left(_) => return Some(cmp::Ordering::Greater),

                        // If the left list runs out of items first, the inputs are in the right
                        // order.
                        EitherOrBoth::Right(_) => return Some(cmp::Ordering::Less),
                    }
                }

                Some(cmp::Ordering::Less)
            }
            (l @ Self::Single(..), r @ Self::List(..)) => dbg!(l.to_list().partial_cmp(r)),
            (l @ Self::List(..), r @ Self::Single(..)) => dbg!(l.partial_cmp(&r.to_list())),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_list(line: &str) -> Vec<Item> {
    if line.trim().is_empty() {
        return Vec::new();
    }

    let csv = &line[1..line.len() - 1];
    let mut items = Vec::new();
    let mut part = String::new();
    let mut level = 0;

    for c in csv.chars() {
        match c {
            '[' => level += 1,
            ']' => level -= 1,
            ',' if level == 0 => {
                match part.parse::<u64>() {
                    Ok(num) => items.push(Item::Single(num)),
                    Err(_) => items.push(Item::List(Box::new(parse_list(&part)))),
                };

                part.clear();
                continue;
            }
            _ => {}
        }

        part.push(c);
    }

    match part.parse::<u64>() {
        Ok(num) => items.push(Item::Single(num)),
        Err(_) => items.push(Item::List(Box::new(parse_list(&part)))),
    };

    items
}

fn main() {
    let input = match std::env::args().nth(1) {
        Some(flag) if flag == "--test" => INPUT_TEST,
        _ => INPUT,
    };

    let mut sum = 0;

    for (i, (a, b)) in input
        .lines()
        .filter(|line| !line.is_empty())
        .chunks(2)
        .into_iter()
        .map(|iter| iter.collect_tuple().unwrap())
        .enumerate()
    // .skip(1)
    // .take(1)
    {
        let i = i + 1;

        let ap = parse_list(a);
        let bp = parse_list(b);

        let ordered = match ap.cmp(&bp) {
            cmp::Ordering::Less | cmp::Ordering::Equal => true,
            cmp::Ordering::Greater => false,
        };

        println!("pair {i} : {ordered:<5} : {a} <= {b}");

        sum += i * ordered as usize;
    }

    // problem A
    let solution_a = sum;
    println!("solution A = {solution_a}");

    // problem B
    let solution_b = "TODO";
    println!("solution B = {solution_b}");
}
