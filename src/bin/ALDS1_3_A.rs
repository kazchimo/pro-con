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
    let vec = read_vec::<String>();
    let mut stack = Vec::<i32>::new();

    for s in vec {
        if let Ok(n) = s.parse::<i32>() {
            stack.push(n as i32);
        } else {
            match s {
                ss if ss.to_string() == "+" => {
                    let n = stack.pop().unwrap();
                    let nn = stack.pop().unwrap();
                    stack.push(nn + n)
                }
                ss if ss.to_string() == "-" => {
                    let n = stack.pop().unwrap();
                    let nn = stack.pop().unwrap();
                    stack.push(nn - n)
                }
                ss if ss.to_string() == "*" => {
                    let n = stack.pop().unwrap();
                    let nn = stack.pop().unwrap();
                    stack.push(nn * n)
                }
                _ => println!("something else: {}", s)
            }
        }
    }

    println!("{}", stack.pop().unwrap());
}
