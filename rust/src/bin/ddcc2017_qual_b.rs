use std::io;
use std::str;
use std::usize;
use std::cmp;


fn main() {
    let (a, b, c, d) = {
        let v = read_values::<usize>();
        (v[0], v[1], v[2], v[3])
    };
    println!("{}", a * 1728 + b * 144 + c * 12 + d);
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