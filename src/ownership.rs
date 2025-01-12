/*
----------Ownership Rules----------
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
*/
fn main(){

{
    let s ="Hello";
}

let x = 5;
let y = x;
// println!("x: {}, y: {}", x, y); this throw error
let z = x.clone();

let s1 = gives_ownership(String::from("Hello"));

fn gives_ownership(par:string) -> String {
    par
}

let s2 = String::from("refrences");
let len = cal_len(&mut s2);  //passing reference (Borrowing)

let mut s3 = String::from("Hello Wolrd");

let slice = &s3[..5];
let entire = &s3[..];
let slice1 = &s3[0..5];
let slice2 = &s3[0..=5];


let arr = [1,2,3,4,5];
let arr_slice = &arr[1..3];
 
}
fn cal_len(s: &mut String) -> usize {
    *s.push_str("sdf");
    s.len()
}