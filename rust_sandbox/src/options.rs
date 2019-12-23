/// You can use the ? operator on Options.  If x is an Option, then evaluating x? will
/// 1. return the underlying value if x is Some,
/// 2. otherwise it will terminate whatever function is being executed and return None.
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age))
}

pub fn run() {
    let some_number = Some(5);
    let some_string = Some("a string");
    // the type is needed because Rust can't infer the type for None
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match six {
        Some(i) => println!("five plus one is {}", i),
        None => println!("this should never happen"),
    }

    placeholder(3);
    placeholder(7);
    placeholder(30);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn placeholder(num: u8) {
    match num {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // use _ to match everything else!
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_birthday() {
        let age = None;
        assert_eq!(None, next_birthday(age));
    }
}
