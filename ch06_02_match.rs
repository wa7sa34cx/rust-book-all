enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    let my_coin = Coin::Penny;
    println!("{}", valuer_in_cents(my_coin));

    let my_qcoin = Coin::Quarter(UsState::Alaska);
    valuer_in_cents(my_qcoin);

    // ----

    let x = Some(5);
    let y = None;

    let x = plus_one(x);
    println!("x is equal {:?}", x);

    let y = plus_one(y);
    println!("x is equal {:?}", y);
    
    // ----
    
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // -- if let -- 
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

}

fn valuer_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}