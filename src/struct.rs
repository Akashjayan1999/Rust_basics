struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32); //tuple struct

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

   
}

impl Rectangle{
     fn square(size: u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
}
pub fn main(){

    let mut user1 = User{
        email: String::from("akash@gmail.com"),
        username: String::from("akash123"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("aa123");
    let user2 = build_user(String::from("another@somewhere"), String::from("anotherusername567"));
 
    let user3 = User{
        email: String::from("another@somewhere"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let color = Color(0, 0, 0);

    let rect = Rectangle{width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels.", rect.area());
    let square = Rectangle::square(3);
    println!("The area of the square is {} square pixels.", square.area());
    
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}