use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(),"Usage: gcd NUMBER").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = maximo_comun_divisor(d,*m);
    }

    println!("El máximo común divisor de {:?} es: {}", numbers ,d);
}

fn maximo_comun_divisor(m: u64, n: u64) -> u64 {
    if n == 0 {
        return m;
    } else if m == 0 {
        return n;
    };
    if n > m {
        return maximo_comun_divisor(n,m);
    };
    let mut t = m;
    let mut u = n;
    let mut v;
    while u != 0 {
        v = u;
        u = t % u;
        t = v;
    }

    t
}
