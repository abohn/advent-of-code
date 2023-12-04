use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";

    // Open in read-only mode, ignoring errors.
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            panic!("Error opening {}: {}", filename, err);
        }
    };

    let reader = BufReader::new(file);

    let mut elves_cals = Vec::new();
    let mut this_elf_cals = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line == "" {
            elves_cals.push(this_elf_cals);
            this_elf_cals = 0;
            continue;
        } else {
            this_elf_cals += line.parse::<i32>().unwrap();
        }
    }

    elves_cals.sort();
    elves_cals.reverse();
    let top_n_cals: i32 = elves_cals[..3].iter().sum();

    println!("{}", top_n_cals);
}
