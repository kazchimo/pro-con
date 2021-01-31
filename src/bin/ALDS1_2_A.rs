use std::fmt::Display;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn println_vec<T: Display>(vec: &Vec<T>) {
    for (i, v) in vec.iter().enumerate() {
        if i != vec.len() - 1 {
            print!("{} ", v)
        } else {
            println!("{}", v)
        }
    }
}

fn main() {
    let _ = read::<u8>();
    let mut vec = read_vec::<u8>();
    let mut conti = true;
    let mut count = 0;

    while conti {
        conti = false;
        for i in (1..vec.len()).rev() {
            if vec[i] < vec[i - 1] {
                vec.swap(i, i - 1);
                conti = true;
                count = count + 1
            }
        }
    }

    println_vec(&vec);
    println!("{}", count)
}
