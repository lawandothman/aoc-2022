use std::{cmp, collections::HashSet};

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, usize) {
    let mut blocked = HashSet::new();
    let mut floor = 0;

    for line in input.lines() {
        line.split(" -> ")
            .fold(None, |acc: Option<(usize, usize)>, str| {
                let (x, y) = str
                    .split_once(',')
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap();
                floor = cmp::max(floor, y);

                if let Some(last) = acc {
                    if x == last.0 {
                        for i in cmp::min(y, last.1)..=cmp::max(y, last.1) {
                            blocked.insert((x, i));
                        }
                    } else if y == last.1 {
                        for i in cmp::min(x, last.0)..=cmp::max(x, last.0) {
                            blocked.insert((i, y));
                        }
                    }
                } else {
                    blocked.insert((x, y));
                }
                Some((x, y))
            });
    }

    (blocked, floor)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut blocked, floor) = parse_input(input);
    let count = blocked.len();
    let mut q = Vec::new();
    q.push((500, 0));

    while let Some(&(x, y)) = q.last() {
        let mut found = false;
        if y <= floor {
            for next in [(x + 1, y + 1), (x - 1, y + 1), (x, y + 1)]
                .into_iter()
                .filter(|next| !blocked.contains(next))
            {
                q.push(next);
                found = true;
            }
        }
        if !found {
            blocked.insert((x, y));
            q.pop();
        } else if y >= floor {
            break;
        }
    }
    Some(blocked.len() - count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut blocked, floor) = parse_input(input);
    let count = blocked.len();
    let mut q = Vec::new();
    q.push((500, 0));

    while let Some(&(x, y)) = q.last() {
        let mut found = false;
        if y <= floor {
            for next in [(x + 1, y + 1), (x - 1, y + 1), (x, y + 1)]
                .into_iter()
                .filter(|next| !blocked.contains(next))
            {
                q.push(next);
                found = true;
            }
        }
        if !found {
            blocked.insert((x, y));
            q.pop();
        }
    }
    Some(blocked.len() - count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
