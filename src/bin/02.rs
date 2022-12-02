pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0u32;
    for game in input.lines() {
        let mut choices = game.split_whitespace();

        let opponent = choices.next().unwrap();
        let player = choices.next().unwrap();

        // A - X rock
        // B - Y paper
        // C - Z scissor

        score += match player {
            "X" => {
                1 + match opponent {
                    "A" => 3,
                    "B" => 0,
                    "C" => 6,
                    _ => panic!("Unsupported opponent choice"),
                }
            }
            "Y" => {
                2 + match opponent {
                    "A" => 6,
                    "B" => 3,
                    "C" => 0,
                    _ => panic!("Unsupported opponent choice"),
                }
            }
            "Z" => {
                3 + match opponent {
                    "A" => 0,
                    "B" => 6,
                    "C" => 3,
                    _ => panic!("Unsupported opponent choice"),
                }
            }
            _ => panic!("Unsupported player choice"),
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0u32;
    for game in input.lines() {
        let mut choices = game.split_whitespace();

        let opponent = choices.next().unwrap();
        let player = choices.next().unwrap();

        // A - X rock - loss
        // B - Y paper - draw
        // C - Z scissor - win

        score += match player {
            "X" => {
                0 + match opponent {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => panic!("Unsupported opponent choice"),
                }
            }
            "Y" => {
                3 + match opponent {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => panic!("Unsupported opponent choice"),
                }
            }
            "Z" => {
                6 + match opponent {
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => panic!("Unsupported opponent choice"),
                }
            }
            _ => panic!("Unsupported player choice"),
        }
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
