use std::io;
use std::str;
use std::usize;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cmp;

fn main() {
    let pass = read_line();
    let n = read_values::<usize>()[0];
    let words = (0..n).map(|_| read_line()).collect::<Vec<_>>();
    for i in 0..n {
        if pass == words[i] {
            println!("YES");
            return;
        }
        for j in 0..n {
            let b1 = pass.clone().into_bytes()[0];
            let b2 = pass.clone().into_bytes()[1];
            if b1 == words[i].clone().into_bytes()[1] && b2 == words[j].clone().into_bytes()[0] {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
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