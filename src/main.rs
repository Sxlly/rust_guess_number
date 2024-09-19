// Shae Sullivan
// COMP2007 - Curtin University
// 19/09/2024

use std::io::stdin;

fn main() {

    _outer_loop: loop {

        let _number_to_guess: u32 = 10;

        println!("Please pick a number: ");

        loop {

            //declare an emoty string variable
            let mut line = String::new();

            //Read input from user in terminal
            let _userInput = stdin().read_line(&mut line);

            let guess: Option<u32> = _userInput.ok()

        }
    }
    
}
