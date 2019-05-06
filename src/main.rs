// each variant can have different types & amounts of 
// associated data
enum IpAddrKind {
    V4(u8, u8, u8, u8), // variant 1
    V6(String), // variant 2
}

enum Message {
    Quit, // no associated data
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String), // String
    ChangeColor(i32, i32, i32), // 3 i32 values
}

impl Message {
    // self here is the associated data in the Message enum
    fn call(&self) {
        // something goes here
    }
}

fn main() {
    // the :: syntax is used either for associated functions,
    // parts of modules,
    // or specifying enum variants as is done here
    
    //let four = IpAddrKind::v4;
    //let six = IpAddrKind::v6;

    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    //fn route(ip_type: IpAddrKind) {
    //
    //};

    //route(IpAddrKind::V4);
    //route(IpAddrKind::V6);

    let m = Message::Write(String::from("hello"));
    m.call(); // call would be called on the "hello" string above

    // some options
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number = Option<i32> = None;
}
