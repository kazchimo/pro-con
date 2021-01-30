// https://atcoder.jp/contests/abs/tasks/abc081_b

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
    read::<u8>();
    let avec = read_vec::<u32>();
    let mut c = 0_u8;
    let mut stop = false;

    loop {
        for a in &avec {
            if a % 2_u32.pow((c + 1) as u32) != 0 {
                stop = true;
                break;
            }
        }

        if stop {
            break
        }

        c = c + 1
    }

    println!("{}", c)
}
