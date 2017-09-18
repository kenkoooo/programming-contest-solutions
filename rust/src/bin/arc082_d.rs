use std::io;
use std::str;
use std::usize;
use std::cmp;


static INF: i32 = 2000000000;

fn main() {
    let x = read_values::<i32>()[0];
    let k = read_values::<usize>()[0];
    let r = {
        let mut r = read_values::<i32>();
        r.push(INF);
        r
    };
    let q = read_values::<usize>()[0];
    let (t, a) = {
        let mut t = vec![INF; q + 1];
        let mut a = vec![INF; q + 1];
        for i in 0..q {
            let v = read_values::<i32>();
            t[i] = v[0];
            a[i] = v[1];
        }
        (t, a)
    };

    let mut y = 0;
    let mut x1 = 0;
    let mut x2 = x;
    let mut upper = true;

    let mut flat = false;
    let mut flat_height = 0;

    let mut r_head = 0;
    let mut t_head = 0;
    let mut prev_time = 0;
    while r_head < k || t_head < q {
        if r[r_head] < t[t_head] {
            // turn
            let time = r[r_head] - prev_time;
            if upper {
                y -= time;
                if y < 0 && x1 < -y {
                    x1 = -y;
                }
            } else {
                y += time;
                if y > 0 && x2 > x - y {
                    x2 = x - y;
                }
            }

            if flat {
                if upper {
                    flat_height -= time;
                } else {
                    flat_height += time;
                }
                flat_height = cmp::max(flat_height, 0);
                flat_height = cmp::min(flat_height, x);
            } else if x1 >= x2 {
                flat = true;
                if upper {
                    flat_height = 0;
                } else {
                    flat_height = x;
                }
            }

            prev_time = r[r_head];
            upper = !upper;
            r_head += 1;
        } else {
            // query
            let time = t[t_head] - prev_time;

            let mut ans = if a[t_head] < x1 {
                x1 + y
            } else if x2 < a[t_head] {
                x2 + y
            } else {
                a[t_head] + y
            };

            if upper {
                ans -= time;
            } else {
                ans += time;
            }

            if flat {
                ans = flat_height;
                if upper {
                    ans -= time;
                } else {
                    ans += time;
                }
            }

            ans = cmp::max(0, ans);
            ans = cmp::min(x, ans);
            println!("{}", ans);

            t_head += 1;
        }
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