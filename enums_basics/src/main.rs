enum IpAddrKind {
    V4,
    V6,
}

// both V4 and V6 will have associated String values
enum IpAddr {
    V4(String),
    V6(String),
}

// this enum has four varients with different types
// Quit has no data associated with it
// Move has an anonymous struct inside it
// Write has a single string
// ChangeColor includes 3 i32 values
enum Message {
    Quit,
    Move {x : i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enums can also have impl blocks
impl Message {
    fn call(&self) {

    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // attach string data to V4 and V6
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let msg = Message::Write(String::from("Hello World"));
    msg.call();
}

fn route(ip_type: IpAddrKind) {}