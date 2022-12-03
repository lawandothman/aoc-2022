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
    Some(
        input
            .lines()
            .map(|sack| sack.chars())
            .map(|sack| sack.map(get_priority).collect_vec())
            .chunks(3)
            .into_iter()
            .map(|group| group.collect_vec())
            .map(|sack| get_badge((&sack[0], &sack[1], &sack[2])))
            .sum::<u32>(),
    )
}

fn compartmentalize((a, b): (&[u32], &[u32])) -> u32 {
    *a.iter().find(|x| b.contains(x)).unwrap()
}

fn get_badge((a, b, c): (&[u32], &[u32], &[u32])) -> u32 {
    *a.iter()
        .find(|x| b.contains(x) && c.contains(x))
        .unwrap()
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
        assert_eq!(part_two(&input), Some(70));
    }
}
