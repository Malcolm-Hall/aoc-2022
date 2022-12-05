#![feature(iter_array_chunks)]

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/d05.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> String {
    part_impl(
        input,
        |count: usize, from_stack: usize, to_stack: usize, stacks: &mut [Vec<char>; 9]| {
            for _ in 0..count {
                let temp = stacks[from_stack].pop().unwrap();
                stacks[to_stack].push(temp);
            }
        },
    )
}

fn part_2(input: &str) -> String {
    part_impl(
        input,
        |count: usize, from_stack: usize, to_stack: usize, stacks: &mut [Vec<char>; 9]| {
            let mut buffer = Vec::new();
            for _ in 0..count {
                buffer.push(stacks[from_stack].pop().unwrap());
            }
            for _ in 0..count {
                stacks[to_stack].push(buffer.pop().unwrap());
            }
        },
    )
}

fn part_impl(input: &str, stack_logic: fn(usize, usize, usize, &mut [Vec<char>; 9])) -> String {
    let (initial_box_state, procedure) = input.split_once("\n\n").unwrap();
    let mut stacks: [Vec<char>; 9] = Default::default();

    initial_box_state.lines().rev().skip(1).for_each(|l| {
        let mut i = 0;
        l.chars().skip(1).step_by(4).for_each(|_crate| {
            if _crate != ' ' {
                stacks[i].push(_crate);
            }
            i += 1;
        });
    });

    for l in procedure.lines() {
        let mut vals = l
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|d| d.parse::<usize>().unwrap());

        let count = vals.next().unwrap();
        let from_stack = vals.next().unwrap() - 1;
        let to_stack = vals.next().unwrap() - 1;

        stack_logic(count, from_stack, to_stack, &mut stacks);
    }

    stacks
        .map(|mut s| s.pop())
        .into_iter()
        .flat_map(|s| s)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), "CMZ");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), "MCD");
    }
}
