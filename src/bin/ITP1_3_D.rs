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
    let v = read_vec::<u16>();
    let mut c = 0_u16;

    for n in v[0]..=v[1] {
        if v[2] % n == 0 {
            c = c + 1
        }
    }

    println!("{}", c)
}
