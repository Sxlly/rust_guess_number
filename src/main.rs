// Shae Sullivan
// COMP2007 - Curtin University
// 19/09/2024

use std::io::stdin;

fn main() {

    '_outer_loop: loop {

        let _number_to_guess: i32 = 10;

        println!("Please pick a number: ");

        loop {

            //declare an empty string variable
            let mut line = String::new();

            //Read input from user in terminal
            let _userInput = stdin().read_line(&mut line);

            //assign users input to guess variable trim string and parse it to have it as an integer
            let _guess: Option<i32> = _userInput.ok().map_or(None, |_| line.trim().parse().ok());

            //match functionality to see what the user inputted _guess matches what condition 
            match _guess{
                
                //if user input is None prompt a number to be put in again
                None => println!("you MUST enter a number!: "),
                Some(n) if n == _number_to_guess => { //if user number matches number to guess
                    println!("Thats Correct!");
                    break '_outer_loop;
                }

                Some(n) if n < _number_to_guess => println!("Higher!"),
                Some(n) if n > _number_to_guess => println!("Lower!"),
                Some(_) => println!("Error!")
            }

        }
    }
    
}
