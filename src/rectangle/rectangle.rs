use std::io;


pub fn areal() -> () {
    let length = read_number_input("length");
    let width = read_number_input("width");
    println!("The areal of the rectangle is: {}", length * width);
}

pub fn read_number_input(named_param: &str) -> u32 {
    let mut input_text = String::new();
    println!("Please input the {} of the rectangle: ", named_param);
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let number = input_text.trim().parse::<u32>().expect("Please type a number!");
    return number;
}