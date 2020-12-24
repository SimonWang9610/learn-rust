struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut hash = HashMap::new();
        let length = indices.len();
        let v = s.as_bytes();

        for (k, v) in indices.into_iter().zip(v.iter()) {
            hash.insert(k as usize, *v);
        }

        String::from_utf8(
            (0..length).map(|i| {
                *hash.get(&i).unwrap()
            }).collect::<Vec<u8>>()
        ).unwrap()
    }
}

//错误思路：
// 对string按索引进行HashMap后，根据indices的值作为HashMap的key来遍历character

//正确理解：
// HashMap {indices[i] : s[i]}
//然后对HashMap的key按照升序进行取值，组成新的字符串
