#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32), // tuple struct
}

fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);

    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };

    println!("{:?}", c2);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::ChangeColor(255, 255, 0);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    match y {
        None => {
            println!("NONE");
        }
        Some(i) => {
            let sum = x + i;
            println!("sum = {}", sum);
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
