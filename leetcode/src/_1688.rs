struct Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut matched = n >> 1;
        let mut teams = n - matched;

        while teams != 1 {
            matched += teams >> 1;
            teams = n - matched;
        }

        matched
    }
}

//matched记录了所有已经被匹配的team
//teams = total - matched