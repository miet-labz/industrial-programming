use std::io;

pub fn input() -> (f64, f64, f64, f64, f64, f64) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let v = s
        .split_whitespace()
        .map(|x| x.parse::<f64>())
        .collect::<Result<Vec<f64>, _>>()
        .unwrap();

    return (v[0], v[1], v[2], v[3], v[4], v[5]);
}

fn main() -> io::Result<()> {
    let (a, b, c, d, e, f) = input();
    if (a == 0f64) && (b == 0f64) && (c == 0f64) && (d == 0f64) && (e == 0f64) && (f == 0f64) {
        println!("5");
    } else if (a * d - c * b != 0f64) && ((e * d - b * f != 0f64) || (a * f - c * e != 0f64)) {
        let y = (a * f - c * e) / (a * d - c * b);
        let x = (d * e - b * f) / (d * a - b * c);
        println!("2 {} {}", x, y);
    } else if ((a * d - c * b == 0f64) && ((e * d - b * f != 0f64) || (a * f - c * e != 0f64)))
        || (a == 0f64 && c == 0f64 && e / b != f / d)
        || (b == 0f64 && d == 0f64 && e / a != f / c)
        || (a == 0f64 && b == 0f64 && c == 0f64 && d == 0f64 && (e / f > 0f64))
    {
        if (a == 0f64 && b == 0f64 && e == 0f64 && d != 0f64 && c == 0f64)
            || (c == 0f64 && d == 0f64 && f == 0f64 && b != 0f64 && a == 0f64)
        {
            let y;
            if b == 0f64 {
                y = f / d;
            } else if d == 0f64 {
                y = e / b;
            } else if e == 0f64 || f == 0f64 {
                y = 0f64;
            }
            // need to add
            else {
                println!("Borrow uninitialized variable: `x`");
                y = 1e9f64
            }
            println!("4 {}", y);
        } else if (a == 0f64 && b == 0f64 && e == 0f64 && c != 0f64 && d == 0f64)
            || (c == 0f64 && d == 0f64 && f == 0f64 && a != 0f64 && b == 0f64)
        {
            let x;
            if a == 0f64 {
                x = f / c;
            } else if c == 0f64 {
                x = e / a;
            } else if e == 0f64 || f == 0f64 {
                x = 0f64;
            }
            //
            else {
                println!("Borrow uninitialized variable: `x`");
                x = 1e9f64
            }
            println!("3 {}", x);
        } else {
            println!("0");
        }
    } else if a == 0f64 && c == 0f64 {
        let y;
        if e == 0f64 {
            y = f / d;
        } else if f == 0f64 {
            y = e / b;
        } else {
            y = e / b;
        }
        println!("4 {}", y);
    } else if b == 0f64 && d == 0f64 {
        let x;
        if e == 0f64 {
            x = f / c;
        } else if f == 0f64 {
            x = e / a;
        } else {
            x = e / a;
        }
        println!("3 {}", x);
    } else if b == 0f64 && e == 0f64 {
        let (k, n);
        k = -c / d;
        n = f / d;
        println!("1 {} {}", k, n);
    } else if d == 0f64 && f == 0f64 {
        let (k, n);
        k = -a / b;
        n = e / b;
        println!("1 {} {}", k, n);
    } else if a == 0f64 && e == 0f64 {
        let (k, n);
        k = -d / c;
        n = f / c;
        println!("1 {} {}", k, n);
    } else if c == 0f64 && f == 0f64 {
        let (k, n);
        k = -b / a;
        n = e / a;
        println!("1 {} {}", k, n);
    } else if a / b == c / d {
        let (k, n);
        k = -c / d;
        n = f / d;
        println!("1 {} {}", k, n);
    } else {
        println!("Are you kidding me?");
    }
    return Ok(());
}

#[cfg(test)]
mod test {
    use super::*;
}
