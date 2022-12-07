use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

type Directories = HashMap<u64, Vec<u64>>;
type Files = HashMap<u64, Vec<u64>>;

fn parse_input(input: &str) -> (u64, Directories, Files) {
    let mut directories: Directories = HashMap::new();
    let mut files: Files = HashMap::new();
    let mut stack = vec!["/"];

    let mut context = 0;
    for line in input.lines() {
        let mut command = line.split_whitespace();
        match command.next().unwrap() {
            "$" => match command.next().unwrap() {
                "cd" => match command.next().unwrap() {
                    "/" => stack.truncate(1),
                    ".." => {
                        stack.pop();
                    }
                    dir => stack.push(dir),
                },
                "ls" => {
                    let mut hasher = DefaultHasher::new();
                    stack.hash(&mut hasher);
                    context = hasher.finish()
                }
                _ => unreachable!(),
            },
            item => match item {
                "dir" => {
                    let dir = command.next().unwrap();
                    let mut hasher = DefaultHasher::new();
                    let mut new = stack.clone();
                    new.push(dir);
                    new.hash(&mut hasher);
                    let hash = hasher.finish();
                    directories.entry(context).or_default().push(hash);
                    directories.insert(hash, Vec::new());
                }
                size => files
                    .entry(context)
                    .or_default()
                    .push(size.parse().unwrap()),
            },
        }
    }
    let mut hasher = DefaultHasher::new();
    vec!["/"].hash(&mut hasher);
    let root = hasher.finish();
    (root, directories, files)
}

fn calculate_size(context: u64, directories: &Directories, files: &Files) -> u64 {
    files.get(&context).map_or(0, |v| v.iter().sum::<u64>())
        + directories
            .get(&context)
            .unwrap()
            .iter()
            .map(|x| calculate_size(*x, directories, files))
            .sum::<u64>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, directories, files) = parse_input(input);
    Some(
        directories
            .keys()
            .filter_map(|x| {
                let size = calculate_size(*x, &directories, &files);
                if size <= 100000 {
                    Some(size)
                } else {
                    None
                }
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (root, directories, files) = parse_input(input);
    let remaining_space = 30000000 - (70000000 - calculate_size(root, &directories, &files));

    Some(
        directories
            .keys()
            .filter_map(|x| {
                let size = calculate_size(*x, &directories, &files);
                if size >= remaining_space {
                    Some(size)
                } else {
                    None
                }
            })
            .min()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
