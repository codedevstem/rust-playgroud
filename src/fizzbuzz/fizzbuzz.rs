use super::super::utils::math as math;
use std::io;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, PartialEq, Eq)]
enum Output {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Output::Fizz => write!(f, "Fizz"),
            Output::Buzz => write!(f, "Buzz"),
            Output::FizzBuzz => write!(f, "FizzBuzz"),
            Output::Number(n) => write!(f, "{}", n),
        }
    }
}
    
pub fn fizzbuzz() {
    println!("Please input a number: ");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            fizzbuzz_to(i);
            println!("fizzbuzz to {} finished", i);
        }
        Err(..) => println!("This was not an integer: {}", trimmed),
    };  
}

fn fizzbuzz_to(n: u32) {
    for i in 1..n + 1 {
        println!("{}", fizzbuzz_evaluate(i));
    }
}

fn fizzbuzz_evaluate(n: u32) -> Output {
    if math::is_divisible_by(n, 15) {
        return Output::FizzBuzz;
    } else if math::is_divisible_by(n, 3) {
        return Output::Fizz;
    } else if math::is_divisible_by(n, 5) {
        return Output::Buzz;
    } else {
        return Output::Number(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fizzbuzz_evaluate() {
        assert_eq!(fizzbuzz_evaluate(1), Output::Number(1));
        assert_eq!(fizzbuzz_evaluate(2), Output::Number(2));
        assert_eq!(fizzbuzz_evaluate(3), Output::Fizz);
        assert_eq!(fizzbuzz_evaluate(4), Output::Number(4));
        assert_eq!(fizzbuzz_evaluate(5), Output::Buzz);
        assert_eq!(fizzbuzz_evaluate(6), Output::Fizz);
        assert_eq!(fizzbuzz_evaluate(7), Output::Number(7));
        assert_eq!(fizzbuzz_evaluate(8), Output::Number(8));
        assert_eq!(fizzbuzz_evaluate(9), Output::Fizz);
        assert_eq!(fizzbuzz_evaluate(10), Output::Buzz);
        assert_eq!(fizzbuzz_evaluate(11), Output::Number(11));
        assert_eq!(fizzbuzz_evaluate(12), Output::Fizz);
        assert_eq!(fizzbuzz_evaluate(13), Output::Number(13));
        assert_eq!(fizzbuzz_evaluate(14), Output::Number(14));
        assert_eq!(fizzbuzz_evaluate(15), Output::FizzBuzz);
    }
}