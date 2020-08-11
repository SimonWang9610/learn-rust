use std::collections::HashMap;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut hash = HashMap::new();
        let mut max = 0 as usize;
        let mut special = arr[0];

        for i in arr {
            let count = hash.entry(i).or_insert(0);
            *count += 1;
            if (*count > max) {
                max = *count;
                special = i;
            }
        }

        special
    }
}

impl Solution2 {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut count: usize = 0;
        let length = (arr.len() as f64 * 0.25).round() as usize;

        while count < arr.len() - length {
            let special_vec = vec![arr[count]; length + 1];
            let iter = special_vec
                .iter()
                .zip(arr[count..count + length + 1].iter())
                .filter(|(a, b)| a == b);
            if iter.count() == length + 1 {
                return arr[count];
            } else {
                count += 1;
            }
        }
        arr[count]
    }
}

impl Solution3 {
    pub fn find_special_integer(nums: Vec<i32>) -> i32 {
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
