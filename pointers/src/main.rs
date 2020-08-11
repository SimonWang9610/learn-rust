use std::ops::Deref;
use std::rc::Rc;

fn main() {
    /* let name = MyBox::new(String::from("Rust"));
    hello(&name);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    staff(c);
    staff(d); */

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("a Rc pointer count: {} +a", Rc::strong_count(&a));
    let b = List::Cons(3, Rc::clone(&a));
    println!("a Rc pointer count: {} +b", Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("a Rc pointer count: {} +c", Rc::strong_count(&a));
    }
    println!("a Rc pointer count: {} ", Rc::strong_count(&a));
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl Drop for List {
    fn drop(&mut self) {
        println!("dropping List with data{:?}", self);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello, {}", name);
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn staff(c: CustomSmartPointer) {
    println!("{:#?} created!", c);
}
