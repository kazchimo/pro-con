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

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
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
