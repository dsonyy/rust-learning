use std::{thread, time};

fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0{
            println!("Fizz Buzz!");
        }
        else if i % 3 == 0 {
            println!("Fizz!");
        }
        else if i % 5 == 0{
            println!("Buzz!");
        }
        else {
            println!("{}", i);
        }
        
        thread::sleep(time::Duration::from_millis(250));
    }
}

