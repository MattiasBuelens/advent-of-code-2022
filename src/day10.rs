use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    AddX(i32),
    Nop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(value) = s.strip_prefix("addx ") {
            Ok(Instruction::AddX(value.parse().unwrap()))
        } else if s == "noop" {
            Ok(Instruction::Nop)
        } else {
            panic!("invalid instruction: {}", s);
        }
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Cpu {
    program: Vec<Instruction>,
    pc: usize,
    current_instruction: Option<Instruction>,
    remaining_cycles: usize,
    x: i32,
}

impl Cpu {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            current_instruction: None,
            remaining_cycles: 0,
            x: 1,
        }
    }

    fn step(&mut self) -> i32 {
        if self.current_instruction.is_none() {
            self.current_instruction = self.program.get(self.pc).cloned();
            self.remaining_cycles = match self.current_instruction {
                Some(Instruction::AddX(_)) => 2,
                Some(Instruction::Nop) => 1,
                None => {
                    // end of file
                    return self.x;
                }
            };
        };
        let x = self.x;
        self.remaining_cycles -= 1;
        if self.remaining_cycles == 0 {
            match self.current_instruction {
                Some(Instruction::Nop) => {
                    // do nothing
                }
                Some(Instruction::AddX(value)) => {
                    self.x += value;
                }
                None => {
                    // end of file???
                }
            }
            self.current_instruction = None;
            self.pc += 1;
        }
        x
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut cpu = Cpu::new(input.to_vec());
    let mut sum = 0;
    for cycle in 1..=220 {
        let x = cpu.step();
        if cycle % 40 == 20 {
            sum += cycle * x;
        }
    }
    sum
}

#[aoc(day10, part2)]
pub fn part2(input: &[Instruction]) -> String {
    let mut screen = [[' '; 40]; 6];
    let mut cpu = Cpu::new(input.to_vec());
    for row in screen.iter_mut() {
        for (x, c) in row.iter_mut().enumerate() {
            let value = cpu.step();
            *c = if value >= (x as i32) - 1 && value <= (x as i32) + 1 {
                '#'
            } else {
                '.'
            };
        }
    }
    screen
        .into_iter()
        .map(|row| {
            let mut row = row.into_iter().collect::<String>();
            row.push('\n');
            row
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = include_str!("../examples/2022/day10.txt").trim();
    }

    #[test]
    fn test_part1_small() {
        let input = input_generator(
            r"
noop
addx 3
addx -5"
                .trim(),
        );
        let mut cpu = Cpu::new(input);
        cpu.step();
        assert_eq!(cpu.remaining_cycles, 0);
        assert_eq!(cpu.x, 1);
        cpu.step();
        assert_eq!(cpu.remaining_cycles, 1);
        assert_eq!(cpu.x, 1);
        cpu.step();
        assert_eq!(cpu.remaining_cycles, 0);
        assert_eq!(cpu.x, 4);
        cpu.step();
        assert_eq!(cpu.remaining_cycles, 1);
        assert_eq!(cpu.x, 4);
        cpu.step();
        assert_eq!(cpu.remaining_cycles, 0);
        assert_eq!(cpu.x, -1);
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 13140);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(
            part2(&input),
            r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            .trim_start()
        );
    }
}
