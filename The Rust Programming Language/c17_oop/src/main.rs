use c17_oop::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with {} options", self.options.len());
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 50,
                options: vec![
                    "Option 1".to_string(),
                    "Option 2".to_string(),
                    "Option 3".to_string(),
                ],
            }),
            Box::new(Button {
                width: 100,
                height: 50,
                label: "Apply".to_string(),
            }),
        ],
    };

    screen.run();
}
