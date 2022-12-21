fn main() {
    let s = String::from("Testing a string");
    let f_word = first_word_index(&s);
    println!("The first word is {}", f_word);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);
}

// &str is a string slice
fn first_word_index(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
