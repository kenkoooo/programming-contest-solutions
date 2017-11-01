use std::io;
use std::str;
use std::usize;
use std::cmp;


fn main() {
    let s = read_line();
    if s.clone().into_bytes()[0] == s.clone().into_bytes()[1] && s.clone().into_bytes()[1] != s.clone().into_bytes()[2] && s.clone().into_bytes()[2] == s.clone().into_bytes()[3] {
        println!("Yes");
    } else {
        println!("No");
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