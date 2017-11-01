use std::io;
use std::str;
use std::usize;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cmp;

fn main() {
    let (mut h, mut m, s, mut t1, mut t2) = {
        let v = read_values::<f64>();
        (v[0], v[1], v[2], v[3], v[4])
    };
    if h == 12.0 { h = 0.0; }
    h *= 5.0;
    t1 *= 5.0;
    t2 *= 5.0;

    h += m * 5.0 / 60.0 + s * 5.0 / 3600.0;
    m += s / 60.0;

    let (from, to) = {
        if t1 > t2 {
            (t2, t1)
        } else {
            (t1, t2)
        }
    };
    let mut count = 0;
    if from < h && h < to { count += 1; }
    if from < m && m < to { count += 1; }
    if from < s && s < to { count += 1; }

    if count == 0 || count == 3 {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T>
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
{
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}