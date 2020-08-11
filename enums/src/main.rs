fn main() {
    let coin = Coin::Penny;
    let coin2 = Coin::calculate(1, 2);
    value_in_cents(coin);
    // value_in_cents(coin2);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn calculate(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("penny");
            1
        }
        Coin::Nickel => {
            println!("nickel");
            5
        }
        Coin::Dime => {
            println!("dime");
            10
        }
        Coin::Quarter => {
            println!("quarter");
            25
        }
    }
}
