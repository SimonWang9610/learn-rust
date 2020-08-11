impl Solution {
    pub fn move_zeros(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;

        while i < nums.len() {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
            i += 1;
        }
    }
}

// after meeting the first zero value, j stop growing up until swap(i, j)
// after swap(i, j), nums[j+1] must be 0
// because all values skipped by nums[i] are 0
// otherwise, swap(i, j)
