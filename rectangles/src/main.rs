#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn square (size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: dbg!(30),
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold react3? {}", rect1.can_hold(&rect3));

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width)
    }

    dbg!(&rect1);
    println!("rect1 is {:#?}", rect1);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
}


enum IpAddrKind {
    V4(String),
    V6(String),
}

struct Ipv4Addr {
    kind: IpAddrKind,
    address: String,
}

struct Ipv6Addr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

struct QuitMessage; //unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct

impl Message {
    fn call(&self) {

    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enumms () {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

}

fn route(ip_kind: IpAddrKind) {}

