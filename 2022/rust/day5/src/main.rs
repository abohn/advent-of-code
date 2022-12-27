use util::read_lines;

fn main() {
    assert!(process("test") == "CMZ", "Failed testcase");
    process("input");
}

fn process(filename: &str) -> String {
    let lines = read_lines(filename).unwrap();

    // Step until finding the num stacks
    let mut stack_id_row = 0;
    let mut n_stacks = 0;
    for (i, line) in lines.enumerate() {
        let line_str = line.unwrap();
        if line_str.chars().nth(1).unwrap() == '1' {
            n_stacks = line_str
                .split_whitespace()
                .next_back()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            stack_id_row = i;
            break;
        }
    }

    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stacks];

    let lines = read_lines(filename).unwrap();
    for (i, line) in lines.enumerate() {
        if i < stack_id_row {
            let crate_str = &line.unwrap();

            // Assume well formatted file, build stacks as we go, then reverse later
            let str_len = crate_str.len();
            for j in 0..n_stacks {
                let loc = 4 * j + 1;
                if loc < str_len {
                    let c = crate_str.chars().nth(loc).unwrap();
                    if c != ' ' {
                        stacks[j].push(c);
                    }
                } else {
                    break;
                }
            }
        } else if i == stack_id_row {
            // Fix the stacks, since they were assembled top down.
            for stack in stacks.iter_mut() {
                stack.reverse();
            }
        } else if i == stack_id_row + 1 {
            // skip blank line
        } else {
            // process move
            let line_str = line.unwrap();
            let mut move_cmd = line_str.split_whitespace();

            let num_to_move = move_cmd.nth(1).unwrap().parse::<i32>().unwrap();

            // -1 to convert from 1 to 0 based indexing
            let origin = move_cmd.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let dest = move_cmd.nth(1).unwrap().parse::<usize>().unwrap() - 1;

            for _ in 0..num_to_move {
                let item = stacks[origin].pop().unwrap();
                stacks[dest].push(item);
            }
        }
    }

    let result: String = stacks.iter().map(|v| v.last().unwrap()).collect();

    println!("{} {}", filename, result);
    return result;
}
