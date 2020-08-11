// 3
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash = HashMap::new();
        for i in nums.into_iter() {
            if hash.contains_key(&i) {
                return true;
            } else {
                hash.insert(i, 1);
            }
        }
        false
    }
}

// 1
// the duplicate element in the sorted array will be consecutive
impl Solution2 {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let length = nums.len();
        nums.sort();

        nums.dedup();
        if length == nums.len() {
            false
        } else {
            true
        }
    }
}

// 2
// HashSet<T> vs. HashMap<T, U>
// HashSet only inserts the value with different hash value
use std::collections::HashSet;

impl Solution3 {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() > nums.iter().collect::<HashSet<_>>().len()
    }
}
