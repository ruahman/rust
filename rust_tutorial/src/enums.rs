enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    Rbg(u8, u8, u8),
    Rgb { red: u8, blue: u8, green: u8 },
}

fn move_test(m: Movement) {
    match m {
        Movement::Up => println!("move up"),
        Movement::Down => println!("move down"),
        Movement::Left => println!("move left"),
        Movement::Right => println!("move right"),
    }
}

pub fn demo() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_test(avatar4);
    move_test(avatar3);
    move_test(avatar2);
    move_test(avatar1);

    let c = Color::Rbg(1, 2, 3);

    match c {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::Rbg(x, y, z) => println!("{}{}{}", x, y, z),
        Color::Rgb {
            red,
            blue,
            green: x,
        } => println!("{}{}{}", red, blue, x),
    }
}
