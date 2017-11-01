use std::io;
use std::str;
use std::usize;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cmp;

fn main() {
    let (n, k) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };
    let problems = (0..n).map(|_| read_values::<usize>()).collect::<Vec<_>>();

    let mut sum = vec![0; 1 << k];

    for i in 0..n {
        let mut t = 0;
        for j in 0..k {
            t *= 2;
            t += problems[i][j];
        }
        sum[t] += 1;
    }

    if sum[0] > 0 {
        println!("YES");
        return;
    }

    for mask1 in 0..(1 << k) {
        for mask2 in 0..(1 << k) {
            if mask1 & mask2 != 0 {
                continue;
            }

            if sum[mask1] > 0 && sum[mask2] > 0 {
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