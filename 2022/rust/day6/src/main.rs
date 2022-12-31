use itertools::Itertools;
use util::read_lines;

fn main() {
    assert!(process("test1", 4) == 7, "Failed testcase");
    assert!(process("test1", 14) == 19, "Failed testcase");

    assert!(process("test2", 4) == 5, "Failed testcase");
    assert!(process("test3", 4) == 6, "Failed testcase");
    assert!(process("test4", 4) == 10, "Failed testcase");
    assert!(process("test5", 4) == 11, "Failed testcase");
    process("input", 4);
    process("input", 14);
}

fn process(filename: &str, header_len: usize) -> usize {
    let line = read_lines(filename).unwrap().next().unwrap().unwrap();

    let required = line
        .chars()
        .collect_vec()
        .windows(header_len)
        .position(|w| w.iter().all_unique())
        .map(|i| i + header_len)
        .unwrap();

    println!("{} {}", filename, required);
    return required;
}
