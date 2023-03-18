use std::cmp::Reverse;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|c| c.parse::<u32>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(c)) = it.next() {
                sum = Some(sum.unwrap_or(0) + c)
            }
            sum
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|c| c.parse::<u32>().ok())
            .batching(|it| it.map_while(|x| x).sum1::<u32>())
            .map(Reverse)
            .k_smallest(3)
            .map(|x| x.0)
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
