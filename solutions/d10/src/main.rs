use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/d10.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2:\n {}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    let operations = get_operations(input);
    let interested_cycles: Vec<usize> = (0..=5).map(|i| 20 + i * 40).collect();
    let mut signal_strength_sum = 0;
    let mut reg_x = 1;

    for (cycle, op) in operations
        .into_iter()
        .enumerate()
        .map(|(i, op)| (i + 1, op))
    {
        if interested_cycles.contains(&cycle) {
            signal_strength_sum += cycle as i32 * reg_x;
        }

        if let Some(val) = op {
            reg_x += val;
        }
    }

    signal_strength_sum
}

fn part_2(input: &str) -> String {
    let operations = get_operations(input);
    let mut reg_x: i32 = 1;
    let mut crt_output = String::new();
    let mut crt_position: i32 = 0;

    for op in operations {
        if crt_position.abs_diff(reg_x) <= 1 {
            crt_output.push('#');
        } else {
            crt_output.push('.');
        }

        if let Some(val) = op {
            reg_x += val;
        }

        if crt_position >= 39 {
            crt_position = 0;
            crt_output.push('\n');
        } else {
            crt_position += 1;
        }
    }

    crt_output.pop();
    crt_output
}

fn get_operations(input: &str) -> Vec<Option<i32>> {
    let mut operations = Vec::new();
    for l in input.lines() {
        match &l[0..4] {
            "noop" => operations.push(None),
            "addx" => {
                let val = l[5..].parse::<i32>().unwrap();
                operations.push(None);
                operations.push(Some(val));
            }
            _ => panic!("Unexpected instruction"),
        }
    }
    operations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 13140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            &part_2(TEST_INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
