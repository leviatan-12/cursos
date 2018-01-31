fn square(x: u32) -> u32 {
    x * x
}

fn function_sin_var() {
    println!("Se entró a una función sin variables");
}

fn main() {
    let clousure_1 = | x:u32 | x*x;
    let clousure_2 = | x:u32 | { x*x };
    let clousure_3 = | x:u32 | -> u32 { x*x };

    let closure_without_vars = || println!("Entered closure without variables");

 println!("square of 4 = {}", square(4));
 println!("square of 4 = {}", clousure_1(4));
 println!("square of 4 = {}", clousure_2(4));
 println!("square of 4 = {}", clousure_3(4));

 function_sin_var();
 closure_without_vars();
}
