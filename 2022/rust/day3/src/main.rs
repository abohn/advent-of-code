use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file(filename: &str) -> File {
    let file: File = match File::open(&filename) {
        Ok(file) => file,
        Err(err) => {
            panic!("Error opening {}: {}", filename, err);
        }
    };

    return file;
}

fn main() {
    assert!(process("test") == 157, "Failed testcase");
    process("input");
}

fn get_index(c: char) -> usize {
    let c_u32 = c as u32;

    let upper_z_u32 = 'Z' as u32;
    let upper_a_u32 = 'A' as u32;
    let lower_a_u32 = 'a' as u32;

    if c_u32 <= upper_z_u32 {
        return (c_u32 - upper_a_u32 + 26) as usize;
    } else {
        return (c_u32 - lower_a_u32) as usize;
    }
}

fn get_dupe_priority(line: &str) -> i32 {
    let divisor = line.len() / 2;
    let first = &line[..divisor];
    let second = &line[divisor..];

    // Build a static bitmap with the first set, break early if
    // the second gets a hit.
    let mut items = vec![false; 52];
    for c in first.chars() {
        items[get_index(c)] = true;
    }

    for c in second.chars() {
        let index = get_index(c);
        if items[index] {
            // Index is 0-51, priorities are 1-52.
            return (index + 1) as i32;
        }
    }
    panic!("didn't find duplicate item!");
}

fn process(input: &str) -> i32 {
    let reader = BufReader::new(get_file(input));

    // Round 1
    // Each rucksack has one dupe. Sum priorities of dupes
    // across rucksacks.
    // a-z -> 1-26
    // A-Z -> 27-52
    //
    // Rucksacks have N chars, with N/2 in each compartment.
    let mut priorities = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        priorities += get_dupe_priority(&line);
    }

    println!("{} {}", input, priorities);
    return priorities;
}
