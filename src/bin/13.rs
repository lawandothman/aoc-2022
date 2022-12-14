use std::{cmp::Ordering, str::FromStr};

#[derive(Eq)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..s.len() - 1];
        let mut int = None;
        let mut stack = Vec::new();
        let mut list = Vec::new();

        for c in s.bytes() {
            match c {
                b'0'..=b'9' => {
                    int = Some(match int.take() {
                        Some(val) => val * 10 + (c - b'0'),
                        None => c - b'0',
                    })
                }
                b',' => {
                    if let Some(val) = int.take() {
                        list.push(Packet::Int(val))
                    }
                }
                b'[' => {
                    stack.push((list, int));
                    list = Vec::new();
                    int = None;
                }

                b']' => {
                    if let Some(val) = int.take() {
                        list.push(Packet::Int(val));
                    }
                    let packet = Packet::List(list);
                    (list, int) = stack.pop().unwrap();
                    list.push(packet)
                }

                _ => unreachable!(),
            }
        }
        if let Some(val) = int.take() {
            list.push(Packet::Int(val));
        }
        Ok(Packet::List(list))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
            (Packet::List(x), Packet::List(y)) => x.cmp(y),
            (x @ Packet::Int(_), Packet::List(y)) => std::slice::from_ref(x).cmp(y.as_slice()),
            (Packet::List(x), y @ Packet::Int(_)) => x.as_slice().cmp(std::slice::from_ref(y)),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|x| x.trim())
            .filter(|l| !l.is_empty())
            .map(|x| x.parse::<Packet>().unwrap())
            .collect::<Vec<Packet>>()
            .chunks(2)
            .enumerate()
            .filter_map(
                |(i, chunk)| match chunk[0].cmp(&chunk[1]) == Ordering::Less {
                    true => Some(i + 1),
                    false => None,
                },
            )
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
