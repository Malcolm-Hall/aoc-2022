#![feature(iter_array_chunks)]

use std::{
    fs::read_to_string,
    iter::once,
    ops::{Div, Rem},
};

fn main() {
    let input = read_to_string("inputs/d11.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation_value: Option<u64>,
    operator: fn(u64, u64) -> Option<u64>,
    test_value: u64,
    true_case_idx: usize,
    false_case_idx: usize,
    inpect_count: u64,
}

impl Monkey {
    fn from_chunk(chunk: [&str; 7]) -> Self {
        let [_, items, operation, test, true_case, false_case, _] = chunk;

        let items: Vec<u64> = items
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        let mut op_iter = operation.split_once("= ").unwrap().1.split(" ").skip(1);

        let operator = match op_iter.next().unwrap() {
            "+" => u64::checked_add,
            "*" => u64::checked_mul,
            _ => panic!("Unexpected operator"),
        };

        let operation_value = match op_iter.next().unwrap() {
            "old" => None,
            val => val.parse::<u64>().ok(),
        };

        let test_value = test.split(' ').last().unwrap().parse::<u64>().unwrap();

        let true_case_idx = true_case
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let false_case_idx = false_case
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Self {
            items,
            operation_value,
            operator,
            test_value,
            true_case_idx,
            false_case_idx,
            inpect_count: 0,
        }
    }

    fn apply_operation(&self, worry_level: u64) -> u64 {
        match self.operation_value {
            None => (self.operator)(worry_level, worry_level),
            Some(value) => (self.operator)(worry_level, value),
        }
        .unwrap()
    }
}

fn part_1(input: &str) -> u64 {
    part_impl(input, |mut monkeys: Vec<Monkey>| {
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let mut next_monkeys = monkeys.clone();
                let mut monkey = &mut monkeys[i];
                for &worry_level in &monkey.items {
                    monkey.inpect_count += 1;

                    let new_worry_level = monkey.apply_operation(worry_level).div(3);

                    if new_worry_level % monkey.test_value == 0 {
                        next_monkeys[monkey.true_case_idx]
                            .items
                            .push(new_worry_level)
                    } else {
                        next_monkeys[monkey.false_case_idx]
                            .items
                            .push(new_worry_level)
                    }
                }
                let mut monkey = monkey.clone();
                monkey.items.clear();
                next_monkeys[i] = monkey;
                monkeys = next_monkeys;
            }
        }
        monkeys
    })
}

fn part_2(input: &str) -> u64 {
    part_impl(input, |mut monkeys: Vec<Monkey>| {
        let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.test_value).product();

        for _ in 0..10_000 {
            for i in 0..monkeys.len() {
                let mut next_monkeys = monkeys.clone();
                let mut monkey = &mut monkeys[i];
                for &worry_level in &monkey.items {
                    monkey.inpect_count += 1;

                    let new_worry_level = monkey.apply_operation(worry_level).rem(common_multiple);

                    if new_worry_level % monkey.test_value == 0 {
                        next_monkeys[monkey.true_case_idx]
                            .items
                            .push(new_worry_level)
                    } else {
                        next_monkeys[monkey.false_case_idx]
                            .items
                            .push(new_worry_level)
                    }
                }
                let mut monkey = monkey.clone();
                monkey.items.clear();
                next_monkeys[i] = monkey;
                monkeys = next_monkeys;
            }
        }

        monkeys
    })
}

fn part_impl(input: &str, monkey_business: fn(Vec<Monkey>) -> Vec<Monkey>) -> u64 {
    let initial_monkeys: Vec<Monkey> = input
        .lines()
        .chain(once(""))
        .array_chunks::<7>()
        .map(|chunk| Monkey::from_chunk(chunk))
        .collect();

    let final_monkeys = monkey_business(initial_monkeys);

    let mut inspect_counts: Vec<u64> = final_monkeys.into_iter().map(|m| m.inpect_count).collect();

    inspect_counts.sort();

    inspect_counts
        .into_iter()
        .rev()
        .take(2)
        .reduce(|l, r| l * r)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 10605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 2713310158);
    }

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
