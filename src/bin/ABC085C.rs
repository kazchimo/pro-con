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
    let v = read_vec::<u64>();
    let ran = if v[1] > (v[0] * 10000 / 2) {
        (0..=v[0]).rev().collect::<Vec<u64>>()
    } else {
        (0..=v[0]).collect::<Vec<u64>>()
    };

    for m in ran {
        for g in 0..=(v[0] - m) {
            let s = v[0] - m - g;
            if 10000 * m + 5000 * g + 1000 * s == v[1] {
                println!("{} {} {}", m, g, s);
                return;
            }
        }
    }

    println!("-1 -1 -1")
}
