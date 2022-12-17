use std::{cmp, collections::BTreeSet};

use itertools::{EitherOrBoth, Itertools as _};
use serde::Deserialize;

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum Item {
    Single(u64),
    List(Vec<Item>),
}

impl Item {
    fn to_list(&self) -> Self {
        match self {
            Item::Single(num) => Self::List(vec![Item::Single(*num)]),
            Item::List(_) => panic!("already a list"),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // If both values are integers, the lower integer should come first. If the left integer
            // is lower than the right integer, the inputs are in the right order. If the left
            // integer is higher than the right integer, the inputs are not in the right order.
            // Otherwise, the inputs are the same integer; continue checking the next part of the
            // input.
            (Self::Single(l), Self::Single(r)) => l.partial_cmp(r),

            // If both values are lists,...
            (Self::List(l), Self::List(r)) => {
                // ... compare the first value of each list, then the second value, and so on.
                for zip in l.iter().zip_longest(r.iter()) {
                    match zip {
                        // the task doesn't really say so, but an ordering decision is made as soon
                        // as one of the comparisons is not equal
                        EitherOrBoth::Both(l, r) => match l.partial_cmp(r)? {
                            cmp::Ordering::Less => return Some(cmp::Ordering::Less),
                            cmp::Ordering::Equal => {}
                            cmp::Ordering::Greater => return Some(cmp::Ordering::Greater),
                        },

                        // If the right list runs out of items first, the inputs are not in the
                        // right order.
                        EitherOrBoth::Left(_) => return Some(cmp::Ordering::Greater),

                        // If the left list runs out of items first, the inputs are in the right
                        // order.
                        EitherOrBoth::Right(_) => return Some(cmp::Ordering::Less),
                    }
                }

                Some(cmp::Ordering::Equal)
            }
            (l @ Self::Single(..), r @ Self::List(..)) => l.to_list().partial_cmp(r),
            (l @ Self::List(..), r @ Self::Single(..)) => l.partial_cmp(&r.to_list()),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Returns true if packet look like a divider packet with `n`.
///
/// E.g., `[[2]]`
fn is_divider_packet(packet: &[Item], n: u64) -> bool {
    if packet.len() != 1 {
        return false;
    }

    match packet.first() {
        Some(Item::List(list)) if list.len() == 1 => match list.first() {
            Some(Item::Single(num)) => *num == n,
            _ => false,
        },
        _ => false,
    }
}

fn main() {
    let input = match std::env::args().nth(1) {
        Some(flag) if flag == "--test" => INPUT_TEST,
        _ => INPUT,
    };

    // problem A

    let sum = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_list)
        .chunks(2)
        .into_iter()
        .map(|iter| iter.collect_tuple().unwrap())
        .enumerate()
        .filter_map(|(i, (a, b))| match a.cmp(&b) {
            cmp::Ordering::Less | cmp::Ordering::Equal => Some(i + 1),
            cmp::Ordering::Greater => None,
        })
        .sum::<usize>();

    let solution_a = sum;

    // problem B

    let packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .chain(["[[2]]", "[[6]]"].into_iter())
        // lines look like JSON arrays so parsing is simple
        .map(|line| serde_json::from_str::<Vec<Item>>(line).unwrap())
        .collect::<BTreeSet<_>>();

    let mut two = 0;
    let mut six = 0;

    // items are iterated in order for BTreeSet
    for (i, packet) in packets.into_iter().enumerate() {
        if is_divider_packet(&packet, 2) {
            two = i + 1;
        }

        if is_divider_packet(&packet, 6) {
            six = i + 1;
            // break early since we know the [[6]] is after the [[2]]
            break;
        }
    }

    let solution_b = two * six;

    println!("solution A = {solution_a}");
    println!("solution B = {solution_b}");
}

/// Specialized parsing method, improves performance over serde_json.
#[allow(dead_code)]
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
                    Err(_) => items.push(Item::List(parse_list(&part))),
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
        Err(_) => items.push(Item::List(parse_list(&part))),
    };

    items
}
