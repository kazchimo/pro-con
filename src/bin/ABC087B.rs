// https://atcoder.jp/contests/abs/tasks/abc087_b
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn search(a: u16, b: u16, c: u16, x: u16) -> u16 {
    let mut res = 0;

    for aa in 0..=a {
        for bb in 0..=b {
            for cc in 0..=c {
                if 500 * aa + 100 * bb + 50 * cc == x {
                    res += 1
                }
            }
        }
    }

    res
}


fn main() {
    let a = read::<u16>();
    let b = read::<u16>();
    let c = read::<u16>();
    let x = read::<u16>();

    let res = if x / 50 % 2 == 1 {
        if c < 1 {
            0
        } else {
            search(a, b, c, x)
        }
    } else {
        if c <= 1 {
            search(a, b, 0, x)
        } else {
            search(a, b, c, x)
        }
    };

    println!("{}", res)
}
