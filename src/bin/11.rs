enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspected: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Operation,
        test: usize,
        true_monkey: usize,
        false_monkey: usize,
    ) -> Self {
        Self {
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
            inspected: 0,
        }
    }

    fn next(&self, item: usize) -> usize {
        match self.operation {
            Operation::Square => item * item,
            Operation::Add(x) => item + x,
            Operation::Multiply(x) => item * x,
        }
    }

    fn inspect_1(&self, item: usize) -> (usize, usize) {
        let worry = self.next(item) / 3;
        let next_monkey = if (worry % self.test) == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        };

        (next_monkey, worry)
    }

    fn inspect_2(&self, item: usize, worry_divisor: usize) -> (usize, usize) {
        let worry = self.next(item) % worry_divisor;
        let next_monkey = if (worry % self.test) == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        };

        (next_monkey, worry)
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for round in input.split("\n\n") {
        let mut line = round.split('\n');

        _ = line.next();

        let (_, items) = line.next().unwrap().split_once(':').unwrap();
        let items = items
            .split(',')
            .map(|item| item.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let (_, operation) = line.next().unwrap().split_once(':').unwrap();
        let operation = if operation.contains('+') {
            let (_, amount) = operation.split_once('+').unwrap();
            Operation::Add(amount.trim().parse().unwrap())
        } else {
            let (_, rhs) = operation.split_once('*').unwrap();
            match rhs.trim() {
                "old" => Operation::Square,
                _ => Operation::Multiply(rhs.trim().parse().unwrap()),
            }
        };

        let (_, divisor) = line.next().unwrap().split_once("by ").unwrap();
        let test = divisor.trim().parse::<usize>().unwrap();

        let (_, true_monkey) = line.next().unwrap().split_once(" monkey ").unwrap();
        let true_monkey = true_monkey.trim().parse::<usize>().unwrap();

        let (_, false_monkey) = line.next().unwrap().split_once(" monkey ").unwrap();
        let false_monkey = false_monkey.trim().parse::<usize>().unwrap();

        monkeys.push(Monkey::new(
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
        ))
    }
    monkeys
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse_input(input);
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let mut monkey = &mut monkeys[m];
            monkey.inspected += monkey.items.len();

            let mut in_transit = Vec::new();

            let curr = &mut monkeys[m];
            for item in curr.items.iter() {
                in_transit.push(curr.inspect_1(*item))
            }
            curr.items.clear();

            for (next_monkey, worry) in in_transit.iter() {
                monkeys[*next_monkey].items.push(*worry)
            }
        }
    }

    monkeys.sort_by(|x, y| y.inspected.cmp(&x.inspected));
    Some(monkeys[0].inspected * monkeys[1].inspected)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse_input(input);
    let worry_mod = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test);
    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            let mut monkey = &mut monkeys[m];
            monkey.inspected += monkey.items.len();

            let mut in_transit = Vec::new();

            let curr = &mut monkeys[m];

            for item in curr.items.iter() {
                in_transit.push(curr.inspect_2(*item, worry_mod))
            }
            curr.items.clear();

            for (next_monkey, worry) in in_transit.iter() {
                monkeys[*next_monkey].items.push(*worry)
            }
        }
    }

    monkeys.sort_by(|x, y| y.inspected.cmp(&x.inspected));
    Some(monkeys[0].inspected * monkeys[1].inspected)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
