enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum Option<T> {
//     Some(T),
//     None,
// }

impl Message {
    fn call(&self) {
        // method body would be defined here
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to {}, {}", x, y),
            Message::Write(text) => println!("Text: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to {}, {}, {}", r, g, b),
}
    }
    fn call_function() {}
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

  #[derive(Debug)]
     enum UsState {
            Alabama,
           Alaska,
             Arizona,
            Arkansas,
            California,
            Colorado,
     }
    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    let localHost = IpAddrKind::V4(127, 0, 0, 1); 

    let message = Message::ChangeColor(0, 0, 0);
    message.call();
    Message::call_function();
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; through error
    let sum = x+y.unwrap_or(0); //or();

    let sum2 = match y {
        Some(n) => x + n,
        None => x,
    };
    println!("{:?}", sum);

  

     value_in_cents(Coin::Quarter(UsState::Alaska));
     
    let five = plus_one(Some(5));
    let none =    plus_one(None);

    let some_3_value =  Some(3);
    match some_3_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_3_value {
        println!("three");
    }

}