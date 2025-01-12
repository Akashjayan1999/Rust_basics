pub fn main(){
    let tup = ("name","akash");
    let (first,second) = tup;
    println!("{} {}", first, second); 
    let first_val = tup.0;
    let second_val = tup.1;
    println!("{} {}", first_val, second_val);

    let arr = [1,2,3,4,5];
    let first = arr[0];
    println!("{}", first);

    let arr = [2;5];
    println!{"{:?}", arr};
}