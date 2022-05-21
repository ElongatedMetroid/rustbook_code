enum IpAddrKind {
    V4,
    V6,
}

// both V4 and V6 will have associated String values
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum State {
    AK, AL, AR, AS, AZ, CA, CO, CT, DC, DE, FL, GA, GU, HI, IA, ID, IL, IN, KS, KY, LA, MA, MD, ME, MI, MN, MO, MP, MS, MT, NC, ND, NE, NH, NJ, NM, NV, NY, OH, OK, OR, PA, PR, RI, SC, SD, TN, TX, UM, UT, VA, VI, VT, WA, WI, WV, WY,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(State),
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            // Coin::Quarter(WA) <--- State will be set to WA
            Coin::Quarter(state) => {
                println!("Quarter is from the state: {:?}", state);
                25
            },
        }
    }
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
    fn call(&self) { }
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

    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(State::WA);

    println!("penny value in cents: {}", coin1.value_in_cents());
    println!("quarter value in cents: {}", coin2.value_in_cents());
}

fn route(ip_type: IpAddrKind) {}