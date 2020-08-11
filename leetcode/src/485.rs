impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut prev: i32 = 0;
        nums.iter().fold(0, |acc, n| {
            prev = *n;
            if *n == 1 && prev == 1 {
                count += 1;
            } else {
                count = 0;
            }

            if count > acc {
                count
            } else {
                acc
            }
        })
    }
}
