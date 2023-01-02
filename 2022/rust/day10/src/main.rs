use std::str::FromStr;

enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn cycles(self: &Self) -> usize {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

impl FromStr for Instruction {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("addx") {
            let (_, val) = s.split_once(' ').unwrap();
            return Ok(Instruction::Addx(val.parse::<i32>().unwrap()));
        } else if s.starts_with("noop") {
            return Ok(Instruction::Noop);
        } else {
            panic!("bad instruction {s}");
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>()
}

fn process(instructions: &Vec<Instruction>) -> i32 {
    let mut cycle: i32 = 0;
    let mut x = 1;
    let mut acc = 0;

    for inst in instructions.iter() {
        for _ in 0..inst.cycles() {
            cycle += 1;
            if (cycle - 20).rem_euclid(40) == 0 {
                acc += cycle * x;
            }
        }

        match inst {
            Instruction::Addx(val) => x += val,
            Instruction::Noop => (),
        }
    }

    acc
}

fn main() {
    let input = parse_input(include_str!["../input"]);
    println!("{}", process(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = super::parse_input(include_str!["../test"]);
        assert_eq!(super::process(&input), 13140);
    }
}
