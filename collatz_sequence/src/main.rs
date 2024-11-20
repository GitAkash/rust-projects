use std::io;

fn collatz_length(mut n: i32) -> i32 {
    let mut length = 0;
    while n > 1 {
        length += 1;
        if n % 2 == 0 {
            n = n / 2;
        } else {
            
            n = n * 3 + 1;
        }
    }
    length
}

// Code of the answer, is indeed better readable.
fn collatz_length2(mut n: i32) -> i32 {
    let mut length = 0;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        length += 1
    }
    length
}

fn main() {
    println!("Enter the number that you want to evaluate its collatz sequence.");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Invalid Input");

    println!(
        "The length of the collatz sequentz of {} is {}.",
        n,
        collatz_length(n)
    )
}