use std::{fs::read_to_string, iter::once};

fn main() {
    let input = read_to_string("inputs/d01.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let mut max_calories = 0;
    let mut current_calories = 0;
    for l in input.lines().chain(once("")) {
        if l.is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += l.parse::<u32>().unwrap()
        }
    }
    return max_calories;
}

fn part_2(input: &str) -> u32 {
    let mut max_calories = [0, 0, 0];
    let mut current_calories = 0;
    for l in input.lines().chain(once("")) {
        if l.is_empty() {
            if current_calories > max_calories[0] {
                max_calories[0] = current_calories;
                max_calories.sort();
            }
            current_calories = 0;
        } else {
            current_calories += l.parse::<u32>().unwrap()
        }
    }
    return max_calories.into_iter().sum();
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_1() {
        let result = part_1(TEST_INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(TEST_INPUT);
        assert_eq!(result, 45000);
    }
}
