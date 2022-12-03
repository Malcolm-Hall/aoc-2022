#![feature(iter_array_chunks)]
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("inputs/d03.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn get_priority(item: char) -> u32 {
    let offset = if item.is_ascii_uppercase() {
        ('A' as u32) - 27
    } else {
        ('a' as u32) - 1
    };

    (item as u32) - offset
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let half = l.len() / 2;
            let left = &l[0..half].chars().collect::<HashSet<_>>();
            let right = &l[half..].chars().collect::<HashSet<_>>();
            let item = left.intersection(&right).next().unwrap();
            get_priority(item.clone())
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|group| {
            let badge_hs = group
                .into_iter()
                .map(|elf| -> HashSet<char> { HashSet::from_iter(elf.chars()) })
                .reduce(|acc, hs| acc.intersection(&hs).cloned().collect())
                .unwrap();

            assert_eq!(badge_hs.len(), 1);

            let badge = badge_hs.iter().next().unwrap();

            get_priority(badge.clone())
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 70);
    }
}
