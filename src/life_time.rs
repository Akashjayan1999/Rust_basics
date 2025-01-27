use std::fmt::Display;

pub fn main() {
   let mut x: &i32;     // Reference declaration
{
    let u = 5;       // Value with limited scope
    // x = &u;       // ERROR: Cannot assign reference to x because u will be dropped
                     // Rust prevents dangling references at compile time
}
// println!("{}", x); // ERROR: x would be dangling reference here


let string1 = String::from("Hello");
let string2 = String::from("World");

let result = longest(string1.as_str(), string2.as_str());

//&i32 is a reference type
//&str is a reference type
//&'a str is a reference type
//&'a i32 is a reference type
//&'a mut i32 is a reference type
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
fn longest2<'a>(s1: &'a str, s2: &str) -> &'a str {
    s1
}

// fn return_eg()-> &str {   //trow error
//     let s = String::from("Hello");
//     return s.as_str();
// }

fn return_eg()-> String  {  
    let s = String::from("Hello");
    return s;
}
let mut x=2;
let  result2:&str;
let  result:&str;
let string1 = String::from("Hello");
{
    let u = 5;       // Value with limited scope
    let x=u;
    let string2 = String::from("World");
     result = longest(string1.as_str(), string2.as_str());
     result2 = longest2(string1.as_str(), string2.as_str());
}
// println!("{}", result); // ERROR: result is out of scope
println!("{} value of x", x); //value will be 2
println!("{}", result2);

struct ImpotantExcerpt<'a> {
    part: &'a str,
   
}

fn struct_example<>() {
 let novaline = String::from("Call me Ishmael. Some years ago...");
 let first_sentence = novaline.as_str();
 let i = ImpotantExcerpt { part: first_sentence };
}  // if first_sentence goes out of scope, we get an error


//1. Each parameter that is a refernece gets its own lifetime parameter
// 2. if there is exactly one input parameter, the lifetime parameter is assigned to all the output parameters
// 3. if there are multiple input parameters, but one of them is &slef or &mut self, the lifetime parameter is assigned to all the output lifetime parameters
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

impl<'a> ImpotantExcerpt<'a> {
    fn return_part(&self,announcement:&str) -> &str { //third rule apply here
        println!("Attention please: {}", announcement); 
        self.part
    }
}

let s:&'static str = "Hello"; //live the life time of the program


fn longest_with_an_announcement<'a,T>(
    x: &'a str,
    y: &'a str,
    announcement: T
) -> &'a str
where
    T:Display,{
     println!("Attention please: {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y        
    }
}
    

    
}