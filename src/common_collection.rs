use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

pub fn common_collections(){



    let a = [1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new(); //vector can have only one type of value
    v.push(5);
    v.push(6);
  {
 let v2 = vec![1, 2, 3, 4, 5];
  }
  let second = v[1];
  let second2 = & v[1];
  let second3 = v[1]; //all are working
 // let third = &v[20]; //it not show error in complie time but it will show error in run time
    println!("{} second", second);
    println!("{} second2", second2);
   println!("{} second3", second3);
   match v.get(0) {
    Some(first) => println!("first is {}", first),
    None => println!("first is none"),
    }

    let mut v1 = vec![100, 32, 57];
    for i in &mut v1 {
        *i += 50;
    }
    for i in &v1 {
        println!("{} ", i);
    }

    // above are same as below
    for x in v.iter() {
        println!("{}", x); // x is &i32
    }
   
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


fn multiple_type_vector(){
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        SpreadsheetCell::Float(f) => println!("{}", f),
        SpreadsheetCell::Text(s) => println!("{}", s),
        _=> println!("None"),
    }
}


pub fn string_collection(){
    //String are stored as a collection of UTF-8 encoded bytes
    let  s1 = String::new();
    let data = "initial contents";
    let s2 = data.to_string();
    let s3: String = "initial contents".to_string();
    let s4 = String::from("initial contents");
    
    let mut s5 = String::from("foo");
    s5.push_str("bar");
    s5.push('l'); //foobar!

    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    print!("{}{}", s6, s7);
    let s8 = s6 + &s7; //s6 has been moved here and can no longer be used

    let hello =String::from("Hello, ");
    //let c:char = &hello[1]; //error
    // let c:char = hello[1]; //error


    for b in hello.bytes() {
        println!("{}", b);
    }


    for c in hello.chars() {
        println!("{}", c);
    }

    for (i, b) in hello.as_bytes().iter().enumerate() {
    println!("byte {} is {}", i, b);
   }
   let ch = hello.chars().nth(0);

   for g in hello.graphemes(true) {
    println!("{}", g);
   }

}


pub fn hashmap_collection(){
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(String::from("Blue"), 20);  //override
    scores.insert(yellow, 50);

    scores.entry(String::from("Blue")).or_insert(30); // not update the value if key is already present
    scores.entry(String::from("Blue")).or_insert(40);
    print!("{:?}", scores);
     println!("{:?}", scores.get(&String::from("Blue")));
    //  println!("{}",blue); //error borrowed value does not live long enough

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }

    for key in scores.keys() {
        println!("{}", key);
    }

    for value in scores.values() {
        println!("{}", value);
    }

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }


    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
}
 

