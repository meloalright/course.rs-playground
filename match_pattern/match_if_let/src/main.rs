enum Direction {
    East,
    West,
    North,
    South,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>  {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum IpAddr {
    Ipv4,
    Ipv6,
 }
 

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin1999 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

fn value_in_cents_1999(coin: Coin1999) -> u8 {
    match coin {
        Coin1999::Penny => 1,
        Coin1999::Nickel => 5,
        Coin1999::Dime => 10,
        Coin1999::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };

    value_in_cents(Coin::Penny);

    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);

    value_in_cents_1999(Coin1999::Quarter(UsState::Alaska));

    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = v {
        println!("three");
    }

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    let age = Some(30);
    println!("在匹配前，age是{:?}",age);
    if let Some(age) = age {
        println!("匹配出来的age是{}",age);
    }
 
    println!("在匹配后，age是{:?}",age);

    let age = Some(30);
    println!("在匹配前，age是{:?}",age);
    match age {
        Some(age) =>  println!("匹配出来的age是{}",age),
        _ => ()
    }
    println!("在匹配后，age是{:?}",age);
}