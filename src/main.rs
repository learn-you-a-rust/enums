#[allow(dead_code)]

// each variant can have different types & amounts of 
// associated data
enum IpAddrKind {
    V4(u8, u8, u8, u8), // variant 1
    V6(String), // variant 2
}

#[derive(Debug)]
enum Message {
    Quit, // no associated data
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String), // String
    ChangeColor(i32, i32, i32), // 3 i32 values
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl Message {
    fn call(&self) {
        match self {
            Write => {
                println!("{:?}", self);
            },
            ChangeColor => (),
            Quit => (),
            Move => (),
        }
    }
}


enum Coin {
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
        Coin::Quarter(state) => { // the `state` here will bind to the 
                                  // state of the coin
            println!("State quarter from {:?}", state);
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

    // the :: syntax is used either for associated functions,
    // parts of modules,
    // or specifying enum variants as is done here
    
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call(); // call would be called on the "hello" string above

    // some options
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // using `if let ` ...
    let some_u8_value = Some(0u8);

    //match some_u8_value {
    //    Some(3) => println!("three"),
    //    _ => (),
    //}
    
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    //match coin {
    //    Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //    _ => count += 1,
    //}

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}
