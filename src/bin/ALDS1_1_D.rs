// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_D&lang=ja

use std::cmp::{max, min};

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// require O(n^2) order
// fn max_diff(r: &i64, rs: Vec<i64>) -> i64 {
//     rs.iter().fold(std::i64::MIN, |acc, ele| if acc < (ele - r) {
//         ele - r
//     } else {
//         acc
//     })
// }
//
// fn main() {
//     let n = read::<usize>();
//     let mut vec = Vec::with_capacity(n);
//
//     for _i in 0..n {
//         vec.push(read::<i64>());
//     }
//
//     let mut maxes = Vec::with_capacity(n);
//
//     for (i, r) in vec.iter().enumerate() {
//         if i != vec.len() - 1 {
//             maxes.push(max_diff(r, vec[(i + 1)..vec.len()].to_vec()));
//         }
//     }
//
//     println!("{}", maxes.iter().fold(&std::i64::MIN, max))
// }

fn main() {
    let n = read::<i64>();
    let mut maxv = -10_i64.pow(9);
    let mut minv = read::<i64>();

    for _ in 0..n-1 {
        let r = read::<i64>();
        maxv = max(maxv, r - minv);
        minv = min(minv, r);
    }

    println!("{}", maxv)
}
