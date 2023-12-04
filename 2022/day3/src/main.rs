use util::read_lines;

fn main() {
    assert!(process("test") == 157, "Failed testcase");
    assert!(process_day2("test") == 70, "Failed testcase day2");
    process_day2("input");
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

fn populate_charmap(string: &str) -> Vec<bool> {
    let mut items = vec![false; 52];
    for c in string.chars() {
        items[get_index(c)] = true;
    }
    return items;
}

fn get_dupe_priority(line: &str) -> i32 {
    let divisor = line.len() / 2;
    let first = &line[..divisor];
    let second = &line[divisor..];

    let items = populate_charmap(first);
    for c in second.chars() {
        let index = get_index(c);
        if items[index] {
            // Index is 0-51, priorities are 1-52.
            return (index + 1) as i32;
        }
    }
    panic!("didn't find duplicate item!");
}

fn get_group_dupe_priority(line1: &str, line2: &str, line3: &str) -> i32 {
    let items1 = populate_charmap(line1);
    let items2 = populate_charmap(line2);
    let items3 = populate_charmap(line3);

    for n in 0..52 {
        if items1[n] && items2[n] && items3[n] {
            return (n + 1) as i32;
        }
    }

    // Index is 0-51, priorities are 1-52.
    panic!("didn't find duplicate item!");
}

fn process(filename: &str) -> i32 {
    // Round 1
    // Each rucksack has one dupe. Sum priorities of dupes
    // across rucksacks.
    // a-z -> 1-26
    // A-Z -> 27-52
    //
    // Rucksacks have N chars, with N/2 in each compartment.
    let mut priorities = 0;
    let lines = read_lines(filename).unwrap();
    for line in lines {
        let line = line.unwrap();
        priorities += get_dupe_priority(&line);
    }

    println!("{} {}", filename, priorities);
    return priorities;
}

fn process_day2(filename: &str) -> i32 {
    let mut lines = read_lines(filename).unwrap();

    // Round 2
    // Each set of 3 rucksacks has one common dupe.
    // Sum priorities of dupes across groups.
    // a-z -> 1-26
    // A-Z -> 27-52
    let mut priorities = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        priorities += get_group_dupe_priority(&line1.unwrap(), &line2.unwrap(), &line3.unwrap());
    }

    println!("{} {}", filename, priorities);
    return priorities;
}
