pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let characters: Vec<char> = x.to_string().chars().collect();

        let mut left_index = 0;
        let mut right_index = characters.len() - 1;

        for _ in characters.iter() {
            if characters[left_index] != characters[right_index] {
                return false;
            }
            if left_index == right_index {
                return true;
            }
            left_index += 1;
            right_index -= 1;
        }

        return true;
    }

    pub fn run() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(-101), false);
    }
}
