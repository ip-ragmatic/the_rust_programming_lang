/* 
 * Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
 *
 * `to_list()`` takes in a string literal and outputs a vector of each component of the string; this includes properly separating alphanumeric and non-alphanumeric parts of the string.
 * - `result` is the output vector
 * - `last` is an index tracker for when there's a non-alphanumeric character. 
 * - In the loop, if `last` doesn't equal the `index` of the matched character, then the program appends a slice of text with range last..index in `text` to `result`; this slice will be a non-breaking sequence of alphanumeric characters (a word or something). Now we can consider the non-alphanumeric character at `index`, and append it to `result`. Finally, the loop adds the length of the character (1) to `last` so that we can add to `result` any alphanumeric sequences between non-alphanumerics again (using `result.push(&text[last..index])`)
 *
 */

const VOWELS: &str = "AEIOUaeiou";

fn to_list(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| !(c.is_alphanumeric())) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

fn to_pig(word: &str) -> String {
    match word.chars().all(|ch| ch.is_alphabetic()) {
        true => {
            if VOWELS.contains(&word[0..1]) {
                format!("{}-hay", word)
            } else {
                let ch = word.chars().next().unwrap().to_lowercase();
                format!("{}-{}ay", &word[1..], ch)
            }
        }
        false => format!("{}", word),
    }
}

fn main() {
    let string = "I think; I gOT, it. ::'figured' out!!!🔥✅🔥";
    
    let text_list = to_list(&string);
    let mut pig = Vec::new();
    for word in text_list {
        pig.push(to_pig(word));
    }

    // println!("{:?}", pig);
    println!("{}", pig.join(""));
}

/* // the following to functions are the unicode handling version. they don't work right though lol. things print weird. this string gets a runtime error using the above code, but doesn't print right with the below code "the symbol ᾭ is alphabetical; but, it's non-ASCII"

fn to_list(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, _) in text.char_indices().filter(|&(_, c)| !(c.is_alphanumeric() || c == '\'')) {
        if last != index {
            result.push(&text[last..index]);
        }
        let matched = &text[index..index + text[index..].chars().next().unwrap().len_utf8()];
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

fn to_pig(word: &str) -> String {
    match word.chars().all(|ch| ch.is_alphabetic()) {
        true => {
            let first_char = word.char_indices().next().unwrap();
            // println!("{:?}", first_char);
            if VOWELS.contains(&first_char.1.to_string()) {
                format!("{}-hay", word)
            } else {
                // let ch = first_char.1.to_lowercase();
                // format!("{}-{}ay", &word[first_char.0 + first_char.1.len_utf8()..], ch)
                format!("{}", &first_char.1)
            }
        }
        false => format!("{}", word),
    }
}
 */