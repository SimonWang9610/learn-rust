pub fn pig_latin(s: &str) -> String {
    let mut v: Vec<&str> = s.split(' ').collect();
    let mut results: Vec<String> = Vec::new();

    for word in &v {
        results.push(translate(word));
    }
    results.join(" ")
}

fn first_char(s: &str) -> char {
    let mut chars = s.chars();
    match chars.next() {
        None => panic!("Empty String"),
        Some(c) => c,
    }
}

fn is_vowel(word: &str) -> bool {
    let first_char = first_char(word);

    for vowel in ['a', 'e', 'i', 'o', 'u'].iter() {
        if *vowel == first_char {
            return true;
        }
    }
    false
}

fn translate(word: &str) -> String {
    if is_vowel(word) {
        let mut ans = word[..].to_string();
        ans.push_str("-hay");
        ans
    } else {
        let subfix = format!("-{}{}", first_char(word), "ay");
        let mut ans = word[1..].to_string();
        ans.push_str(&subfix);
        ans
    }
}
