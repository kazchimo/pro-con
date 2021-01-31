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

fn main() {
    loop {
        let v = read_vec::<u16>();

        if (v[0] == 0) & (v[1] == 0) {
            break;
        }

        if v[0] < v[1] {
            println!("{} {}", v[0], v[1])
        } else {
            println!("{} {}", v[1], v[0])
        }
    }
}
