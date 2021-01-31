fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    for i in 1..10001 {
        let v = read::<u16>();
        if v == 0 {
            break;
        }
        println!("Case {}: {}", i, v)
    }
}
