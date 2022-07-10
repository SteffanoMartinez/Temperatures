// Steffano Martinez | Chapter 3 Exercises | Rev 1.0//
use std::io;

fn main() {
    let mut a = 1;
    while a == 1
    {
        //All of this to get user input//
        let mut input = String::new();
        println!("Enter your temperature: ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let user_temp = input.trim().parse().expect("Not a valid number");
        println!("Is this in Celsius or Fahrenheit?:  (C/F)");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let input2 = input2.trim(); // Added this since if loop conditional woul be "C\n"
        //***************************//
        //Logic and output//
        if input2 == "C" {
            let result: f32 = cel_to_faren(user_temp);
            println!("The temperature is {:.2}", result);
        } else if input2 == "F" {
            let result: f32 = faren_to_cel(user_temp);
            println!("The temperature is {:.2}", result);
        }
        println!("Another conversion?: (y/n)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed");
        let choice = choice.trim();
        if choice == "y" { a = 1 } else { a = 0 }; 
    }
    println!("Program ended.");
    //***************************//
}

fn faren_to_cel (x: f32) -> f32 {
    (x - 32.0) * 0.5556
}

fn cel_to_faren (x: f32) -> f32 {
    (x * 1.8) + 32.0
}