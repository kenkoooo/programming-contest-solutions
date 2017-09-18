use std::io;
use std::str;
use std::usize;
use std::cmp;
use std::collections::BTreeSet;

fn main() {
    let (n, k) = {
        let v = read_values::<usize>();
        (v[0], v[1] as u64)
    };

    let mut v = read_values::<u64>();
    let set: BTreeSet<_> = v.drain(..).collect();
    v.extend(set.into_iter());
    if v.len() == 1 {
        if k == v[0] {
            println!("POSSIBLE");
        } else {
            println!("IMPOSSIBLE");
        }
        return;
    }

    v.sort();
    let mut g = v[1] - v[0];
    for i in 0..(v.len() - 1) {
        g = gcd(g, v[i + 1] - v[i]);
    }

    if k <= v[v.len() - 1] && (v[v.len() - 1] - k) % g == 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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