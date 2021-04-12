use std::collections::HashMap;

fn main() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 6, 8, 11, 21, 2, 3, 15, 77, 99, 13, 15, 2, 6, 77, 6];
    numbers.sort();
    println!("{:?}", numbers);
    
    println!("The average of numbers is {}", avrg(&numbers));
    println!("The median of numbers is {}", median(&numbers));

    mode(&numbers);

    // println!("{}", 5.0 / 2.0);
}

fn avrg(numbers: &Vec<i32>) -> i32 {
    let len = numbers.len();

    let mut sum = 0;
    for item in numbers {
          sum += item;
    }

    sum / len as i32 
}

//https://codereview.stackexchange.com/questions/173338/calculate-mean-median-and-mode-in-rust
// fn avrg_se(numbers: &Vec<i32>) -> f32 {    
//     numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
// }

fn median(numbers: &Vec<i32>) -> i32 {
    let len = numbers.len();
    let mid = len / 2;

    match len % 2 {
        0 => (numbers[mid] + numbers[mid + 1]) / 2,
        _ => numbers[mid],
    }
}

fn mode(numbers: &Vec<i32>) {
    let mut map = HashMap::new();

    for item in numbers {
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
