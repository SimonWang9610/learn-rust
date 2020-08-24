// find the first occurrence and the [degree] occurrence of each element
// if the occurrence times < degree, skip the element
// min([degree - 1] - [0] + 1) of each element

// calculate the degree
// HashMap store the index of each element
// compare the index vector of HashMap

use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut hash: HashMap<i32, Vec<usize>> = HashMap::new();
        let degree = find_degree(&nums);
        let mut length = nums.len();
        for (i, n) in nums.iter().enumerate() {
            if let Some(v) = hash.get_mut(&n) {
                v.push(i);
            } else {
                hash.insert(*n, vec![i]);
            }
        }

        for v in hash.values() {
            if v.len() < degree {
                continue;
            } else {
                length = length.min(v[degree - 1] - v[0] + 1);
            }
        }

        length as i32
    }
}

fn find_degree(nums: &Vec<i32>) -> usize {
    let mut hash = HashMap::new();
    for n in nums {
        let count = hash.entry(n).or_insert(0);
        *count += 1;
    }

    match hash.values().max() {
        Some(&v) => v,
        None => nums.len(),
    }
}
