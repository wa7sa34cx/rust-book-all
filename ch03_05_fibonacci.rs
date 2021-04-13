fn main() {    
    let f = gen_fibonacci(15);
    println!("{:?}", f);
}

fn gen_fibonacci(num: usize) -> Vec<i32> {
    let mut f: Vec<i32> = vec![0, 1];

    if num <= 2 {
        return f;
    }

    let mut index: usize = 2;
    while index < num {
        f.push(f[index - 1] + f[index - 2]);
        index += 1;
    };

    f
}