fn main() {

    let s = String::from("222 is CCXXII");
    let word = first_word(&s);

    println!("Full string = {}", s);
    println!("First word = {}", word);
    println!("First word with slice = {}", firts_word_slice(&s));
}


// USE iteration by String
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

//USE Slice
fn firts_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
} 
