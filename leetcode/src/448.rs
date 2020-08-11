// similar to Solution2
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for n in 0..nums.len() {
            let idx = (nums[n].abs() - 1) as usize;
            nums[idx] = -nums[idx].abs();
        }
        let mut rtn = vec![];
        for (idx, &n) in nums.iter().enumerate() {
            if n > 0 {
                rtn.push((idx + 1) as i32);
            }
        }
        return rtn;
    }
}

// if the number exists, result[number] will be changed
// even though the number is repeated, result[number] always points to the same location
// if the number never occur, result[number] will not be changed
impl Solution2 {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![true; nums.len()];

        nums.iter().for_each(|x| {
            result[*x as usize - 1] = false;
        });

        result
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if *v { Some(i as i32 + 1) } else { None })
            .collect::<Vec<i32>>()
    }
}

use std::collections::HashSet;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len()
        let mut set = nums.into_iter().collect::<HashSet<i32>>();
        
    }
}