// https://atcoder.jp/contests/abs/tasks/arc065_a

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let mut s = read::<String>();
    let words = ["dream", "dreamer", "erase", "eraser"];

    loop {
        for w in &words {
            if s.ends_with(w) {
                s.truncate(s.len() - w.len());
                break;
            } else if w == &"eraser" {
                println!("NO");
                return
            }
        }

        if s.len() == 0 {
            println!("YES");
            return;
        }
    }
}

