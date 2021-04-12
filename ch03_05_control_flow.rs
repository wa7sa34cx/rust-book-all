fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    let mut a = 15;

    while a != 0 {
        println!("{}", a);
        a -= 1;
    }


    let arr = ["a", "b", "c", "d", "e"];

    for element in arr.iter() {
        println!("The value is {}", element);
    }

    // yet anothe for looping

    for number in (1..4).rev() {
        println!("{}", number);
    }

}
