use std::collections::HashSet;

type Loc = Vec<isize>;

fn is_touching(v1: &Loc, v2: &Loc) -> bool {
    return v1[0].abs_diff(v2[0]) <= 1 && v1[1].abs_diff(v2[1]) <= 1;
}

fn propagate_snake(snake: &mut Vec<Loc>) {
    for i in 1..snake.len() {
        if !is_touching(&snake[i - 1], &snake[i]) {
            snake[i] = snake[i - 1]
                .iter()
                .zip(snake[i].iter())
                .map(|(&h, &t)| t + (h - t).min(1).max(-1))
                .collect();
        }
    }
}

fn process(input: &str) -> (usize, usize) {
    let snake_len = 10;
    let mut snake_locs = vec![HashSet::new(); snake_len];

    let mut snake: Vec<Loc> = vec![vec![0; 2]; snake_len];
    for l in input.lines() {
        let cmd = l.split_once(' ').unwrap();
        let dir = cmd.0.chars().nth(0).unwrap();
        let steps = cmd.1.parse::<i32>().unwrap();

        // Move the head
        for _ in 0..steps {
            match dir {
                'U' => {
                    snake[0][1] += 1;
                }
                'D' => {
                    snake[0][1] -= 1;
                }
                'L' => {
                    snake[0][0] -= 1;
                }
                'R' => {
                    snake[0][0] += 1;
                }
                _ => {
                    panic!("Unknown direction {}", dir);
                }
            }

            // If tail isn't touching, move it by 1 step in all directions max.
            propagate_snake(&mut snake);

            // update head, tail heatmaps
            for i in 0..snake_len {
                snake_locs[i].insert(snake[i].clone());
            }
        }
    }

    let p1 = snake_locs[1].len();
    let p2 = snake_locs[snake_len - 1].len();
    println!("{} {}", p1, p2);
    (p1, p2)
}

fn main() {
    assert!(
        process(include_str!("../test")) == (13, 1),
        "failed testcase"
    );

    process(include_str!("../input"));
}
