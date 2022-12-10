use crate::core::{Day};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

pub struct DaySeven {
    input: String
}

// this implementation should/could be a derive-macro
impl From<String> for DaySeven {
    fn from(input: String) -> Self {
        DaySeven { input }
    }
}

lazy_static! {
    static ref CD_DOWN: Regex = Regex::new(r"(\$ cd )([\w/]+)").unwrap();
    static ref CD_UP: Regex = Regex::new(r"(\$ cd )([.]{2})").unwrap();
    static ref FILE: Regex = Regex::new(r"(\d+)[ \w\.]+").unwrap();
}

impl Day for DaySeven {
    type R = u64;

    fn day() -> String where Self: Sized {
        String::from("7")
    }

    /*
          keep map of dir names to size.
          $ cd {dir_name} => set current dir to dir_name, => push {dir_name}
           on dir stack
          $ ls do nothing, or, rather parse all lines until a $ is encountered
          increment all dirs in stack with the size of each file encountered
          $ cd .. pop dir stack

         when done, filter map on size and sum
    */
    fn part_one(&self) -> Self::R {
        let dir_size = self.get_directory_size();
        return dir_size.values()
            .filter(|size| *size <= &100000)
            .sum();
    }

    fn part_two(&self) -> Self::R {
        let dir_size = self.get_directory_size();
        let total_disk_space = 70000000;
        let minimum_disk_space_required = 30000000;
        let current_unused_disk_space = total_disk_space - match dir_size.get("//") {
            Some(x) => x, None => &0
        };
        let minimum_directory_size = minimum_disk_space_required - current_unused_disk_space;
        match dir_size.values()
            .filter(|dir| *dir >= &minimum_directory_size)
            .min() {
            Some(min) => *min,
            None => 0
        }

    }
}

impl DaySeven {
    fn get_directory_size(&self) -> HashMap<String, u64> {
        let mut dir_size: HashMap<String, u64> = HashMap::new();
        let mut dir_stack = Vec::new();
        for line in self.input.lines() {
            match CD_DOWN.captures_iter(line).next() {
                Some(down) => {
                    dir_stack.push(String::from(&down[2]));
                    continue
                },
                None => ()
            };

            match CD_UP.captures_iter(line).next() {
                Some(_up) => {
                    dir_stack.pop();
                    continue
                },
                None => ()
            };
            match FILE.captures_iter(line).next() {
                Some(file) => {
                    // if dir_stack.last().is_some() {
                    //     match dir_stack.last() {
                    //         Some(s) if s.eq("hmz") => println!("dir stack: {:?}", dir_stack),
                    //         _ => ()
                    //     }
                    // }
                    let current_stack = dir_stack.clone();
                    let mut path: String = String::new();
                    for dir in current_stack.iter() {
                        path.push_str(dir);
                        path.push_str("/");
                        let old_size = match dir_size.get(path.as_str()) {
                            Some(old_size) => old_size,
                            None => &0
                        };
                        let new_size = *old_size + (&file[1].parse::<u64>().unwrap());
                        dir_size.insert(path.to_string(), new_size);
                    };
                },
                None => ()
            };
        };
        dir_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "$ cd /
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
                7214296 k";
        let actual_res = DaySeven::from(String::from(input)).part_one();
        assert_eq!(95437, actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = "$ cd /
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
                7214296 k";
        let actual_res = DaySeven::from(String::from(input)).part_two();
        assert_eq!(24933642, actual_res);
    }
}
