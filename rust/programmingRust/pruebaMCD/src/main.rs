fn maximo_comun_divisor(m: i64, n: i64) -> i64 {
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

fn main() {
    let a = -200;
    let b = -36;
    println!("El máximo común divisor entre {} y {}  es {}", a,b,maximo_comun_divisor(a,b));
}

#[test]
fn test_maximo_comun_divisor() {
    assert_eq!(2,maximo_comun_divisor(100,2));
}
