use std::any::type_name;
use std::collections::HashMap;

fn main() {
    /* let mut v = vec![1, 2, 3, 4, 5];
    // v.push(6);
    let mut first = &v[0];
    println!("the first is  {}", first);

    let test = vec![
        Multiple::Int(3),
        Multiple::Float(1.5),
        Multiple::Text(String::from("test")),
    ];
    let add: i32 = Multiple::Add(&test[0], 2);
    println!("add is {:#?}", add);

    let mut a = String::from("hello");
    println!("address a {:p}", &a);
    let b = " world".to_string();
    let c = format!("{}-{}", a, b);
    println!("{}, {}, {}", a, b, c); */

    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 20);

    let score = scores.get("Blue");
    println!("{:#?}", score);
}

fn print_type<T>(name: &str, x: &T) -> () {
    println!("type of {} is {}", name, type_of(x));
}

fn remove(x: String) -> () {
    println!("address b {:p}", &x);
}

#[derive(Debug)]
enum Multiple {
    Int(i32),
    Float(f64),
    Text(String),
}
impl Multiple {
    fn Add(x: &Multiple, y: i32) -> i32 {
        match x {
            Multiple::Int(number) => {
                println!("sum is {}", number + y);
                number + y
            }
            _ => 0,
        }
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
