#![allow(dead_code)]

// result is just another enum

enum Choice {
    Ant(i32),
    Bee(i32),
    Cat(i32),
    Other,
}

fn get_result(c: Choice) -> Result<i32, String> {
    match c {
        Choice::Ant(x) => Ok(x),
        Choice::Bee(x) => Ok(x),
        Choice::Cat(x) => Ok(x),
        _ => Err("you selected other".to_string()),
    }
}

// propagates the error up the call stack
fn pick_choice(c: Choice) -> Result<i32, String> {
    // the ? operator will return the error up the call stack
    // otherwise it will return the result  "return Err()"
    // buble up the error up the call stack
    let res = get_result(c)?; // give result otherwise return Err
    Ok(res)
}

pub fn results() {
    let c = Choice::Ant(32);
    let result = get_result(c);
    match result {
        Ok(x) => println!("result: {}", x),
        Err(e) => println!("error: {}", e),
    }
    let result = get_result(Choice::Other);
    match result {
        Ok(x) => println!("result: {}", x),
        Err(e) => println!("error: {}", e),
    }

    let result = pick_choice(Choice::Other);
    match result {
        Ok(x) => println!("result: {}", x),
        Err(e) => println!("error?: {}", e),
    }

    let foo: Result<String, String> = Ok::<String, String>("This is the result".into());
    if foo.is_ok() {
        let res = foo.unwrap();
        println!("I'm okay: {}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_results() {
        results();
    }
}
