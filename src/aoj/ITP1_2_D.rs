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
    let v = &read_vec::<i16>()[..];
    let (w, h, x, y, r) = (v[0], v[1], v[2], v[3], v[4]);

    println!("{}", if (x <= 0) | (y <= 0) | (x + r > w) | (y + r > h) {
        "No"
    } else {
        "Yes"
    })
}

