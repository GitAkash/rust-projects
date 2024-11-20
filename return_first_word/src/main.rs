use std::io;


fn main() {
    println!("Enter a sentence or a word!");
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .unwrap();

    let word = first_word(&sentence);
    println!("{}", word)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // if space is found, return the characters till that space appears.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    // if no space found return the whole word.
    &s[..]
}