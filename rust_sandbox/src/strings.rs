use regex::Regex;

pub fn run() {
    create_strings();
    update_strings();
    iterate_string();
}

fn create_strings() {
    // this creates an empty string
    let mut s = String::new();
    println!("string is: {}", s);

    let data = "initial contents";
    // to_string is a method available on any type that implements the Display trait
    let s: String = data.to_string();
    println!("string is: {}", s);

    // same as above
    let s = String::from("initial contents");

    // strings are UTF-8 encoded
    let hello = String::from("你好");
    println!("string is: {}", hello);
}

fn update_strings() {
    let mut s = String::from("foo");

    // grow a string
    let s2 = "bar";
    s.push_str(s2);
    println!("string is: {}", s);
    // push_str takes a string slice because we don't necessarily want to take
    // ownership of the parameter. Otherwise, the line below won't compile.
    println!("s2 is: {}", s2);

    // append a single char
    s.push('!');
    println!("string is: {}", s);

    // combine two strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // note that s1 has been "moved" here and can no longer be used
    // The + operator uses the "add" method, whose signature looks like:
    // fn add(self, s: &str) -> String
    // This method takes ownership of s1, appends a copy of the contents of s2,
    // and then returns ownership of the result.
    let s3 = s1 + &s2;

    // this looks ugly:
    // let s = s1 + "-" + &s2 + "-" + &s3;

    // use the "format!" macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("string is: {}", s);

    // format! doesn't take ownership of s1
    println!("s1 is {}", s1);
}

// You can't index into Strings!
// A String is a wrapper over a Vec<u8>. Strings are encoded in UTF-8 format.
// Indexing into a UTF-8 encoded string doesn't really make sense because
// the "byte" for a given index may not make sense at all.
fn access_string() {
    let s = String::from("hello");
    // the line below won't compile!
    // let h = s[0];
}

fn iterate_string() {
    // If you need to perform operations on individual Unicode scalar values,
    // the best way to do so is to use the chars method.
    let s = String::from("你好");
    for c in s.chars() {
        println!("char is {}", c);
    }

    // The bytes method returns each raw byte, which might be appropriate for
    // your domain.
    for b in s.bytes() {
        println!("byte is {}", b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_literals() {
        let speech = "\"Outch!\" said the well.\n";

        println!(
            "A string literal may
            span multiple lines."
        );

        // If one line of a string ends with a backslash, then the newline and the leading
        // whitespace on the line are dropped. So this prints a single line.
        println!(
            "It was a bright, cold day in April, and \
             there were four of us \
             more or less."
        );

        // Raw strings. Everything inside a raw string are included verbatim. No escape sequences
        // are recognized.
        let default_win_install_path = r"C:\Program Files\Gorillas";
        let pattern = Regex::new(r"\d+(\.\d+)*");
    }

    #[test]
    fn byte_strings() {
        // A byte string is a slice of u8 values, ie bytes, rather than UTF8 characters.
        let method: &[u8; 3] = b"GET";
        assert_eq!(method, &[b'G', b'E', b'T']);
    }

    #[test]
    fn ways_to_create_strings() {
        // .to_string() converts a &str to a String. This copies the string.
        let error_message = "too many pets".to_string();

        // format! works just like println! except it doesn't add a newline at the end
        let date = format!("{}-{}-{}", 2019, 11, 11);
        assert_eq!(date, "2019-11-11");

        // arrays, slices, and vectors of strings have two methods (concat and join) that form a
        // new String from many strings.
        let bits = vec!["veni", "vidi", "vici"];
        assert_eq!(bits.concat(), "venividivici");
        assert_eq!(bits.join(", "), "veni, vidi, vici");
    }
}
