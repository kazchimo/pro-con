fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read::<u32>();
    println!("{}:{}:{}", s / 3600, s / 60 % 60, s % 60)
}
