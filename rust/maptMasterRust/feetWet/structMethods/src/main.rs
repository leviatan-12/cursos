use std::collections::HashMap;

fn main() {
    let personaje_prueba = Personaje::nuevo_nombrado("Daniel".to_string());
    println!("{}", personaje_prueba.get_fuerza());
    prueba_vec_arr_tup_hash();
}

struct Personaje{
    nombre: String,
    sabiduria: i16,
    fuerza: f32,
    carisma: u16
}

/*
Según lo que entiendo y quizás me pueda equivocar, básicamente para crear constructores y métodos
de una estructura y se hace con impl
*/
impl Personaje {
//Este tipo de funciones se les conoce como asociadas porque no tienen self como primer parámetro
    fn nuevo_nombrado(nombre1: String) -> Personaje {
        Personaje {
            fuerza: 35.2,
            nombre: nombre1,
            sabiduria: -100,
            carisma: 200
        }
    }

//
    fn get_fuerza(&self) -> f32 {
        self.fuerza
    }
}

//Además existen los arrays, las tuplas, los vectores y los hashmaps
fn prueba_vec_arr_tup_hash() {
    println!("-------");
    /*
    Primero aprendemos a declarar arrays, al igual que en c, uno pone tipo y número de datos, pero
    no es forzoso, pues el sistema puede inferirlos.
    */
    let nuevo_array: [u16;6] = [1,1,2,3,5,8];
    for i in nuevo_array.iter() {
        println!("{}", i);
    }
    /*
        Después checamos las tuplas, qke se parecen a los arrays, pero no es forzoso que todos los
        datos sean del mismo tipo, se indica el valor de cada tipo de dato, la extensión de la
        declaración muestra la extensión de la tupla. AL igual que el array, el compilador puede
        inferir los tipos.
    */
    println!("-------");
    let nueva_tupla: (u8, &str) = (6, "Cadena");
    println!("{:?}", nueva_tupla);

    /*
        Seguimos con los vectores, que al igual que los vectores en c++, uno puede agregar n
        elementos del mismo tipo.
    */
    println!("-------");
    let mut nuevo_vector: Vec<u8> = Vec::new();
    nuevo_vector.push(1);
    nuevo_vector.push(1);
    nuevo_vector.push(2);
    for i in nuevo_vector.iter() {
        println!("{}", i );
    }
    /*
        Ahora el hashmap que es un elemento de llave-valor. A diferencia de los otros, se necesita
        traer el módulo para usarlo.
    */
    println!("-------");
    let mut nuevo_hash = HashMap::new();
    nuevo_hash.insert("saludo","Hola HashMap!".to_string());
    println!("{:?}", nuevo_hash.get("saludo"));
}
