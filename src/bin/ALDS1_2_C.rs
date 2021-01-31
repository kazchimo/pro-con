// https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/2/ALDS1_2_C

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

fn bubble_sort<T: PartialOrd>(vec: &mut Vec<T>) -> &Vec<T> {
    let mut conti = true;

    while conti {
        conti = false;
        for i in (1..vec.len()).rev() {
            if vec[i] < vec[i - 1] {
                vec.swap(i, i - 1);
                conti = true;
            }
        }
    }

    vec
}

fn selection_sort<T: PartialOrd>(vec: &mut Vec<T>) -> &Vec<T> {
    for i in 0..vec.len() {
        let mut minj = i;

        for j in i..vec.len() {
            if vec[j as usize] < vec[minj as usize] {
                minj = j
            }
        }

        if i != minj {
            vec.swap(i as usize, minj as usize);
        }
    }

    vec
}

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

fn println_vec<T: Display>(vec: &Vec<T>) {
    for (i, v) in vec.iter().enumerate() {
        if i != vec.len() - 1 {
            print!("{} ", v)
        } else {
            println!("{}", v)
        }
    }
}

#[derive(PartialEq, Clone)]
struct Card(String);

impl Card {
    fn contains<T: ToString>(&self, s: &T) -> bool {
        self.0.ends_with(s.to_string().as_str())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let n = self.0.chars().last()?;
        let nn = other.0.chars().last()?;
        n.partial_cmp(&nn)
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Card(s.to_string()))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn println_sta(b: bool) {
    println!("{}", if b { "Stable" } else { "Not stable" })
}

fn main() {
    let _ = read::<u8>();
    let vec = read_vec::<Card>();
    let bubble_vect = &mut vec.clone();
    let selection_vect = &mut vec.clone();
    bubble_sort(bubble_vect);
    selection_sort(selection_vect);

    let to_number = |c: &Card| c.0.chars().last().and_then(|c| c.to_digit(10)).unwrap();

    let nums = vec.iter().map(to_number).collect::<HashSet<u32>>();

    let bubble_st = nums.iter().all(|&x| {
        vec.iter().filter(|v| v.contains(&x)).collect::<Vec<&Card>>()
            == bubble_vect.iter().filter(|v| v.contains(&x)).collect::<Vec<&Card>>()
    });
    let selection_st = nums.iter().all(|&x| {
        vec.iter().filter(|v| v.contains(&x)).collect::<Vec<&Card>>()
            == selection_vect.iter().filter(|v| v.contains(&x)).collect::<Vec<&Card>>()
    });

    println_vec(bubble_vect);
    println_sta(bubble_st);
    println_vec(selection_vect);
    println_sta(selection_st);
}
