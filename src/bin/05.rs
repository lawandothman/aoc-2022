use std::collections::VecDeque;

use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<Vec<usize>>) {
    let (crates, moves) = input.split("\n\n").collect_tuple().unwrap();

    let crates = crates
        .lines()
        .flat_map(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, ch)| ch.is_alphanumeric())
        })
        .into_grouping_map()
        .collect::<VecDeque<char>>()
        .into_iter()
        .sorted_by_key(|(idx, _)| *idx)
        .map(|(_, stack)| stack)
        .collect::<Vec<VecDeque<char>>>();

    let moves = moves
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    (crates, moves)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut crates, moves) = parse_input(input);

    for m in moves {
        for _ in 0..m[0] {
            let item = crates[m[1] - 1].pop_front().unwrap();
            crates[m[2] - 1].push_front(item);
        }
    }
    Some(
        crates
            .into_iter()
            .filter_map(|mut stack| stack.pop_front())
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut crates, moves) = parse_input(input);

    let mut tmp = VecDeque::new();

    for m in moves {
        for _ in 0..m[0] {
            let item = crates[m[1] - 1].pop_front().unwrap();
            tmp.push_back(item);
        }
        for _ in 0..m[0] {
            let item = tmp.pop_back().unwrap();
            crates[m[2] - 1].push_front(item);
        }
    }
    Some(
        crates
            .into_iter()
            .filter_map(|mut stack| stack.pop_front())
            .collect::<String>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
