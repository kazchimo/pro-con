// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/4/ALDS1_4_C

const M: i32 = 1046527;

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

fn indexize(c: char) -> i32 {
    let s: &str = &c.to_string();
    match s {
        "A" => 1,
        "C" => 2,
        "G" => 3,
        "T" => 4,
        _ => 0
    }
}


fn hash(key: i32, i: i32) -> usize {
    fn hash1(key: i32) -> i32 {
        key % M
    }

    fn hash2(key: i32) -> i32 {
        1 + (key % (M - 1))
    }

    ((hash1(key) + i * hash2(key)) % M) as usize
}


fn get_key(s: &String) -> i32 {
    s.chars().enumerate().fold(0_i32, |acc, ele| indexize(ele.1) + (ele.0 as i32 + 1) * 5)
}

fn find(vec: &Vec<String>, s: &String) {
    let key = get_key(s);
    let mut i = 0;

    loop {
        let h = hash(key, i);
        if &vec[h] == s {
            println!("yes");
            break;
        } else if vec[h] == "" {
            println!("no");
            break;
        }

        i = i + 1;
    }
}

fn insert(vec: &Vec<String>, s: &String) -> Option<usize> {
    let key = get_key(s);
    let mut i = 0;

    loop {
        let h = hash(key, i);
        if vec[h] == "" {
            return Some(h)
        } else if &vec[h] == s {
            return None
        }

        i = i + 1
    }
}


fn main() {
    let n = read::<i32>();
    let mut vec = Vec::<String>::with_capacity(M as usize);
    for _ in 0..M {
        vec.push("".to_string())
    }

    for _ in 0..n {
        let cmd = read_vec::<String>();

        if cmd[0] == "insert" {
            if let Some(h) = insert(&vec, &cmd[1]) {
                vec[h] = cmd[1].clone()
            }
        } else if cmd[0] == "find" {
            find(&vec, &cmd[1])
        }
    }
}
