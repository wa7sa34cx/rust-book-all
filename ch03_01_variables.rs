fn main() {
    let mut x = 5.0;
    println!("The value of x is: {}", x);
    x = x / 7.0;
    println!("The new value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // spaces is shadowed here
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // compilation error
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let y: u8 = 9;
    println!("The value of y is: {}", y);

    // addition
    let sum = 5 + 10;

    // division
    let quotient: f32 = -56.7 / 32.2; // default is f62
    println!("The value of quotient is: {}", quotient);

    let t = true;
    let f: bool = false;
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of third element of taple is {}", tup.2);

    // The Array Type
    let arr = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];     
    
    println!("The month is {}", months[1]);
}