use std::{i64, process};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = egcd(a, m);
    if g == 1 {
        Some((x % m + m) % m)
    } else {
        None
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();

    let subcommand = args.next().unwrap_or_else(|| {
        eprintln!("Usage: mod <subcommand> <args>");
        process::exit(1);
    });

    match subcommand.as_str() {
        "--gcd" => {
            if args.len() != 2 {
                eprintln!("Usage: mod --gcd <a> <modulus>");
                process::exit(1);
            }

            let a = args.next().unwrap().parse::<i64>().unwrap();
            let b = args.next().unwrap().parse::<i64>().unwrap();

            let gcd_val = gcd(a, b);
            println!("GCD({}, {}): {}", a, b, gcd_val);
        }
        "--inv" => {
            if args.len() != 2 {
                eprintln!("Usage: mod --inv <a> <modulus>");
                process::exit(1);
            }

            let a = args.next().unwrap().parse::<i64>().unwrap();
            let modulus = args.next().unwrap().parse::<i64>().unwrap();

            let inv = mod_inv(a, modulus).unwrap();
            println!("Modular inverse of {} (mod {}): {}", a, modulus, inv);
        }
        "--egcd" => {
            if args.len() != 2 {
                eprintln!("Usage: mod --egcd <a> <modulus>");
                process::exit(1);
            }

            let a = args.next().unwrap().parse::<i64>().unwrap();
            let b = args.next().unwrap().parse::<i64>().unwrap();

            let (g, x, y) = egcd(a, b);
            println!(
                "Extended GCD({}, {}): (gcd: {}, s: {}, t: {})",
                a, b, g, y, x
            );
        }
        "-" => {
            if args.len() != 2 {
                eprintln!("Usage: mod - <a> <modulus>");
                process::exit(1);
            }
            let a = args.next().unwrap().parse::<i64>().unwrap();
            let b = args.next().unwrap().parse::<i64>().unwrap();

            println!("{} % {} = {}", a, b, a % b);
        }
        _ => {
            eprintln!("Invalid subcommand. Valid options are: --gcd, --egcd, --inv and - for modulo operations");
            process::exit(1);
        }
    }
}
