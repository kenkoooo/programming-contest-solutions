use std::io;
use std::str;
use std::usize;
use std::cmp;


fn main() {
    let stone: u8 = "S".to_owned().into_bytes()[0];
    let (h, w) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };
    let (a, b) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };
    let mut map = (0..h).map(|_| {
        let v = read_line().into_bytes();
        let mut p = vec![false; w];
        for i in 0..w {
            if v[i] == stone {
                p[i] = true;
            }
        }
        p
    }).collect::<Vec<_>>();

    let mut vertical = 0;
    let mut horizontal = 0;

    let mut ans = a + b;
    for i in 0..(h / 2) {
        for j in 0..(w / 2) {
            let b1 = map[i][j];
            let b2 = map[i][w - 1 - j];
            let b3 = map[h - 1 - i][j];
            let b4 = map[h - 1 - i][w - 1 - j];

            if b1 && b2 && b3 && b4 {
                ans += a + b + cmp::max(a, b);
            } else {
                if (b1 && b3) || (b2 && b4) {
                    vertical += 1;
                }
                if (b1 && b2) || (b3 && b4) {
                    horizontal += 1;
                }
            }
        }
    }

    ans += cmp::max(vertical * a, horizontal * b);
    let mut vv = true;
    let mut hh = true;
    for i in 0..h {
        for j in 0..w {
            vv &= map[i][j] == map[h - 1 - i][j];
            hh &= map[i][j] == map[i][w - 1 - j];
        }
    }
    if vv { ans -= a; }
    if hh { ans -= b; }

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