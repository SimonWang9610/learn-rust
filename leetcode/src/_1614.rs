
struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let v = s.as_bytes();
        let mut count = 0;

        v.iter().fold(0, |acc, &ele| {
            
            if ele == 40 {
                count += 1;
            } else if ele == 41 {
                count -= 1;
            }

            acc.max(count)
        })
    }
}

//使用count来标记未闭合的 "()”
//每次更新count的时候，就与现有的max_depth比较，返回最新的max_depth

//在列表里搜索，可以搜索条件标记为状态属性，根据搜索结果返回的状态来更新目标值