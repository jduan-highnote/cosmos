use std::fmt::Display;

pub fn run() {
    // dangling_references();
    this_works();
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(s1.as_str(), s2);
    println!("The longest string is '{}'", result);
    calling_longest1();
}

// fn dangling_references() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     // x is out of scope here so the reference r isn't valid anymore!
//     // in other words: the subject of the reference doesn't live as long as the reference.
//     println!("r is {}", r);
// }

fn this_works() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
    println!("x: {}", x);
}

// The function signature now tells Rust that for some lifetime 'a, the function takes two
// parameters, both of which are string slices that live at least as long as lifetime 'a. The
// function signature also tells Rust that the string slice returned from the function will live at
// least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned
// by the longest function is the same as the smaller of the lifetimes of the references passed in.
// These constraints are what we want Rust to enforce. Remember, when we specify the lifetime
// parameters in this function signature, we’re not changing the lifetimes of any values passed in
// or returned. Rather, we’re specifying that the borrow checker should reject any values that
// don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly
// how long x and y will live, only that some scope can be substituted for 'a that will satisfy
// this signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// We specify a lifetime parameter 'a for the parameter x and the return type,
// but not for the parameter y, because of the lifetime of y doesn't have any
// relationship with the lifetime of x or the return value.
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// This function doesn't compile!
// When returning a reference from a function, the lifetime parameter for the return type needs to
// match the lifetime parameter for one of the parameters. If the reference returned does not refer
// to one of the parameters, it must refer to a value created within this function, which would be
// a dangling reference because the value will go out of scope at the end of the function.
// There's no way we can specify lifetime parameters that would change the dangling reference, and
// Rust won't let us create a dangling reference. In this case, the best fix would be to return an
// owned data type rather than a reference so the calling function is then responsible for cleaning
// up the value. (ie the calling function becomes the owner of the returned object)

// fn longest3<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// This function compiles.
fn calling_longest1() {
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("short string");
        let result = longest(s1.as_str(), s2.as_str());
        println!("The longest string is '{}'", result);
    }
}

// This function doesn't compile! The error is "s2 doesn't live long enough".
// In order for "result" to be valid for the println! statement, "s2" would
// need to be valid until the end of the outer scope. Rust knows this because
// we annotated the lifetimes of the function parameters and return values using
// the same lifetime parameter 'a.
// fn calling_longest2() {
//     let s1 = String::from("long string is long");
//     let result;
//     {
//         let s2 = String::from("short string");
//         result = longest(s1.as_str(), s2.as_str());
//     }
//     println!("The longest string is '{}'", result);
// }

// lifetime annotations in struct are similar to functions/methods
#[derive(Debug)]
// This lifetype annotation means an instance of `Excerpt` can't outlive the
// reference it holds in its "part" field.
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
        // If you return "announcement" instead, you would get an error of "lifetime mismatch"
        // announcement
    }
}

// This function has generic type parameter T, trait bounds (T: Display), and lifetimes.
// Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the
// generic type parameter T go in the same list inside the angle brackets after the function name.
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Note that the lifetime annotations 'a and 'b can be inferred by Rust
// based on the elision rules: each parameter that is a reference gets its
// own lifetime parameter.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let x = 12;
    // This line doesn't compile because "borrowed value doesn't live long enough".
    // The lifetime 'a can be anything while x only lives as long as this function lives.
    //    let y: &'a i32 = &x;
}

fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}

// This function is invalid: `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.
//fn invalid_output<'a>() -> &'a String {
//    &String::from("foo")
//}

/// Methods are annotated similarly to functions.
struct Owner(i32);

impl Owner {
    // Explicitly annotate lifetimes. This isn't needed due to elision rules.
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    // Explicitly annotate lifetimes. This isn't needed due to elision rules.
    fn print<'a>(&'a self) {
        println!("Owner is {}", self.0);
    }
}

#[cfg(test)]
mod tests {
    use crate::lifetime::{failed_borrow, pass_x, print_refs, Excerpt, Owner};

    #[test]
    fn explicit_lifetime_annotation() {
        print_refs(&1, &2);

        // `failed_borrow` contains no references to force `'a` to be
        // longer than the lifetime of the function, but `'a` is longer.
        // Because the lifetime is never constrained, it defaults to `'static`.
        failed_borrow();
    }

    #[test]
    fn functions_that_return_refs() {
        let (x, y) = (1, 2);
        let ref_x = pass_x(&x, &y);
        assert_eq!(1, *ref_x);
    }

    #[test]
    fn annotate_methods() {
        let mut owner = Owner(99);
        owner.add_one();
        owner.print();
        assert_eq!(100, owner.0);
    }

    #[test]
    fn annotate_structs() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence: &str = novel.split('.').next().expect("Could not find a '.'");
        let excerpt = Excerpt {
            part: first_sentence,
        };
        println!("First sentence is {:?}", excerpt);

        let announcement = String::from("Storm is coming.");
        let part = excerpt.announce_and_return_part(&announcement);
        assert_eq!(part, "Call me Ishmael");
    }
}
