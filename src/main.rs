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
