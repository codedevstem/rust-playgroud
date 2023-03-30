use std::io;

mod utils;
mod fizzbuzz;
mod rectangle;
use fizzbuzz::fizzbuzz::fizzbuzz as fizzbuzz;
use rectangle::rectangle::areal as rectangle_areal;

fn main() {
    
    let mut input_text = String::new();
    'outer : loop {
        println!("Please select a program to run: ");
        println!("1. fizzbuzz");
        println!("2. rectangle areal");
        println!("0. exit");
        
        io::stdin().read_line(&mut input_text).expect("Failed to read line");
        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => match i {
                1 => fizzbuzz(),
                2 => rectangle_areal(),
                0 => {
                    println!("exit");
                    break 'outer;
                },
                _ => println!("invalid input"),
            },
            Err(..) => println!("This was not an integer: {}", trimmed),
        };
        input_text.clear();
        println!("------- Next ------- \n\n\n\n")
    }
        println!("Exiting main program")
    
}

