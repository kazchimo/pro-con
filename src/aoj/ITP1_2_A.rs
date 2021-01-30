fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let vs = read_vec::<i16>();
    println!("{}", match vs[0] - vs[1] {
        n if n > 0 => "a > b",
        n if n < 0 => "a < b",
        _ => "a == b"
    })
}
