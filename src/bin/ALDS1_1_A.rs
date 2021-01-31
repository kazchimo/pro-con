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
    let n = read::<usize>();
    let mut vec = read_vec::<u16>();
    println_vec(&vec);

    for i in 1..n {
        let v = vec[i];
        let mut j = i;

        while (j > 0) && (vec[j - 1] > v) {
            vec[j] = vec[j - 1];
            j -= 1
        }

        vec[j] = v;

        println_vec(&vec);
    }
}
