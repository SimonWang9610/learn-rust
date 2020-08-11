use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        if arr.len() == 0 {
            vec![]
        } else {
            let mut sorted = arr.clone();
            sorted.sort();
            let mut rank = 1;
            let mut current = sorted[0];
            let mut ranks = sorted
                .iter()
                .map(|a| {
                    // rank always grows up by 1 in the sorted array
                    // unless it encounters the same element
                    if current != *a {
                        rank += 1;
                        current = *a;
                    }
                    (*a, rank)
                })
                .collect::<HashMap<i32, i32>>();
            arr.iter()
                .map(|num| *ranks.get(num).unwrap())
                .collect::<Vec<i32>>()
        }
    }
}
