// if array is monotonic, the count should be the same as the index
// positive for counting a[i+1] > a[i]
// negative for counting a[i+1] < a[i]
// positive == index || negative == index, the loop continue
// otherwise, return false

impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut positive: i32 = 0;
        let mut negative: i32 = 0;
        let mut temp: i32 = a[0];
        for (index, val) in a.iter().enumerate() {
            if index == 0 {
                continue;
            }

            if increasing(temp, *val) {
                positive += 1;
            }

            if decreasing(temp, *val) {
                negative += 1;
            }

            if positive as usize == index || negative as usize == index {
                temp = *val;
                continue;
            } else {
                return false;
            }
        }
        true
    }
}

pub fn increasing(a: i32, b: i32) -> bool {
    if b - a >= 0 {
        true
    } else {
        false
    }
}

pub fn decreasing(a: i32, b: i32) -> bool {
    if b - a <= 0 {
        true
    } else {
        false
    }
}

//same algorithm but simpler
impl Solution2 {
    let mut increase = true;
    let mut decrease = true;

    for i in 1..a.len() {
        increase &= a[i] >= a[i-1];
        decrease &= a[i] <= a[i-1];
    }

    increase || decrease
}