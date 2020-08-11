impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        queries
            .iter()
            .map(|q| -> i32 {
                let mut count = 0;
                let count_query = smallest(q);

                for word in words.iter() {
                    if count_query < smallest(word) {
                        count += 1;
                    }
                }
                count
            })
            .collect::<Vec<i32>>()
    }
}

pub fn smallest(word: &str) -> i32 {
    let min = word.chars().min().unwrap();
    word.matches(min).count() as i32
}
