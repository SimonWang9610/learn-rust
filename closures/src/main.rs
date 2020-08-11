use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let input = 10;
    let random = 7;

    generate_workout(input, random);
}

fn expensive_calculation(x: u32) -> u32 {
    println!("calculating slowly....");
    thread::sleep(Duration::from_secs(2));
    x
}

fn generate_workout(input: u32, random: u32) {
    let mut expensive = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if input < 25 {
        println!("Today, do {} pushups", expensive.value(input));
        println!("Next, do {} situps", expensive.value(input));
    } else {
        if random == 3 {
            println!("Take a break day");
        } else {
            println!("today, run for {} minutes", expensive.value(input));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v // why v can be returned?
                  //integer has 'Copy' trait, stored on the stack
                  // String has no 'Copy' trait, allocated on the heap
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_cacher() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }
}
