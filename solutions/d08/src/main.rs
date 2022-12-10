use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/d08.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let grid = get_grid(input);
    let mut visible_count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if is_visible(&grid, height, i, j) {
                visible_count += 1;
            }
        }
    }

    visible_count
}

fn part_2(input: &str) -> usize {
    let grid = get_grid(input);
    let mut max_scenic_score = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            let scenic_score = calculate_scenic_score(&grid, height, i, j);
            max_scenic_score = max_scenic_score.max(scenic_score);
        }
    }

    max_scenic_score
}

fn get_grid(input: &str) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();
    for l in input.lines() {
        let mut row = Vec::new();
        for c in l.chars() {
            let height = c.to_digit(10).unwrap();
            row.push(height);
        }
        grid.push(row);
    }

    grid
}

fn is_visible(grid: &Vec<Vec<u32>>, height: &u32, i: usize, j: usize) -> bool {
    let i_max = grid.len() - 1;
    let j_max = grid[0].len() - 1;

    if i == 0 || i == i_max || j == 0 || j == j_max {
        return true;
    }

    if (0..i).rev().all(|idx| &grid[idx][j] < height) {
        return true;
    }

    if ((i + 1)..=i_max).all(|idx| &grid[idx][j] < height) {
        return true;
    }

    if (0..j).rev().all(|idx| &grid[i][idx] < height) {
        return true;
    }

    if ((j + 1)..=j_max).all(|idx| &grid[i][idx] < height) {
        return true;
    }

    false
}

fn calculate_scenic_score(grid: &Vec<Vec<u32>>, height: &u32, i: usize, j: usize) -> usize {
    let i_max = grid.len() - 1;
    let j_max = grid[0].len() - 1;

    if i == 0 || i == i_max || j == 0 || j == j_max {
        return 0;
    }

    let mut scenic_score = 1;

    let score = (0..=i)
        .rev()
        .enumerate()
        .skip(1)
        .find(|&(_, idx)| &grid[idx][j] >= height || idx == 0)
        .map(|(score, _)| score)
        .unwrap();
    scenic_score *= score;

    let score = (i..=i_max)
        .enumerate()
        .skip(1)
        .find(|&(_, idx)| &grid[idx][j] >= height || idx == i_max)
        .map(|(score, _)| score)
        .unwrap();
    scenic_score *= score;

    let score = (0..=j)
        .rev()
        .enumerate()
        .skip(1)
        .find(|&(_, idx)| &grid[i][idx] >= height || idx == 0)
        .map(|(score, _)| score)
        .unwrap();
    scenic_score *= score;

    let score = (j..=j_max)
        .enumerate()
        .skip(1)
        .find(|&(_, idx)| &grid[i][idx] >= height || idx == j_max)
        .map(|(score, _)| score)
        .unwrap();
    scenic_score *= score;

    scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 21)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 8)
    }
}
