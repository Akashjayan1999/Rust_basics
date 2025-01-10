// src/advanced-concept.rs
pub fn advanced_function() {
    println!("This is an advanced function!");
}

pub fn main() {
  let mut x = Vec::new();
  //vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  x.push(1);
  x.push(2);
  println!("{:?}", x);
  let y = is_even(&x);
  println!("{:?}", x);
  println!("{:?}", y);

}

fn is_even(vec :&Vec<i32>)-> Vec<i32> {
    let mut even = Vec::new();
    for i in vec{
        if *i % 2 == 0{
            even.push(*i);
        }
    }
    return even
}