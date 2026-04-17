use std::{cmp::Ordering, io, process::Command};

use rand::Rng;

fn main() {
    Command::new("clear").status().expect("Crash and Burn.");

    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(0..10);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // expect crasha o código inteiro, mostrando a msg

        println!("You guessed: {guess}");

        let num_input = guess
            .trim()
            .parse::<i32>()
            .expect("Não foi possível converter"); // converte para um número a string digitada

        match num_input.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
