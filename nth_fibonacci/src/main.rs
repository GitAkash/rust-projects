// Doing an excercise from chapter 3 from the rustbook
use std::io;

fn main() {
    loop {
        let mut want = String::new();
        println!("Which fibonacci number do you want?");
        io::stdin()
            .read_line(&mut want)
            .expect("Failed to read line");
        let want: u32 = match want.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("fib({}) => {}", want, fibonacci(want));
        break;
    }
}

fn fibonacci(n: u32) -> u32 {
    let (mut a, mut b, mut i) = (0, 1, 0);
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    return a;
}
