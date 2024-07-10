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

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    //// enums

    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => return true,
                _ => return false,
            }
        }
    }

    let today = Days::Sunday;
    println!("Is today a weekend? {}", today.is_weekend());

    match today {
        Days::Monday => println!("Monday"),
        Days::Tuesday => println!("Tuesday"),
        Days::Wednesday => println!("Wednesday"),
        Days::Thursday => println!("Thursday"),
        Days::Friday => println!("Friday"),
        Days::Saturday => println!("Saturday"),
        Days::Sunday => println!("Sunday"),
    }

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

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_enums() {
        run()
    }
}
