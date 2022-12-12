use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("inputs/d12.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let (height_map, start, end) = parse_height_map(input);

    let (dist, _) = dijkstra(&height_map, start, end).unwrap();

    *dist.get(&end).unwrap()
}

fn part_2(input: &str) -> u32 {
    let (height_map, _, end) = parse_height_map(input);

    let starting_points = height_map
        .iter()
        .filter(|&(_, &v)| v == 0)
        .map(|(&k, _)| k)
        .collect::<Vec<_>>();

    let mut all_min_dists = Vec::new();
    for start in starting_points {
        let Some((dist, _)) = dijkstra(&height_map, start, end) else {
            continue;
        };

        let min_dist = *dist.get(&end).unwrap();
        all_min_dists.push(min_dist);
    }

    all_min_dists.into_iter().min().unwrap()
}

fn parse_height_map(input: &str) -> (HashMap<(i32, i32), u32>, (i32, i32), (i32, i32)) {
    let mut height_map: HashMap<(i32, i32), u32> = Default::default();
    let mut start = None;
    let mut end = None;
    for (i, line) in input.lines().enumerate() {
        for (j, mut height) in line.char_indices() {
            let position = (i as i32, j as i32);
            if height == 'S' {
                start = Some(position);
                height = 'a';
            } else if height == 'E' {
                end = Some(position);
                height = 'z';
            } else if !height.is_ascii_lowercase() {
                panic!("Unexpected height: {}", height);
            }

            let height = height as u32 - 'a' as u32;
            height_map.insert(position, height);
        }
    }

    (height_map, start.unwrap(), end.unwrap())
}

fn dijkstra(
    height_map: &HashMap<(i32, i32), u32>,
    start: (i32, i32),
    end: (i32, i32),
) -> Option<(
    HashMap<(i32, i32), u32>,
    HashMap<(i32, i32), Option<(i32, i32)>>,
)> {
    let mut dist = HashMap::new();
    let mut prev: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
    let mut queue = HashSet::new();

    for &v in height_map.keys() {
        dist.insert(v, u32::MAX);
        prev.insert(v, None);
        queue.insert(v);
    }

    dist.insert(start, 0);

    loop {
        let u = queue
            .iter()
            .map(|&v| (v, dist.get(&v).unwrap()))
            .filter(|&(_, &dist)| dist < u32::MAX)
            .min_by_key(|&(_, dist)| dist)
            .map(|(v, _)| v);

        if u.is_none() {
            return None;
        }

        let u = u.unwrap();
        queue.remove(&u);

        if u == end {
            return Some((dist, prev));
        }

        let last_height = height_map.get(&u).unwrap();
        const DIRECTION_OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut neighbors = Vec::new();
        for o in DIRECTION_OFFSETS {
            let n = (u.0 as i32 + o.0, u.1 as i32 + o.1);
            if !height_map.contains_key(&n) {
                continue;
            }

            let next_height = height_map.get(&n).unwrap();
            if next_height.saturating_sub(*last_height) > 1 {
                continue;
            }

            if queue.contains(&n) {
                neighbors.push(n);
            }
        }

        for next_position in neighbors {
            let alt = dist.get(&u).unwrap() + 1;
            if &alt < dist.get(&next_position).unwrap() {
                dist.insert(next_position, alt);
                prev.insert(next_position, Some(u));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 31)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 29)
    }
}
