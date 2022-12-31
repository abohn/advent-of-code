use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ParseError;

enum Node {
    Dir(String),
    File(String, i32),
}

impl FromStr for Node {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(' ').unwrap();

        return match lhs {
            "dir" => Ok(Node::Dir(rhs.to_string())),
            _ => Ok(Node::File(rhs.to_string(), lhs.parse::<i32>().unwrap())),
        };
    }
}

fn get_dir_sizes(
    dir: &PathBuf,
    fs: &HashMap<PathBuf, Vec<Node>>,
    sizes: &mut HashMap<PathBuf, i32>,
) -> i32 {
    let size: i32 = fs
        .get(dir)
        .unwrap()
        .iter()
        .map(|n| match n {
            Node::Dir(subdir) => {
                let subdir_path = dir.join(subdir);
                let subdir_size = get_dir_sizes(&subdir_path, fs, sizes);
                sizes.insert(subdir_path, subdir_size);
                subdir_size
            }
            Node::File(_name, filesize) => *filesize,
        })
        .sum();

    size
}

fn process() -> i32 {
    let input = include_str!("../input");

    let mut path = PathBuf::from("/");
    let mut dir_data: HashMap<PathBuf, _> = HashMap::new();

    for data in input.split('$').skip(1) {
        match data
            .trim()
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            ["ls"] => {
                if !dir_data.contains_key(&path) {
                    let entries = data
                        .lines()
                        .skip(1) // Skip the ls line
                        .filter_map(|line| line.parse::<Node>().ok())
                        .collect::<Vec<Node>>();
                    dir_data.entry(path.clone()).or_insert(entries);
                }
            }
            ["cd", "/"] => {
                path = PathBuf::from("/");
            }
            ["cd", ".."] => {
                path.pop();
            }
            ["cd", rel_path] => {
                path.push(rel_path);
            }
            _ => {
                panic! {"parse error"}
            }
        }
    }

    let mut sizes: HashMap<PathBuf, i32> = HashMap::new();
    get_dir_sizes(&PathBuf::from("/"), &dir_data, &mut sizes);

    let part1_total = sizes.values().filter(|&&s| s <= 100000).sum::<i32>();
    println!("{}", part1_total);
    part1_total
}

fn main() {
    process();
}
