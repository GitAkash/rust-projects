// From 8.3 in the rust book
// Convert strings to pig latin.
// The first consonant of each word is moved to
// the end of the word and “ay” is added,
// so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added
// to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    println! {"Enter your text."};
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("error: unable to read user input");
    println!("{}", convert_sentence(&user_input));
}

fn convert_sentence(s: &str) -> String {
    s.split_whitespace()
        .map(convert_words)
        .collect::<Vec<String>>()
        .join(" ")
}

// -- SENTENCE CODE BEFORE --
// let sentence: String = s.into();
// let words: Vec<&str> = sentence.split(' ').collect();
// let modified: Vec<String> = words.into_iter().map(convert_words).collect();
// let new_sentence = modified.join(" ");
// new_sentence

fn convert_words(s: &str) -> String {
    let mut word: String = String::from(s);
    let ch = word.chars().next().unwrap();
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    match ch {
        c if vowels.contains(&c) => word.push_str("-hay"),
        c => word.push_str(&format!("-{}ay", c)),
    }
    word
}

// -- WORD CONCATENATION BEFORE --
// if vowels.contains(&ch) {
//     word.push_str("-hay");
// } else {
//     word.remove(0);
//     word.push('-');
//     word.push(ch);
//     word.push_str("ay");
