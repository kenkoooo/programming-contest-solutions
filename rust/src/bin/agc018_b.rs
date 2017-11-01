use std::io;
use std::str;
use std::usize;
use std::cmp;


fn main() {
    let (n, m) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };
    let a = (0..n).map(|_| {
        let mut v = read_values::<usize>();
        for i in 0..v.len() {
            v[i] -= 1;
        }
        v
    }).collect::<Vec<_>>();

    let mut dead = vec![false; m];
    let mut ans = n;
    let mut heads = vec![0; n];
    for _ in 0..(m - 1) {
        let mut count = vec![0; m];
        for i in 0..n {
            while dead[a[i][heads[i]]] {
                heads[i] += 1;
            }
            count[a[i][heads[i]]] += 1;
        }

        let mut largest = a[0][heads[0]];
        for j in 0..m {
            if count[largest] < count[j] {
                largest = j;
            }
        }
        ans = cmp::min(ans, count[largest]);

        for i in 0..n {
            if a[i][heads[i]] == largest {
                heads[i] += 1;
            }
        }
        dead[largest] = true;
    }

    println!("{}", ans);
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