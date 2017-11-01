use std::io;
use std::str;
use std::usize;
use std::cmp;


fn main() {
    let (n, c) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };
    let mut v = (0..n).map(|_| { read_values::<usize>()[0] }).collect::<Vec<_>>();
    v.sort();

    let mut ans = 0;
    let mut head = 0;
    let mut tail = n - 1;
    while head <= tail {
        if head == tail {
            ans += 1;
            break;
        } else if v[head] + v[tail] + 1 <= c {
            head += 1;
            tail -= 1;
            ans += 1;
        } else {
            tail -= 1;
            ans += 1;
        }
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