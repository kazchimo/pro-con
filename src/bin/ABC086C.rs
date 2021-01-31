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

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

type Point = (i32, i32);
// type Plan = Vec<i32>;

fn take_dist(from: Point, to: Point) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

// recursive style but takes a long time
// fn simulate(plans: Vec<Plan>) -> bool {
//     return if plans.len() == 1 {
//         true
//     } else {
//         let step = take_step((plans[0][1], plans[0][2]), (plans[1][1], plans[1][2]));
//         let span = plans[1][0] - plans[0][0];
//         if (span >= step) & ((step - span) % 2 == 0) {
//             return simulate(plans[1..plans.len()].to_vec());
//         } else {
//             false
//         }
//     };
// }

fn main() {
    let n = read();
    let plans = read_vec2::<i32>(n);
    let mut start = vec![vec![0_i32, 0_i32, 0_i32]];
    start.extend(plans);

    let mut can = true;

    for (i, plan) in start.iter().enumerate() {
        if i == n as usize {
            break;
        }

        let dist = take_dist((plan[1], plan[2]), (start[i + 1][1], start[i + 1][2]));
        let step = start[i + 1][0] - plan[0];

        if (step < dist) | (dist % 2 != step % 2) {
            can = false
        }
    }

    println!("{}", if can { "Yes" } else { "No" });
}
