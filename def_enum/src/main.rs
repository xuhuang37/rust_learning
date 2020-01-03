#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("{}","2133213");
    }
}

fn route(kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Option<T>{
    Some(T),
    None,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback_2 = IpAddr2::V6(String::from("::1"));

    let home_3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback_3 = IpAddr3::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number : Option<i32> = None;

    let x:i8 = 5;
    let y:Option<i8> = some(5);

    // let sum = x + y; 
}
