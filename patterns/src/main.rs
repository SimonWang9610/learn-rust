use std::fmt;
use std::ops::Deref;

fn main() {
    let w = Wrapper(vec![String::from("simon"), String::from("hello")]);
    println!("{}", w);

    let a: i32 = return_closure()(0);
    println!("a {}", a);
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}], len: {}", self.0.join(", "), self.len())
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
