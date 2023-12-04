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

fn process(instructions: &Vec<Instruction>) -> (i32, String) {
    let mut sprite_pos = 1;
    let mut acc = 0;
    let mut cycle: i32 = 0;
    let mut display: Vec<char> = Vec::new();

    let screen_width = 40;

    for inst in instructions.iter() {
        for _ in 0..inst.cycles() {
            cycle += 1;
            let scan_pos = (cycle - 1).rem_euclid(screen_width);

            // p1
            if (cycle - screen_width / 2).rem_euclid(screen_width) == 0 {
                acc += cycle * sprite_pos;
            }

            // p2
            if scan_pos == 0 && cycle != 1 {
                display.push('\n');
            }
            display.push(if (scan_pos - sprite_pos).abs() <= 1 {
                '#'
            } else {
                '.'
            });
        }

        match inst {
            Instruction::Addx(val) => sprite_pos += val,
            Instruction::Noop => (),
        }
    }
    display.push('\n');

    (acc, display.into_iter().collect())
}

fn main() {
    let input = parse_input(include_str!["../input"]);

    let (signal_strength_acc, display) = process(&input);
    println!("{}\n{}", signal_strength_acc, display);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = super::parse_input(include_str!["../test"]);
        assert_eq!(
            super::process(&input),
            (
                13140,
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
                .to_string()
            )
        );
    }
}
