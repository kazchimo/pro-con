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

// fn main() {
//     read::<u16>();
//     let s = read_vec::<i32>();
//     read::<u16>();
//     let t = read_vec::<i32>();
//     let mut c = 0;
//
//     for tt in t {
//         if s.contains(&tt) {
//             c = c + 1
//         }
//     }
//
//     println!("{}", c)
// }

// with terminator
fn main() {
    read::<u16>();
    let s = read_vec::<i32>();
    read::<u16>();
    let t = read_vec::<i32>();
    let mut c = 0;

    for tt in t {
        let mut i = 0;
        let mut ss = s.clone();
        ss.push(tt);

        while ss[i] != tt {
            i = i + 1
        }

        if i != ss.len() - 1 {
            c = c + 1
        }
    }

    println!("{}", c)
}
