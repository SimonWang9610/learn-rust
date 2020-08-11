use std::collections::HashMap;

pub fn mean(v: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    for i in v {
        sum += i;
    }
    (sum / v.len() as i32) as f64
}

pub fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    let pos = v.len() as f64 / 2 as f64;
    let pos = (pos.round() - 1.0) as usize;
    v[pos]
}

pub fn mode(v: &Vec<i32>) -> Vec<i32> {
    let mut mode: Vec<i32> = Vec::new();
    let mut max_count: i32 = 0;
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for item in v {
        let count = hash.entry(*item).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
        }
    }

    for (key, val) in &hash {
        if *val == max_count {
            mode.push(*key);
        }
    }
    mode
}
