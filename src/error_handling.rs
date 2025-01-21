use std::fs::{self,File};
use std::io::{self, Error, Read,ErrorKind};
use std::net::IpAddr;



pub fn error_handling() {
    // panic!("crash and burn");
    a();
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // Err(error) => {
        //     panic!("Problem opening the file: {:?}", error)
        // },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_errr => {
                panic!("Problem opening the file: {:?}", other_errr)
            }
        }
    };

    let f = File::open("hello.txt").unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn a(){
    b();
}

fn b(){
    c(31);
}

fn c(num:i32){
    if(num==32){
        panic!("Dont pass 32");
    }
   
}


fn read_username_from_file()->Result<String, std::io::Error>{
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file()->Result<String, std::io::Error>{
    fs::read_to_string("hello.txt")
}

fn main() -> Result<(),Box<dyn std::error::Error>>{ //Box<dyn std::error::Error> is a trait object va;id for any type that implements the Error trait
  let f = File::open("hello.txt")?;
  Ok(())
}

fn unwrap(){
    let f:IpAddr = "127.0.0.1".parse().unwrap(); // if it is dynmaic not use unwrap for better error handling
}