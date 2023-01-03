use std::cmp::Ordering;
use std::str::FromStr;

enum Operation {
    Add(i64),
    Mult(i64),
    Square,
}

impl Operation {
    fn apply(self: &Self, v: i64) -> i64 {
        match self {
            Operation::Add(rhs) => v + rhs,
            Operation::Mult(rhs) => v * rhs,
            Operation::Square => v * v,
        }
    }
}

impl FromStr for Operation {
    type Err = core::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, rhs) = s
            .split("new = old ")
            .skip(1)
            .next()
            .unwrap()
            .split_once(' ')
            .unwrap();
        if op == "+" {
            return Ok(Operation::Add(rhs.parse::<i64>().unwrap()));
        } else if rhs == "old" {
            return Ok(Operation::Square);
        } else {
            return Ok(Operation::Mult(rhs.parse::<i64>().unwrap()));
        }
    }
}

struct Test {
    divisor: i64,
    success_monkey: usize,
    fail_monkey: usize,
}

impl Test {
    fn test(self: &Self, item: i64) -> usize {
        if item.rem_euclid(self.divisor) == 0 {
            return self.success_monkey;
        } else {
            return self.fail_monkey;
        }
    }
}

impl FromStr for Test {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        let divisor = lines[0]
            .split("Test: divisible by ")
            .skip(1)
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let success_monkey = lines[1]
            .split("If true: throw to monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let fail_monkey = lines[2]
            .split("If false: throw to monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        return Ok(Test {
            divisor,
            success_monkey,
            fail_monkey,
        });
    }
}

struct Monkey {
    op: Operation,
    items: Vec<i64>,
    test: Test,
    inspected: i64,
}

impl Monkey {
    fn get_actions(self: &mut Self, worry_div: i64, common_div: i64) -> Vec<(i64, usize)> {
        let mut actions: Vec<(i64, usize)> = Vec::new();
        for item in self.items.iter() {
            self.inspected += 1;

            // Mod out the common divisor, as it doesn't impact passing and keeps values lower
            let new_val = (self.op.apply(*item) / worry_div) % common_div;
            let dst_monkey = self.test.test(new_val);
            actions.push((new_val, dst_monkey));
        }

        // Consume the items, then pass then out
        self.items = Vec::new();

        actions
    }
}

impl Ord for Monkey {
    fn cmp(&self, other: &Monkey) -> Ordering {
        self.inspected.cmp(&other.inspected)
    }
}

impl Eq for Monkey {}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Monkey) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Monkey) -> bool {
        self.inspected == other.inspected
    }
}

impl FromStr for Monkey {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        // get items
        let items = lines[1]
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        // get operation
        let op = lines[2].parse::<Operation>()?;
        let test = lines[3..].join("\n").parse::<Test>()?;
        let inspected = 0;

        return Ok(Monkey {
            op,
            items,
            test,
            inspected,
        });
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|input| input.parse::<Monkey>().unwrap())
        .collect()
}

fn process(mut monkies: Vec<Monkey>, rounds: i32, worry_div: i64) -> i64 {
    let common_div = monkies.iter().fold(1, |acc, e| acc * e.test.divisor);
    for _ in 0..rounds {
        for i in 0..monkies.len() {
            let actions = monkies[i].get_actions(worry_div, common_div);

            for (val, dst) in actions {
                monkies[dst].items.push(val);
            }
        }
    }

    // Sort in descending order
    monkies.sort_by(|a, b| b.cmp(a));
    println!(
        "{:?}",
        monkies.iter().map(|m| m.inspected).collect::<Vec<i64>>()
    );
    return monkies[0].inspected * monkies[1].inspected;
}

fn main() {
    let input = parse_input(include_str!["../input"]);
    //println!("{}", process(input, 20, 3));
    println!("{}", process(input, 10000, 1));
}
