// sort()
// Boyer-Moore Voting Algorithm
// HashMap
// Divide & Conquer
// self

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let length = nums.len() / 2;
        let mut special: i32 = nums[0];

        for val in nums.iter() {
            let filter = nums.iter().filter(|a| *a == val);
            if filter.count() > length {
                special = *val;
                break;
            }
        }
        special
    }
}

// Boyer-Moore Voting Algorithm
// the most common element always keeps its counter over 0
impl Solution2 {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut special = nums[0];

        for &n in &nums[1..] {
            if count == 0 {
                special = n;
            }

            if n == special {
                count += 1;
            } else {
                count -= 1;
            }
        }

        special
    }
}

// Divide & Conquer

impl Solution3 {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // divide vector length to 1
        if nums.len() == 1 {
            return nums[0];
        }

        let half = nums.len() / 2;

        // find the majority of left and right respectively
        let majority_left = Self::majority_element(nums[..half].to_vec());
        let majority_right = Self::majority_element(nums[half..].to_vec());

        if majority_left == majority_right {
            return majority_left;
        }

        // compare the whole vector ([left..right])
        // determine the majority of the vector between the left and right majorities
        let mut count_left = 0;
        let mut count_right = 0;

        for &n in &nums {
            if n == majority_left {
                count_left += 1;

                if count_left > half {
                    return majority_left;
                }
            }

            if n == majority_right {
                count_right += 1;
                if count_right > half {
                    return majority_right;
                }
            }
        }
        // default return majority_left
        if count_right > count_left {
            majority_right
        } else {
            majority_left
        }
    }
}

impl Solution4 {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // sort() can ony be used to '&mut self'
        let mut nums = nums;
        nums.sort();
        nums[nums.len() / 2]
    }
}

// HashMap
use std::collections::HashMap;

impl Solution5 {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut hash = HashMap::new();
        let mut special = nums[0];

        for &n in &nums {
            let count = hash.entry(n).or_insert(0);
            *count += 1;

            if *count > nums.len() / 2 {
                special = n;
            }
        }
        special
    }
}
