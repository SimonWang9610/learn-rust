// 1
// search should go on both sides
// if sum(left + right) < target, the left should be larger
// if sum(left + right) > target, the right should be smaller
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i: usize = 0;
        let mut j: usize = numbers.len() - 1;

        while numbers[i] + numbers[j] != target {
            if numbers[i] + numbers[j] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }

        vec![i as i32 + 1, j as i32 + 1]
    }
}

// 2
// standard library
impl Solution2 {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &v) in numbers.iter().enumerate() {
            if let Some(idx) = numbers[i + 1..].binary_search(target - v).ok() {
                return vec![i as i32 + 1, idx as i32 + 1];
            }
        }
        vec![]
    }
}
