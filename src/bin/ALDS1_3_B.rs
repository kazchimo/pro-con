// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/3/ALDS1_3_B

use std::collections::VecDeque;

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
    let nq = read_vec::<u32>();
    let n = nq[0];
    let q = nq[1];
    let mut que = VecDeque::<(String, u32)>::with_capacity(n as usize);
    let mut time = 0;

    for _ in 0..n {
        let task = read_vec::<String>();
        que.push_back((task[0].clone(), task[1].parse::<u32>().unwrap()));
    }

    loop {
        if let Some(task) = que.pop_front() {
            if task.1 <= q {
                time = time + task.1;
                println!("{} {}", task.0, time)
            } else {
                time = time + q;
                que.push_back((task.0, task.1 - q))
            }
        } else {
            break
        }
    }
}
