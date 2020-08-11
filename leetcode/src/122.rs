// buy at the lowest point
// sell at the the highest point

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut i = 0;

        while i < prices.len() - 1 {
            let mut j = i;

            while j < prices.len() - 1 {
                if increasing(prices[j], prices[j + 1]) {
                    j += 1;
                } else {
                    break;
                }
            }

            profit += prices[j] - prices[i];
            i = j + 1;
        }
        profit
    }
}

pub fn increasing(current: i32, future: i32) -> bool {
    if future - current > 0 {
        true
    } else {
        false
    }
}

// this solution will skip (current, next) if current >= next
impl Solution2 {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .zip(prices[1..].iter())
            .fold(0, |acc, (a, b)| if b > a { acc + b - a } else { acc })
    }
}

// profit should be the sum of positive deltas of adjacent elements
// we should not add any negative delta
// because we should sold before the price decreases
impl Solution3 {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            profit += 0.max(prices[i] - prices[i - 1]);
        }
        profit
    }
}

// peek() get the next ele without consuming the iterator as &&T
impl Solution4 {
    impl Solution3 {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut profit = 0;
            let peek_iter = prices.iter().peekable()
            while let Some(_) = peek_iter.peek() {
                let current = peek_iter.next().unwrap();
                if let Some(&future) = peek_iter.peek() {
                    if future - current > 0 {
                        profit += future - current;
                    }
                }
            }
        }
    }
    
}