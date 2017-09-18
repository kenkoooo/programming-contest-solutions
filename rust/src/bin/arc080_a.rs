use std::io;
use std::io::{Read, Stdin};
use std::str;
use std::str::FromStr;
use std::usize;
use std::fmt::Debug;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.parse::<usize>();
    let a = (0..n).map(|_| {
        return sc.parse::<i32>();
    }).collect::<Vec<i32>>();

    let mut count = vec![0; 2];
    for x in &a {
        if x % 4 == 0 {
            count[0] += 1;
        } else if x % 2 != 0 {
            count[1] += 1;
        }
    }
    if count[0] >= count[1] || (count[0] + 1 == count[1] && count[0] + count[1] == n) {
        println!("Yes");
    } else { println!("No"); }
}

struct Scanner {
    stdin: Stdin,
    buf: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            stdin: io::stdin(),
            buf: Vec::with_capacity(256),
        }
    }

    fn parse<T: FromStr>(&mut self) -> T
        where <T as FromStr>::Err: Debug
    {
        self.buf.clear();
        let mut it = self.stdin.lock().bytes();
        let mut c = it.next().unwrap().unwrap();
        while c == ' ' as u8 || c == '\n' as u8 {
            c = it.next().unwrap().unwrap();
        }
        while !(c == ' ' as u8 || c == '\n' as u8) {
            self.buf.push(c);
            c = it.next().unwrap().unwrap();
        }
        str::from_utf8(&self.buf).unwrap().parse::<T>().unwrap()
    }
}