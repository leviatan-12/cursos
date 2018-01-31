fn main() {
    let mut vector: Vec<i32> = Vec::with_capacity(5);
    for i in 1..8 {
        vector.push(i);
    }

    println!("{:?}",vector);
}
