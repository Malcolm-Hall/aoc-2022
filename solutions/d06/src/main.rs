use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/d06.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    part_impl(input, 4)
}

fn part_2(input: &str) -> usize {
    part_impl(input, 14)
}

fn part_impl(input: &str, unique_len: usize) -> usize {
    let mut buffer: Vec<char> = Default::default();
    for (idx, c) in input.char_indices() {
        if buffer.contains(&c) {
            buffer = buffer.into_iter().skip_while(|&x| x != c).skip(1).collect();
        }
        buffer.push(c);
        if buffer.len() == unique_len {
            return idx + 1;
        }
    }
    panic!("Could not find start");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn test_part_1() {
        let expected_results = [7, 5, 6, 10, 11];
        TEST_INPUTS
            .into_iter()
            .enumerate()
            .for_each(|(idx, input)| {
                assert_eq!(part_1(input), expected_results[idx]);
            })
    }

    #[test]
    fn test_part_2() {
        let expected_results = [19, 23, 23, 29, 26];
        TEST_INPUTS
            .into_iter()
            .enumerate()
            .for_each(|(idx, input)| {
                assert_eq!(part_2(input), expected_results[idx]);
            })
    }
}
