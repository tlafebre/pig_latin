/*
  Convert strings to pig latin. The first consonant of each word is moved to the end of the word
  and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
  to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

fn starts_with_consonant(word: String) -> bool {
    vec!["a", "e", "i", "o", "u"].into_iter().any(|c| c == &word[0..1].to_lowercase())

}

fn main() {
    let word = String::from("Tjeerd");

    let word2 = String::from("apple");

    let word3 = String::from("Apple");
}

