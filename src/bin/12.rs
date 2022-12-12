use std::mem::swap;

#[derive(Debug)]
struct Map {
    elevations: Vec<Vec<isize>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn char_to_num(c: char) -> isize {
    c as isize - 'a' as isize
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut map = Map {
        elevations: Vec::new(),
        start: (0, 0),
        end: (0, 0),
    };

    for (i, l) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in l.chars().enumerate() {
            if c == 'S' {
                map.start = (i, j);
                row.push(char_to_num('a'));
            } else if c == 'E' {
                map.end = (i, j);
                row.push(char_to_num('z'))
            } else {
                row.push(char_to_num(c))
            }
        }
        map.elevations.push(row);
    }
    let (n, m) = (map.elevations.len(), map.elevations[0].len());
    let mut visited = vec![vec![-1; m]; n];
    let mut positions = vec![map.end];
    let mut next = Vec::new();
    let mut step = -1;

    let neighbours = |i, j| {
        let mut positions = Vec::new();
        if i > 0 {
            positions.push((i - 1, j));
        }
        if j > 0 {
            positions.push((i, j - 1));
        }
        if i < n - 1 {
            positions.push((i + 1, j))
        }
        if j < m - 1 {
            positions.push((i, j + 1))
        }
        positions
    };
    visited[map.end.0][map.end.1] = 0;

    loop {
        step += 1;
        for (i, j) in positions.drain(..) {
            if (i, j) == map.start {
                return Some(step);
            }
            for (ni, nj) in neighbours(i, j) {
                if visited[ni][nj] == -1 && map.elevations[i][j] - map.elevations[ni][nj] <= 1 {
                    visited[ni][nj] = step;
                    next.push((ni, nj));
                }
            }
        }
        swap(&mut next, &mut positions);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
