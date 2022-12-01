pub fn part_one(input: &str) -> Option<u32> {
    let parsed_calories: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|elf_rations| {
            elf_rations
                .split('\n')
                .map(|rations| rations.parse().expect("Failed to parse calorie"))
                .collect()
        })
        .collect();

    Some(
        parsed_calories
            .iter()
            .map(|elf_rations| elf_rations.iter().sum())
            .max()
            .expect("Input was empty"),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
