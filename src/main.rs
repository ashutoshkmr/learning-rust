fn main() {
    let s = String::from("Hello");
    let len = calculate_len(&s[..]);
    println!("Length of s = {}", len);
    let len = first_word(&s[..]);
    println!("Length of first word = {}", len);
}

fn calculate_len(s: &str) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
