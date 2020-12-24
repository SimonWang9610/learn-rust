struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut hash = HashMap::new();

        for n in nums.into_iter() {
            let count = hash.entry(n).or_insert(0);
            *count +=1;
        }

        hash.iter().fold(0, |acc, (_, v)| acc + (v - 1) * v / 2)
    }
}

//题目没有要求输出具体pair的结果
//所以问题简化为两部分：
//  1）计算列表里的重复元素数目 
//  2）根据重复元素数目，计算出每个不同元素能创建的pair

//使用HashMap，只需要遍历一次列表，而不是重复遍历列表