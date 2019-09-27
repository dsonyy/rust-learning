
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("An error occured while entering data!");

        let guess: i32 = guess.trim().parse()
            .expect("Please enter valid number!");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingo! You win with the number {}.", secret);
                break;
            }
        }
    }

}