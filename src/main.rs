#[allow(dead_code)]

// An IP address can only be 
// either version 4 or 6, but
// obviously not both
enum IpAddrKind {
    V4,
    V6,
}

// We can create instances of
// each of the two variants of
// IpAddrKind like this
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// We can then define a function
// that takes any IpAddrKind
fn route(ip_kind: IpAddrKind) {}

// The function can then be called
// with either variant
route(IpAddrKind::V4);
route(IpAddrKind::V6);

// We can assign types to enum
// values
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

// Rust does not have NULL
enum Option<T> {
    None,
    Some(T),
}

// Match control flow
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six + plus_one(five);
let none = plus_one(None);
