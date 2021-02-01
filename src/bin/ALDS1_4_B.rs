// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/4/ALDS1_4_B

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
    let n = read::<u32>();
    let s = read_vec::<i32>();
    read::<u32>();
    let t = read_vec::<i32>();
    let mut c = 0;

    for tt in t {
        let mut left = 0;
        let mut right = n;

        while left < right {
            let mid = (left + right) / 2;
            if s[mid as usize] == tt {
                c = c + 1;
                break;
            } else if tt < s[mid as usize] {
                right = mid
            } else {
                left = mid + 1
            }
        }
    }

    println!("{}", c)
}
