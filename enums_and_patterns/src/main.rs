// Chapter 6 of The Rust Programming Language

fn main() {
    test_enum();
}

/*
  A possible implementation of IP Addresses using a mix of enums and structs
  enum IPAddrKind {
    V4,
    V6,
  }

  struct IPAddr {
    kind: IPAddrKind,
    address: String,
  }
*/

// An alternative IP Address implmentation using only an enum
enum IPAddr {
    V4(String),
    V6(String),
}

impl IPAddr {
    // get address from inside enum
    fn get_addr(&self) -> &str {
        match &self {
            &IPAddr::V4(v4addr) => v4addr,
            &IPAddr::V6(v6addr) => v6addr,
        }
    }

    fn get_addr_type(&self) -> &str {
        match &self {
            &IPAddr::V4(_) => "IPV4",
            &IPAddr::V6(_) => "IPV6",
        }
    }
}

// Message enum (for a state machine?)
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    // Match and print based on type in enum
    fn call(&self) {
        match &self {
            Message::Write(msg) => println!("{}", msg),
            Message::Quit => println!("Quitting!"),
            Message::Move { x, y } => println!("Moving to ({},{})", x, y),
            Message::ChangeColor(r, g, b) => println!("Changing to ({},{},{})", r, g, b),
        }
    }
}

fn test_enum() {
    //let four = IPAddrKind::V4;
    //let six = IPAddrKind::V6;
    let home = IPAddr::V4(String::from("127.0.0.1"));
    let _loopback = IPAddr::V6(String::from("::1"));
    // it turns out that IpAddr, Ipv6Addr, and Ipv4Addr are all already defined
    Message::Write(home.get_addr().to_string()).call();
    Message::Write(home.get_addr_type().to_string()).call();
    Message::ChangeColor(255, 255, 255).call();
    Message::Move { x: (0), y: (0) }.call();
    Message::Quit.call();
    for i in 0..10 {
        print_under_10(i);
    }
    options();
}

// Match on options
fn succ(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Guarded match, uses conditions to match
fn print_under_10(x: u8) {
    match x {
        i if i < 10 => println!("{}", i),
        _ => (),
    }
}

fn options() {
    let five = Some(5);
    let six = succ(five);
    let _none = succ(None);
    if let Some(some_six) = six {
        println!("Val is {}", some_six);
    }
}
