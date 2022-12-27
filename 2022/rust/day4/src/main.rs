use util::read_lines;

fn main() {
    assert!(process("test") == 2, "Failed testcase");
    process("input");
}

struct ElfRange {
    lo: i32,
    hi: i32,
}

impl ElfRange {
    fn new(range_str: &str) -> Self {
        let mut limits = range_str.split('-');
        Self {
            lo: limits.next().unwrap().parse::<i32>().unwrap(),
            hi: limits.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

fn process(filename: &str) -> i32 {
    let mut redundancies = 0;
    let lines = read_lines(filename).unwrap();
    for line in lines {
        let line = line.unwrap();
        let mut elf_ranges = line.split(',');
        let r1 = ElfRange::new(elf_ranges.next().unwrap());
        let r2 = ElfRange::new(elf_ranges.next().unwrap());

        let r2_in_r1 = r2.lo >= r1.lo && r2.hi <= r1.hi;
        let r1_in_r2 = r1.lo >= r2.lo && r1.hi <= r2.hi;
        if r2_in_r1 || r1_in_r2 {
            redundancies += 1;
        }
    }

    println!("{} {}", filename, redundancies);
    return redundancies;
}
