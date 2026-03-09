use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut index = -1;
        for num in nums.iter() {
            index += 1;
            map.insert(*num, index);
        }

        index = -1;

        for num in nums.iter() {
            index += 1;
            let potential_target = target - num;
            if let Some(potential_target_index) = map.get(&potential_target)
                && *potential_target_index != index
            {
                return vec![index.clone(), potential_target_index.clone()];
            }
        }

        vec![]
    }

    pub fn run() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 4, 11, 3], 6), vec![0, 1]);
    }
}
