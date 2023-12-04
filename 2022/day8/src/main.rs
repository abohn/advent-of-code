use std::cmp::max;

type Grid = Vec<Vec<u8>>;

fn scenic_score(grid: &Grid, x: usize, y: usize) -> usize {
    let y_len = grid.len();
    let x_len = grid[0].len();

    // handle edges
    if x == 0 || x == x_len - 1 || y == 0 || y == y_len - 1 {
        return 0;
    }

    let orig_h = grid[y][x];

    // Consider a house at 5, 5 3 8 2 1,
    // take_while().count() = (3).count = 1
    // we can see the 8 too, so add 1. Don't include the edges to avoid
    // double counting.
    return (grid[y][1..x]
        .into_iter()
        .rev()
        .take_while(|h| *h < &orig_h)
        .count()
        + 1)
        * (grid[y][x + 1..x_len - 1]
            .into_iter()
            .take_while(|h| *h < &orig_h)
            .count()
            + 1)
        * (grid[1..y]
            .into_iter()
            .map(|l| l[x])
            .rev()
            .take_while(|h| *h < orig_h)
            .count()
            + 1)
        * (grid[y + 1..y_len - 1]
            .into_iter()
            .map(|l| l[x])
            .take_while(|h| *h < orig_h)
            .count()
            + 1);
}

fn is_visible(grid: &Grid, x: usize, y: usize) -> bool {
    let y_len = grid.len();
    let x_len = grid[0].len();

    // handle edges
    if x == 0 || x == x_len - 1 || y == 0 || y == y_len - 1 {
        return true;
    }

    let orig_h = grid[y][x];

    // Check visibility from each direction
    return grid[y][0..x]
        .into_iter()
        .fold(true, |vis, h| vis && h < &orig_h)
        || grid[y][x + 1..x_len]
            .into_iter()
            .fold(true, |vis, h| vis && h < &orig_h)
        || grid[0..y]
            .into_iter()
            .map(|l| l[x])
            .fold(true, |vis, h| vis && h < orig_h)
        || grid[y + 1..y_len]
            .into_iter()
            .map(|l| l[x])
            .fold(true, |vis, h| vis && h < orig_h);
}

fn parse_input(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("only digits") as u8)
                .collect()
        })
        .collect();
    grid
}

fn process(input: &str) -> (i32, usize) {
    let grid = parse_input(input);

    let mut p1 = 0;
    let mut p2 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            p1 += is_visible(&grid, x, y) as i32;
            p2 = max(p2, scenic_score(&grid, x, y));
        }
    }

    println!("{} {}", p1, p2);
    (p1, p2)
}

fn main() {
    assert!(
        process(include_str!("../test")) == (21, 8),
        "failed testcase"
    );

    process(include_str!("../input"));
}
