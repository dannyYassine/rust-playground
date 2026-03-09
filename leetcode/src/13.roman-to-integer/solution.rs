use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_number = 0;
        let mut last_roman_number = 0;
        let characters: Vec<char> = s.chars().collect();
        let map: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut right_index: isize = s.len() as isize - 1;

        while right_index >= 0 {
            let current = characters[right_index as usize];
            let value = *map.get(&current).unwrap();

            if value >= last_roman_number {
                roman_number += value;
            } else {
                roman_number -= value;
            }
            last_roman_number = value;

            right_index -= 1;
        }

        roman_number
    }

    pub fn run() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
