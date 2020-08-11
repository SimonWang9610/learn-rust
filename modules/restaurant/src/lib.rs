#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod front_of_house;
use crate::front_of_house::hosting;

mod back_of_house;
pub use crate::back_of_house::*;

mod test;

pub fn eat_at_restaurant() {
    test::test();
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{}", meal.toast);
    println!("{:?}", back_of_house::Menu::Apple(String::from("apple")));
}
