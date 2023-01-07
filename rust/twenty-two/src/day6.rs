use std::collections::{HashSet, VecDeque};
use crate::core::{Day};

pub struct DaySix {
    input: String
}

impl From<String> for DaySix {
    fn from(input: String) -> Self {
       DaySix { input }
    }
}

impl Day for DaySix {
    type R = u32;

    fn day() -> String where Self: Sized {
        "6".to_string()
    }

    fn part_one(&self) -> Self::R {
        self.find_marker_of_length(4)
    }

    fn part_two(&self) -> Self::R {
        self.find_marker_of_length(14)
    }
}

impl DaySix {
    fn find_marker_of_length(&self, marker_length: usize) -> u32 {
        let mut marker = VecDeque::new();
        for (i, c) in self.input.chars().enumerate() {
            if marker.len() < marker_length {
                marker.push_back(c);
            } else if marker.len() == marker_length {
                let unique_chars: HashSet<&char> = marker.iter().collect();
                if unique_chars.len() == marker_length {
                    return i as u32;
                }
                marker.pop_front();
                marker.push_back(c);
            }
            //println!("marker candidate: {:?}", marker);
        };
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let actual_res = DaySix::from(input.to_string())
            .part_one();
        assert_eq!(7, actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let actual_res = DaySix::from(input.to_string())
            .part_two();
        assert_eq!(19, actual_res);
    }
}
