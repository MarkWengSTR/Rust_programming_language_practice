fn main() {
    let s = "hello world";

    let word = first_word(&s);

    println!("{}", word);

    let s2 = String::from("hello world");

    let word2 = first_word(&s2);
    println!("{}", word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

