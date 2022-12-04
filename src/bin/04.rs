pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let assignments = line
                    .split(',')
                    .map(|assignment| {
                        assignment
                            .split('-')
                            .map(|n| n.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<Vec<u32>>>();

                let x1 = assignments[0][0];
                let x2 = assignments[0][1];

                let y1 = assignments[1][0];
                let y2 = assignments[1][1];

                (x1 >= y1 && x2 <= y2) || (y1 >= x1 && y2 <= x2)
            })
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let assignments = line
                    .split(',')
                    .map(|assignment| {
                        assignment
                            .split('-')
                            .map(|n| n.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<Vec<u32>>>();

                let assignment_1 = assignments[0][0]..=assignments[0][1];
                let assignment_2 = assignments[1][0]..=assignments[1][1];

                assignment_1.into_iter().any(|x| assignment_2.contains(&x))
            })
            .count()
            .try_into()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
