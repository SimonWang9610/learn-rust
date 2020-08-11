impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if nums.len() * nums[0].len() == (r * c) as usize {
            let mut flat = nums.into_iter().flatten();
            let mut result = vec![];
            for _ in 0..r {
                let mut row = vec![];
                for _ in 0..c {
                    if let Some(v) = flat.next() {
                        row.push(v);
                    }
                }
                if row.len() == c as usize {
                    result.push(row);
                }
            }
            result
        } else {
            nums
        }
    }
}
