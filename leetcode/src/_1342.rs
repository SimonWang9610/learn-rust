struct Solution;

impl Solution {

    pub fn number_of_steps(num: i32) -> i32 {
        
        let mut count = 0;
        let mut result = num;

        while result != 0 {

            if result&1 == 0 || result == 1 {
                count += 1;
                result = result >> 1;
            } else {
                count += 2;
                result = result >> 1;
            }
        }
        count
    }

}

pub fn count(num: i32) -> i32 {

    if num == 1 {
        1
    } else {
        if num&1 == 1 {
            2 + count(num  >> 1)
        } else {
            1 + count(num >> 1)
        }
    }
    
}

// even >> 1 等价于 even / 2
// odd >> 1 等价于 (odd - 1) / 2

// integer << i = integer * (2 ^ i)

// integer&1 如果是odd，则返回1，否则返回0
// &： 相同为都是1 则返回1，否则返回0
// ^: 相同位不相同则返回1，否则返回0，可以用于判断两个数是否相等
