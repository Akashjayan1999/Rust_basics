use std::string;


use chrono::{Local, DateTime};
mod advanced_concept;
mod compound_type;
fn main() {
    advanced_concept::advanced_function();
    advanced_concept::main();
    compound_type::main();
    let now = Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
    println!("Hello, world!");
    let ans = is_even(2343);
    print!("{}", ans);

let numbers  = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
for number in numbers.iter() {
    let ans = is_even(*number);
    println!("{}", ans);
}

'outer: loop {
    println!("Outer loop");
     loop {
        println!("Inner loop");
        break 'outer; 
    }
}

let x1  = "Hello";
let x2 = "World";
let mut x3 = String::from(x1) + " " + x2;
x3.push_str("!");
let x4 = &x3;
println!("{}", x3);

fn get_sting_length(q: &str) -> usize {
    return q.chars().count();
}

let q :&str = "Hello, world!";
let v = String::from("Hello, world!");
println!("{}", get_sting_length(q));


struct Person{
    name: String,
    age: i32,
   
}
let person = Person{
    name: String::from("John"),
    age: 23,
};
println!("Name: {}, Age: {}", person.name, person.age);

let ans = find_first_alpha_char("1234");

match ans {
    Some(c) => println!("Found: {}", c),
    None => println!("Not found"),
}
  let ans2 = get_string_length();
    match ans2 {
        Ok(n) => println!("Length: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
fn is_even(n: i64) -> bool {
    return n % 2 == 0
}


fn find_first_alpha_char(s: &str) -> Option<char> {
    for c in s.chars() {
        if c.is_alphabetic() {
            return Some(c);
        }
    }
    None
}

fn get_string_length()-> Result<usize, String> {
    let q :&str = "Hello, world!";
    let v = String::from("Hello, world!");
    return Ok(q.chars().count());
}
