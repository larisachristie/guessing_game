use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
  println!("Welcome to the guessing game!");
  println!("I have a number in mind. Can you guess it in five rounds?");
  println!("(Enter a number between 1 and 100)");

  let number: u8 = rand::thread_rng().gen_range(1, 101);

  for count in 1..6 {
    let mut user_guess = String::new();
    io::stdin()
      .read_line(&mut user_guess)
      .expect("Failed to read line");
    
    let user_guess: u8 = match user_guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match user_guess.cmp(&number) {
      Ordering::Less => {
        if count < 5 {
          println!("Your guess: {}. Too small, try again!", user_guess);
        } else {
          println!("Your guess: {}. Too small, you lose!", user_guess);
          break;
        }
      },
      Ordering::Greater => {
        if count < 5 {
          println!("Your guess: {}. Too big, try again!", user_guess);
        } else {
          println!("Your guess: {}. Too big, you lose!", user_guess);
          break;
        }
      },
      Ordering::Equal => {
        println!("Your guess: {}. You win!", user_guess);
        break;
      }
    }
  }
}