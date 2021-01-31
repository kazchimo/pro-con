// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/3/ALDS1_3_C

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
    let n = read::<i32>();
    let mut list = Vec::new();

    for _ in 0..n {
        let cmd = read_vec::<String>();

        match cmd[0].as_str() {
            "insert" => list.insert(0, cmd[1].parse::<i32>().unwrap()),
            "delete" => {
                let idx = list.iter().enumerate().find_map(|x| if x.1 == &cmd[1].parse::<i32>()
                    .unwrap() {
                    Some(x.0)
                } else {
                    None
                });

                if let Some(i) = idx {
                    list.remove(i);
                }
                ()
            }
            "deleteFirst" => {
                list.remove(0);
                ()
            }
            "deleteLast" => {
                list.remove(list.len() - 1);
                ()
            },
            _ => println!("something else")
        }
    }

    println_vec(&list);
}

