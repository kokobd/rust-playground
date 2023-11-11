fn main() {
    let msg = Message::Move { x: 3, y: 4 };

    match msg {
        Message::Quit => {
            println!("Quit variant");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {}, green {}, and blue {}", r, g, b);
        }
    }

    let s = String::from("hello");
    print_str(&s);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn print_str(s: &str) {
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}