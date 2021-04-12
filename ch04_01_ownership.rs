fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);

    // -------

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    // -------

    // let arr = [1, 2, 3, 4, 5];
    // calc_sum(arr);

    // -------

    let s2 = String::from("Google");
    let len = calc_len(&s2);
    println!("The length of '{}' is {}", s2, len);

    // -------

    let mut s3 = String::from("abc");
    change(&mut s3);
    println!("{}", s3);

    // -------

    let s4 = String::from("Happy New Year!");
    let i = first_word(&s4);
    println!("{}", i);

    let happy = &s4[0..5];
    let new = &s4[6..9];
    
    println!("{}, {}", happy, new);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(x: u32) {
    let x = x + 1; 
    println!("{}", x);
}

// fn calc_sum(arr: ) {
//     7
// }

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("def");
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    // s.len()
    &s[..]
}