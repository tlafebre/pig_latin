/*
  Convert strings to pig latin. The first consonant of each word is moved to the end of the word
  and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
  to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

fn is_vowel(letter: &str) -> bool {
    vec!["a", "e", "i", "o", "u"].into_iter().any(|c| c == &letter.to_lowercase())
}

fn pigify_one(word: String) -> String {
    let first_letter = &word[0..1];
    if is_vowel(first_letter) {
        format!("{}-hay", &word)
    } else {
        format!("{}-{}ay", &word[1..], &first_letter)
    }
}

fn pigify(string: &str) {
    let mut new_str = String::from("");

    for word in string.split_whitespace() {
        new_str.push_str(&(pigify_one(word.to_string())));
        new_str.push(' ');
    }
    println!("{}", &new_str);
}

fn main() {
    let word = String::from("Tjeerd likes to write in Rust and eat a lot of apples");
    pigify(&word);
}
