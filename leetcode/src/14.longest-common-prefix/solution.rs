use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest = String::new();
        let mut done = false;
        let mut index = 0;

        while !done {
            let mut set: HashSet<String> = HashSet::new();

            for str in strs.iter() {
                if index == str.len() {
                    done = true;
                    break;
                }
                let value = str.chars().nth(index).unwrap();
                set.insert(String::from(value));
            }

            if set.len() > 1 {
                break;
            }

            if !done {
                longest = longest + set.iter().next().unwrap();
                index += 1;
            }
        }

        return longest;
    }

    pub fn run() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ]),
            String::from("fl")
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("abc"),
                String::from("abc"),
                String::from("abc"),
            ]),
            String::from("abc")
        );
    }
}
