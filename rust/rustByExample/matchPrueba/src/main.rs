use std::char;

fn match_cadena(cadena: &str) {
    match cadena {
        "hola" => println!("Hola mundo!"),
        _ => println!("cadena default"),
    }
}

fn is_uppercase(c: u8) -> u8 {
    match c {
        b'a'...b'z' => c-32,
        _ => c,
    }
}

fn is_alfanumerico(c: u8) -> bool {
    match c {
        b'a'...b'z' | b'0'...b'9' | b'A'...b'Z' => true,
        _ => false
    }
}

fn main() {
    let cadena_1 = "hola";
    let cadena_2 = "otra";
    match_cadena(cadena_1);
    match_cadena(cadena_2);
    println!("{}", is_uppercase(b'a') as char);
    println!("a {}", is_alfanumerico(b'a'));
    println!("0 {}", is_alfanumerico(b'0'));
    println!("A {}", is_alfanumerico(b'A'));
    println!("@ {}", is_alfanumerico(b'@'));

    let tupla = (1,'a');
    let (a,b) = tupla;
    println!("{} {}",a,b );
}
