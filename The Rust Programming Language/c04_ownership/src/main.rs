
#[allow(unused_variables)]
fn main() {

    let m = Message::Write(String::from("Hello, world!"));
    m.call();
    let n = Message::Quit;
    n.call();

    let o = Message::Move { x: 10, y: 20 };
    o.call();

    let p = Message::ChangeColor(255, 0, 0);
    p.call();
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let some_nothing:Option<i32> = None;
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
        }
    }
}