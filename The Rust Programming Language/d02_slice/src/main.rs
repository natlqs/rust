fn main() {
    let s = String::from("hello world");
    let a = first_word(&s);
    // let hello = &s[0..5];
    // let he = &s[..2];
    // let world = &s[6..11];
    // let wo = &s[6..];
    // println!("{}, {}, {}, {}", hello, world, he, wo);
    println!("{}", a);
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}