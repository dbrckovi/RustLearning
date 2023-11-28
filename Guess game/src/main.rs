use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter guess: ");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                let mut again = String::new();
                println!("Play again? (Y/N)");
                io::stdin()
                    .read_line(&mut again)
                    .expect("Failed to read line");
                println!("{again}");
                if again.trim().eq("Y") { 
                    continue;
                }
                else {
                    break;
                }
            }
        }
    }
}
