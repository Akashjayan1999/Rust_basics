/*
----------Ownership Rules----------
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
4. A value can be moved, either copied or moved, from its owner to another variable.
5. When a value is moved, the original variable can no longer be used.
6. Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
7. rust can have mutiple refrences but only one mutable refrence

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



fn reference_examples() {
    let mut vector = vec![1, 2, 3];

    // Immutable references (&)
    let read_only = &vector[0];    // OK - multiple allowed
    let another_read = &vector[0]; // OK - can have many
    println!("{}, {}", read_only, another_read);

    // Mutable references (&mut)
    let write_access = &mut vector[0];  // OK - but only one at a time
    *write_access += 1;                 // Can modify value
    // let another_write = &mut vector[0];  // ERROR - only one mutable reference allowed
    
    // Cannot mix mut and immut references
    // let read = &vector[0];         // ERROR - cannot borrow as immutable
    // let write = &mut vector[0];    // while mutable borrow exists
    
    println!("Modified: {}", vector[0]);
}