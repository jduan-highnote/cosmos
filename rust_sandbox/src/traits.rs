// One of the great discoveries in programming is that it’s possible to write code that operates on
// values of many different types, even types that haven’t been invented yet.
//
// It’s called polymorphism.
//
// Rust supports polymorphism with two related features: traits and generics. These concepts will
// be familiar to many programmers, but Rust takes a fresh approach inspired by Haskell’s
// typeclasses.
//
// Generics and traits are closely related. For example, you can write a function to compare two
// values and find the smaller one. The function signature would looke like this:
//      fn min<T: Ord>(value1: T, value2: T) -> T
//
// This function works with any type T that implements the Ord trait.
//
// Using Traits
//
// A trait is a feature that any given type may or may not support. Most often, a trait represents
// a capability: something a type can do.
//
//     A value that implements std::io::Write can write out bytes.
//
//     A value that implements std::iter::Iterator can produce a sequence of values.
//
//     A value that implements std::clone::Clone can make clones of itself in memory.
//
//     A value that implements std::fmt::Debug can be printed using println!() with the
//     {:?} format specifier.
//
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::Write;

pub fn run() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Make America Great Again"),
        location: String::from("Washington DC"),
        author: String::from("Trump"),
        content: String::from("Make America Great Again"),
    };

    println!("1 news article: {}", article.summarize3());

    notify(tweet);
    notify2(article);
}

pub trait Summary {
    fn summarize(&self) -> String;

    // trait can have methods with default implementation
    // this can be overridden by types that implement this trait
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }

    // Default implementations can call other methods in the same trait, even if those other
    // methods don’t have a default implementation. In this way, a trait can provide a lot of
    // useful functionality and only require implementors to specify a small part of it.
    // This is the "template pattern". The template itself is implemented in the trait while
    // various hooks are implemented by the types themselves.
    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// traits as parameters
// this function can be called with any type that implements Summary
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// "trait bound"
// this is equivalent to the function above, which is actually syntax sugar
fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

trait Display {
    fn show(&self) -> String;
}

// specify multiple traits using +
fn notify3<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
    println!("Show me the item: {}", item.show());
}

// "trait bound" using "where" clause between return type and open curly brace
// this is easier to read when you have many trait bounds
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    99
}

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn say_hello2<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n");
    out.flush()
}

// Find the top occurring elements from a vector.
// This is how to special a type parameter that implements multiple traits.
fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) -> Vec<&T> {
    let mut map = HashMap::new();
    for value in values {
        let counter = map.entry(value).or_insert(0);
        *counter += 1;
    }

    let mut map_vec: Vec<_> = map.iter().collect();
    map_vec.sort_by_key(|a| a.1);
    map_vec.reverse();
    let key_vec: Vec<&T> = map_vec.iter().map(|a| *a.0).collect();
    let first_n = if key_vec.len() > 10 {
        10
    } else {
        key_vec.len()
    };
    key_vec[0..first_n].to_vec()
}

trait Mapper {}
trait Reducer {}
trait Serialize {}
struct DataSet {}
// Generic functions can have multiple type parameters: M and R.
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(data: &DataSet, map: M, reduce: R) {
    // not implemented
}

// Alternative syntax: bounds can be specified in the where clause
fn run_query2<M, R>(data: &DataSet, map: M, reduce: R)
where
    M: Mapper + Serialize,
    R: Reducer + Serialize,
{
    // not implemented
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traits_need_to_be_in_scope() {
        // The Write trait needs to be in scope. Otherwise, all its methods (such as "write_all")
        // are hidden.
        use std::io::Write;

        let mut buf: Vec<u8> = vec![];
        buf.write_all(b"hello").unwrap();
        assert_eq!(5, buf.len());
    }

    #[test]
    fn trait_objects() {
        let mut buf: Vec<u8> = vec![];

        // This line doesn't compile because a variable's size has to be known at compile time and
        // types that implement Write can be any size.
        // let writer: Write = buf;

        // A reference to a trait type, like writer, is a called a "trait object". Like any other
        // reference, a trait object points to some value, it has a lifetime, and it can be either
        // mut or shared.
        let writer: &mut Write = &mut buf;

        // What makes a trait object different is that Rust usually doesn’t know the type of the
        // referent at compile time. So a trait object includes a little extra information about
        // the referent’s type. This is strictly for Rust’s own use behind the scenes: when you
        // call writer.write(data), Rust needs the type information to dynamically call the right
        // write method depending on the type of *writer. You can’t query the type information
        // directly, and Rust does not support downcasting from the trait object &mut Write back to
        // a concrete type like Vec<u8>.
        //
        // In memory, a trait object is a "fat pointer" consisting of a pointer to the value, plus
        // a pointer to a table representing that value's type. (Vec<u8> in this example)

        // Rust automatically converts ordinary references into trait objects when needed. Let's
        // say "say_hello" is a function that takes a "&mut Write", this works:
        //
        // let mut local_file: File = File::create("hello.txt")?;
        // say_hello(&mut local_file)?; // Rust converts "&mut File" to "&mut Write"
    }

    #[test]
    fn test_top_ten() {
        let names = vec![
            String::from("Oakland"),
            String::from("Oakland"),
            String::from("Oakland"),
            String::from("Alameda"),
            String::from("San Francisco"),
            String::from("San Francisco"),
        ];
        let top10 = top_ten(&names);
        assert_eq!(vec!["Oakland", "San Francisco", "Alameda"], top10);
    }
}
