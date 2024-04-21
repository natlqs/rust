fn main() {

    let s1 = String::from("hello");
    let s2 = "syaccad";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);

}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
