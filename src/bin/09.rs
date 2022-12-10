use std::collections::HashSet;

type Point = (i32, i32);

fn head(dir: &str, pos: &mut Point) {
    match dir {
        "U" => pos.1 += 1,
        "D" => pos.1 -= 1,
        "R" => pos.0 += 1,
        "L" => pos.0 -= 1,
        _ => unreachable!(),
    }
}

fn tail(head: Point, pos: &mut Point) {
    let d = (head.0 - pos.0, head.1 - pos.1);
    if d.0.abs() == 2 || d.1.abs() == 2 {
        pos.0 += d.0.signum();
        pos.1 += d.1.signum();
    }
}

fn solve(input: &str, knot_count: usize) -> Option<usize> {
    let mut rope = vec![(0, 0); knot_count];
    let mut visited = HashSet::from([(0, 0)]);

    for line in input.lines() {
        let (direction, count) = line.split_once(' ').unwrap();
        let count = count.parse::<usize>().unwrap();

        for _ in 0..count {
            head(direction, &mut rope[0]);
            for i in 1..rope.len() {
                tail(rope[i - 1], &mut rope[i]);
                visited.insert(*rope.last().unwrap());
            }
        }
    }

    Some(visited.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 10)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
