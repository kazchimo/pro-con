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

fn bubble_sort<T: PartialOrd>(vec: &mut Vec<T>) -> &Vec<T> {
    let mut conti = true;

    while conti {
        conti = false;
        for i in (1..vec.len()).rev() {
            if vec[i] < vec[i - 1] {
                vec.swap(i, i - 1);
                conti = true;
            }
        }
    }

    vec
}

fn selection_sort<T: PartialOrd>(vec: &mut Vec<T>) -> &Vec<T> {
    for i in 0..vec.len() {
        let mut minj = i;

        for j in i..vec.len() {
            if vec[j as usize] < vec[minj as usize] {
                minj = j
            }
        }

        if i != minj {
            vec.swap(i as usize, minj as usize);
        }
    }

    vec
}

fn insertion_sort<T: PartialOrd + Clone>(vec: &mut Vec<T>) -> &Vec<T> {
    for i in 1..vec.len() {
        let v = vec[i].clone();
        let mut j = i;

        while (j > 0) && (vec[j - 1] > v) {
            vec[j] = vec[j - 1].clone();
            j -= 1
        }

        vec[j] = v;
    }

    vec
}
