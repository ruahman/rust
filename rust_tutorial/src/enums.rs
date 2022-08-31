enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_test(m: Movement){
    match m {
        Movement::Up => println!("move up"),
        Movement::Down => println!("move down"),
        Movement::Left => println!("move left"),
        Movement::Right => println!("move right")
    }
}

pub fn demo(){
    let avatar1 = Movement::Left;    
    let avatar2 = Movement::Right;    
    let avatar3 = Movement::Up;    
    let avatar4 = Movement::Down;    

    move_test(avatar4);
    move_test(avatar3);
    move_test(avatar2);
    move_test(avatar1);
}
