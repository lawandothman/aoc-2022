use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|sack| sack.chars())
            .map(|sack| sack.map(get_priority).collect_vec())
            .map(|x| compartmentalize(x.split_at(x.len() / 2)))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn compartmentalize((a, b): (&[u32], &[u32])) -> u32 {
    *a.into_iter().find(|x| b.contains(x)).unwrap()
}

fn get_priority(item_type: char) -> u32 {
    if item_type.is_uppercase() {
        item_type as u32 - 38
    } else {
        item_type as u32 - 96
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
