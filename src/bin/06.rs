pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .as_bytes()
            .windows(4)
            .position(|slice| {
                slice
                    .iter()
                    .enumerate()
                    .all(|(i, x)| !slice[i + 1..].contains(x))
            })
            .unwrap()
            + 4,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
