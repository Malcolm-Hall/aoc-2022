#![feature(iter_next_chunk)]

use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("inputs/d04.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut ranges = l.split(',').map(|assignment| {
                assignment
                    .split('-')
                    .map(|r| r.parse::<u32>().unwrap())
                    .next_chunk::<2>()
                    .unwrap()
            });

            let [[ll, lr], [rl, rr]] = ranges.next_chunk::<2>().unwrap();

            if (ll <= rl && lr >= rr) || (rl <= ll && rr >= lr) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut ranges = l
                .split(',')
                .map(|assignment| {
                    assignment
                        .split('-')
                        .map(|r| r.parse::<u32>().unwrap())
                        .next_chunk::<2>()
                        .unwrap()
                })
                .map(|[lrange, rrange]| (lrange..=rrange));

            let [lelf, relf] = ranges.next_chunk::<2>().unwrap();
            let left: HashSet<u32> = HashSet::from_iter(lelf);
            let right: HashSet<u32> = HashSet::from_iter(relf);
            let mut intersection = left.intersection(&right).into_iter();

            if intersection.next().is_some() {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 4);
    }
}
