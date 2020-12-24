struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut v1: Vec<&u8> = vec![];
        let mut v2: Vec<&u8> = vec![];
        let mut result = true;

        for ch in word1.iter() {
            v1.append(&mut ch.as_bytes().iter().collect::<Vec<&u8>>());
        }

        for ch in word2.iter() {
            v2.append(&mut ch.as_bytes().iter().collect::<Vec<&u8>>());
        }

        if v1.len() != v2.len() {
            result = false;
        } else {
            for (w1, w2) in  {
                if w1 != w2 {
                    result = false;
                    break;
                } 
            }
        }
        result
    }
}

//内存使用 less than 9.38%