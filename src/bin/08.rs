#[macro_export]
macro_rules! tree {
    ($x: ident, $y: ident, $dir_value: ident, $dir_max: ident, $seen: ident) => {
        if $dir_value > $dir_max {
            $seen[$y][$x] += 1;
            $dir_max = $dir_value;
        }
    };
}

pub fn part_one(input: &str) -> Option<usize> {
    let trees = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.chars()
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as isize)
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let h = trees.len();
    let w = trees[0].len();
    let mut seen = vec![vec![0isize; w]; h];

    for y in 0..h {
        let mut east_h = -1;
        let mut west_h = -1;
        for x in 0..w {
            let w_idx = w - x - 1;

            let west = trees[y][w_idx];
            let east = trees[y][x];

            tree!(x, y, east, east_h, seen);
            tree!(w_idx, y, west, west_h, seen);
        }
    }
    for x in 0..w {
        let mut north_h = -1;
        let mut south_h = -1;
        for y in 0..h {
            let n_idx = h - y - 1;

            let north = trees[n_idx][x];
            let south = trees[y][x];

            tree!(x, y, south, south_h, seen);
            tree!(x, n_idx, north, north_h, seen);
        }
    }

    Some(
        seen.iter()
            .flat_map(|x| x.iter())
            .filter(|x| **x != 0)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
