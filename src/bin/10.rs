enum Instruction {
    Noop,
    AddX(isize),
}

struct Cpu {
    cycle: usize,
    instructions: Vec<Instruction>,
    next: usize,
    curr: usize,
    register: isize,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        let next = match instructions[0] {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        };
        Self {
            cycle: 0,
            instructions,
            next,
            curr: 0,
            register: 1,
        }
    }
    fn cycle(&mut self) {
        self.cycle += 1;
        if self.next == self.cycle {
            match self.instructions[self.curr] {
                Instruction::Noop => {
                    self.curr += 1;
                }
                Instruction::AddX(x) => {
                    self.register += x;
                    self.curr += 1;
                }
            }

            if self.has_instructions() {
                match self.instructions[self.curr] {
                    Instruction::Noop => self.next = self.cycle + 1,
                    Instruction::AddX(_) => self.next = self.cycle + 2,
                }
            }
        }
    }

    fn has_instructions(&self) -> bool {
        self.curr < self.instructions.len()
    }

    fn draw_pixel(&self, crt: &mut String) {
        if self.cycle > 1 && self.cycle % 40 == 0 {
            crt.push('\n');
        }

        let curr = self.cycle % 40;

        match (self.register - curr as isize).abs() {
            n if n < 2 => crt.push('#'),
            _ => crt.push('.'),
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| match l {
            "noop" => Instruction::Noop,
            _ => {
                let (_, x) = l.split_once(' ').unwrap();
                Instruction::AddX(x.parse::<isize>().unwrap())
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut sum = 0;
    let mut cpu = Cpu::new(parse_instructions(input));
    while cpu.has_instructions() {
        cpu.cycle();
        match cpu.cycle + 1 {
            20 | 60 | 100 | 140 | 180 | 220 => sum += (cpu.cycle + 1) as isize * cpu.register,
            _ => {}
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cpu = Cpu::new(parse_instructions(input));
    let mut crt = String::new();
    while cpu.has_instructions() {
        cpu.draw_pixel(&mut crt);
        cpu.cycle();
    }
    Some(crt)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let output = String::from(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
        );
        assert_eq!(part_two(&input), Some(output));
    }
}
