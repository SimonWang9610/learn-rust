fn main() {
    println!("Hello, world!");
}

struct Solution(i32);

use std::collections::HashSet;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &v) in numbers.iter().enumerate() {
            if let Some(idx) = numbers[i + 1..].binary_search(&(target - v)).ok() {
                return vec![i as i32 + 1, (i + idx + 2) as i32];
            }
        }
        vec![]
    }
}
