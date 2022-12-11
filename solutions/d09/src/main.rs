use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("inputs/d09.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    part_impl::<1>(input)
}

fn part_2(input: &str) -> usize {
    part_impl::<9>(input)
}

fn part_impl<const N: usize>(input: &str) -> usize {
    let mut tail_trail: HashSet<(i32, i32)> = Default::default();
    let mut head = (0, 0);
    let mut tails: [(i32, i32); N] = [(0, 0); N];

    tail_trail.insert(tails.last().unwrap().clone());

    for l in input.lines() {
        let (direction, amount) = l.split_once(' ').unwrap();
        let mut next_tails: [(i32, i32); N] = [(0, 0); N];
        let mut count = amount.parse::<u32>().unwrap();

        while count > 0 {
            head = next_head(head, direction);
            for (i, tail) in tails.iter().enumerate() {
                let current_head = if i == 0 { &head } else { &next_tails[i - 1] };
                next_tails[i] = next_tail(current_head, &tail);
            }
            tails = next_tails;
            tail_trail.insert(tails.last().unwrap().clone());
            count -= 1;
        }
    }

    tail_trail.len()
}

fn next_head(head: (i32, i32), direction: &str) -> (i32, i32) {
    match direction {
        "R" => (head.0 + 1, head.1),
        "L" => (head.0 - 1, head.1),
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        _ => panic!("Unexpected direction"),
    }
}

fn next_tail(next_head: &(i32, i32), old_tail: &(i32, i32)) -> (i32, i32) {
    let dx = next_head.0 - old_tail.0;
    let dy = next_head.1 - old_tail.1;

    if i32::max(dx.abs(), dy.abs()) <= 1 {
        return old_tail.clone();
    }

    (old_tail.0 + dx.signum(), old_tail.1 + dy.signum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 13)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 1)
    }

    #[test]
    fn test_part_2_large() {
        let test_input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(part_2(test_input), 36)
    }
}
