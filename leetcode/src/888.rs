impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let sum_a: i32 = a.iter().sum();
        let sum_b: i32 = b.iter().sum();

        for &i in &a {
            let temp = i - (sum_a - sum_b) / 2;
            if b.contains(&temp) {
                return vec![i, temp];
            }
        }
        vec![]
    }
}

// sum(A) -A[i] + B[j] == sum(B) - B[j] + A[i]
// A[i] - B[j] == (sum(A) - sum(B)) / 2
