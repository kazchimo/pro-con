// https://atcoder.jp/contests/abs/tasks/abc085_b
use std::collections::HashSet;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read::<usize>();
    let mut ds = Vec::<u8>::with_capacity(n);

    for _ in 0..n {
        ds.push(read::<u8>());
    }

    let u: HashSet<u8> = ds.into_iter().collect();
    println!("{}", u.len())
}
