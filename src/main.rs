// Convert string to pig latin. The first consonant of each word is moved
// to the end of the word and "ay" is added, so "first" becomes "irst-fay".
// Words that start with a vowel have "hay" added to the end instead 
// ("apple" becomes "apple-hay"). Keep in mind the details about UTF.8
// enconding!
use std::io;

fn convert_for_consonant(word: String) -> String { 
    let mut letters = word.chars();
    let first_letter = letters.next(); 
    let cropped_word = letters.as_str();
    let mut new_word = String::from(cropped_word);

    match first_letter {
        Some(letter) => new_word.push(letter),
        None => (),
    }
    new_word.push_str("ay");
    new_word
}

fn convert_for_vocal(mut word: String) -> String { 
    word.push_str("hay");
    word.to_string();
    word
}

fn is_vowel(c: Option<char>) -> bool {
    match c {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => true,
        _ => false,
    }
}

fn main() {
    println!("Please input a word.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input_letters = input.chars();
    let input_first_letter = input_letters.next();

    if is_vowel(input_first_letter) {
        println!("{}", convert_for_vocal(input));
    } else {
        println!("{}", convert_for_consonant(input));
    }
}   
