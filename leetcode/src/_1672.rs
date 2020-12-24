struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.into_iter().fold(0, |acc, customer| {
            acc.max(customer.iter().sum())
        })
    }
}