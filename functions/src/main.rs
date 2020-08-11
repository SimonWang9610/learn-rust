fn main() {
    let mut count: i32 = 0;

    let result: i32 = loop {
        println!("{} times", count);
        count = count + 1;
        if count == 10 {
            break 10;
        }
    };

    println!("final result is {}", result);
}
