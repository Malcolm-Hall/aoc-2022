#![feature(is_some_and)]

use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("inputs/d07.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn get_total_dir_sizes(input: &str) -> HashMap<String, u32> {
    let mut dir_stack: Vec<&str> = Default::default();
    let mut dir_sizes: HashMap<String, u32> = Default::default();
    let mut lines = input.lines().peekable();

    while let Some(mut words) = lines.next().map(|l| l.split(' ')) {
        match words.next().unwrap() {
            "$" => match words.next().unwrap() {
                "cd" => match words.next().unwrap() {
                    ".." => {
                        dir_stack.pop();
                    }
                    next_dir => dir_stack.push(next_dir),
                },
                "ls" => {
                    let mut file_size_acc = 0;
                    loop {
                        if lines.peek().is_none()
                            || lines.peek().is_some_and(|next_l| next_l.starts_with("$"))
                        {
                            let current_dir = dir_stack.join("/");
                            dir_sizes.insert(current_dir, file_size_acc);
                            break;
                        }

                        let next_word = lines.next().unwrap().split(' ').next().unwrap();
                        match next_word {
                            "dir" => continue,
                            file_size => file_size_acc += file_size.parse::<u32>().unwrap(),
                        }
                    }
                }
                _ => panic!("Unexpected command"),
            },
            _ => panic!("Unexpected"),
        }
    }

    let mut total_dir_sizes: HashMap<String, u32> = Default::default();
    for dir in dir_sizes.keys() {
        let total_size: u32 = dir_sizes
            .iter()
            .filter(|(k, _)| k.starts_with(dir))
            .map(|(_, v)| v)
            .sum::<u32>();
        total_dir_sizes.insert(dir.clone(), total_size);
    }

    total_dir_sizes
}

fn part_1(input: &str) -> u32 {
    let total_dir_sizes = get_total_dir_sizes(input);

    total_dir_sizes
        .into_values()
        .filter(|v| v <= &100_000)
        .sum()
}

fn part_2(input: &str) -> u32 {
    let total_dir_sizes = get_total_dir_sizes(input);

    let current_size = total_dir_sizes.get("/").unwrap();
    let min_to_delete = current_size - 40_000_000;

    total_dir_sizes
        .into_values()
        .filter(|v| v >= &min_to_delete)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 95437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 24933642);
    }
}
