fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 20,
    };

    println!("the area of rectangle is {}", rect.area());
    println!("rectangle is {:#?}", rect);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width = self.width + 1;
        self.height = self.height + 1;
        self.width * self.height
    }
}

// fn area(rectangle: &mut Rectangle) -> u32 {
//     println!("the area is {}", rectangle.width * rectangle.height);
//     rectangle.width = 31;
//     rectangle.height = 21;
//     rectangle.width * rectangle.height
// }
