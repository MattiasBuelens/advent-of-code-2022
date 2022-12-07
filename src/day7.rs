use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    CD(String),
    LS,
}

#[derive(Debug)]
pub enum Listing {
    Directory(String),
    File(String, u64),
}

#[derive(Debug, Default)]
pub struct Directory {
    directories: HashMap<String, Directory>,
    files: Vec<(String, u64)>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(dir) = s.strip_prefix("$ cd ") {
            Ok(Command::CD(dir.to_string()))
        } else if s == "$ ls" {
            Ok(Command::LS)
        } else {
            panic!("unknown command: {}", s)
        }
    }
}

#[derive(Debug, Default)]
struct FileSystem {
    current_directory: Vec<String>,
    root: Directory,
}

impl FileSystem {
    fn change_directory(&mut self, path: &str) {
        match path {
            "/" => {
                self.current_directory.clear();
            }
            ".." => {
                self.current_directory
                    .pop()
                    .expect("no parent directory found");
            }
            path => {
                self.current_directory.push(path.to_string());
            }
        }
    }

    fn current_directory_mut(&mut self) -> &mut Directory {
        let mut directory = &mut self.root;
        for part in &self.current_directory {
            directory = directory
                .directories
                .get_mut(part)
                .expect("directory not found");
        }
        directory
    }
}

impl Directory {
    fn total_size(&self) -> u64 {
        self.files.iter().map(|(_, size)| *size).sum::<u64>()
            + self
                .directories
                .iter()
                .map(|(_, dir)| dir.total_size())
                .sum::<u64>()
    }

    fn visit(&self, visitor: &mut impl FnMut(&Directory) -> ()) {
        visitor(&self);
        for (_, dir) in self.directories.iter() {
            dir.visit(visitor);
        }
    }
}

impl FileSystem {
    fn process_commands(&mut self, input: &str) {
        let mut lines = input.lines().into_iter().peekable();
        while let Some(command) = lines.next() {
            match command.parse().unwrap() {
                Command::CD(dir) => {
                    self.change_directory(&dir);
                }
                Command::LS => {
                    let current_directory = self.current_directory_mut();
                    while let Some(line) = lines.next_if(|s| !s.starts_with('$')) {
                        if let Some(dir) = line.strip_prefix("dir ") {
                            current_directory
                                .directories
                                .entry(dir.to_string())
                                .or_default();
                        } else if let Some((size, name)) = line.split_once(' ') {
                            current_directory
                                .files
                                .push((name.to_string(), size.parse().unwrap()));
                        } else {
                            panic!("unknown listing: {}", line);
                        }
                    }
                }
            }
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let mut state = FileSystem::default();
    state.process_commands(input);
    let mut total_size = 0u64;
    state.root.visit(&mut |dir| {
        let dir_size = dir.total_size();
        if dir_size <= 100_000 {
            total_size += dir_size;
        }
    });
    total_size
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
