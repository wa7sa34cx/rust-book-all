fn main() {
    //  let mut s = String::new();

    let data = "some information";
    let s = data.to_string();

    println!("{}", data);
    println!("{}", s);

    // ----

    let s2 = String::from("My future is awesome");
    println!("{}", s2);

    // ----

    let mut s3 = String::from("foo");
    s3.push_str(" bar");
    println!("{}", s3);

    // ----

    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5);
    println!("s5 is {}", s5);

    // ----

    let mut s6 = String::from("lo");
    s6.push('l');
    println!("{}", s6);

    // ---- + operator - take ownership

    let ms1 = String::from("Hello,");
    let ms2 = String::from(" world!");
    let ms3 = ms1 + &ms2;
    println!("{}", ms3);

    // ---- FORMAT!!! doesn’t take ownership

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // ---- It doesn't work

    // let s1 = String::from("hello");
    // let h = s1[0];

    // --- chars in String
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // --- bytes in String
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
