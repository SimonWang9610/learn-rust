struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut hash = HashSet::new();

        for v in allowed.as_bytes().iter() {
            hash.insert(v);
        }
        
        words.into_iter().fold(0, |acc, word| {
            
            let mut contain = true;

            for ch in word.as_bytes().iter() {
                if hash.contains(ch) {
                    continue;
                } else {
                    contain = false;
                    break;
                }
            }

            if contain {
                acc + 1
            } else {
                acc
            }
        })
    }
}

//把匹配字符串存储为HashSet，每次对比借用 HashSet.contains()
