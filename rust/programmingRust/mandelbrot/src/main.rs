extern crate num;

use num::Complex;

mod parseo;

fn tiempo_escape(c: Complex<f64>, limite: u32) -> Option<u32> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limite {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn main() {
    println!("Hello, world!");
}
