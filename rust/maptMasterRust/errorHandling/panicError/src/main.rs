use std::io::Read;
use std::path::Path;
use std::fs::File;

fn main() {
    let path = Path::new("data.txt");
    let mut file = File::open(&path).expect("Error al intentar abrir data.txt");

    let mut s: String = String::new();
    file.read_to_string(&mut s).expect("Error al intentar leer los contenidos del archivo");

    println!("Leyendo la cadena {}",s);
}
