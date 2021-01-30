// https://atcoder.jp/contests/abs/tasks/abc088_b

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
    let _ = read::<u16>();
    let mut avec = read_vec::<u16>();

    avec.sort();
    avec.reverse();

    let score = avec
        .iter()
        .enumerate()
        .fold((0_u16, 0_u16), |acc, ele| {
            if ele.0 % 2 == 0 {
                (acc.0 + ele.1, acc.1)
            } else {
                (acc.0, acc.1 + ele.1)
            }
        });

    println!("{}", score.0 - score.1)
}
