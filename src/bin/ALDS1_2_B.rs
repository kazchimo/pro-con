// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/2/ALDS1_2_B

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
    let n = read::<u8>();
    let mut vec = read_vec::<u8>();
    let mut count = 0;

    for i in 0..n {
        let mut minj = i;

        for j in i..n {
            if vec[j as usize] < vec[minj as usize] {
                minj = j
            }
        }

        if i != minj {
            vec.swap(i as usize, minj as usize);
            count = count + 1
        }
    }

    println_vec(&vec);
    println!("{}", count)
}
