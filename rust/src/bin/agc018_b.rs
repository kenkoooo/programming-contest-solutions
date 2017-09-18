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
        read_values::<usize>()
    }).collect::<Vec<_>>();

    let mut min = n;
    let mut alive = vec![true; m];
    for _ in 0..(m - 1) {
        let mut count = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                let sport = a[i][j] - 1;
                if alive[sport] {
                    count[sport] += 1;
                    break;
                }
            }
        }

        let mut max = 0;
        for i in 0..m {
            max = cmp::max(max, count[i]);
        }
        min = cmp::min(min, max);
        for i in 0..m {
            if count[i] == max {
                alive[i] = false;
                break;
            }
        }
    }

    println!("{}", min);
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