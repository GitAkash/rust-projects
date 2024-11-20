// Excercise from chapter 3 in the rustbook
use std::io;

fn main() {
    println!("Enter the temperature in Fahrenheit!");

    loop {
        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: i32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius = (fahrenheit - 32) * 5 / 9;
        println!("The temperature in celsius would be: {celsius}");
        break;
    }
}
