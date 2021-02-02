// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/5/ALDS1_5_A
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

fn solve(a: &Vec<i32>, i: i32, m: &i32, n: &i32) -> bool {
    if m == &0 {
        true
    } else if &i >= n {
        false
    } else {
        solve(a, (i + 1), m, n) || solve(a, (i + 1), &(m - a[i as usize]), n)
    }
}


fn main() {
    let n = read::<i32>();
    let a = read_vec::<i32>();
    let _ = read::<i32>();
    let m = read_vec::<i32>();

    for mm in m  {
        if solve(&a, 0, &mm, &n) {
            println!("yes")
        } else {
            println!("no")
        }
    }
}
