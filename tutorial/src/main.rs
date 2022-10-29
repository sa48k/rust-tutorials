fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    if s.len() == word.len() {
        println!("The whole string '{}' was one word", s);
    } else {
        println!("First word in the string '{}' is {}", s, word);
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}