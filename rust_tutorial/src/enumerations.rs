#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

#[allow(unused_variables)]
pub fn run() {
    println!("Enumerations...");
    // let c: Color = Color::Red;
    // let c: Color = Color::RgbColor(10, 0, 0);
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        Color::CmykColor { black: 0, .. } => println!("white"),
        _ => println!("some other color"),
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_enumerations() {
        run()
    }
}
