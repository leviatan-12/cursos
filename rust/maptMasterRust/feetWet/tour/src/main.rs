fn main() {
    prueba_if();
    prueba_ciclos();
    prueba_struct();
}

/*
Los if pueden regresar valores. Esto se puede indicar no poniendo un ; en la
última linea, siempre que el ; falte indica que se regresa un valor.
*/
fn prueba_if() {
    let valor = if 1 == 2 {
        "son iguales"
    } else {
        "no son iguales"
    };
    println!("el valor es: {}",valor);
}

/*
Hay 3 tipos de loop: loop, while y for, la diferencia entre el while y el loop es la necesidad de
que haya una condicional para quitar el loop. El for es como en python que recorre los elementos de
una lista.
*/
fn prueba_ciclos() {
    let mut valor = 10;
    loop {
        if valor < 0 {
            break;
        }
        println!("{}",valor);
        valor -= 1;
    }
    valor = 10;
    while valor >= 0 {
        println!("{}",valor);
        valor -= 1;
    }

    for i in 1..10 {
        println!("{}", i);
    }
}

/*
Para crear tipos de datos personalizados se utilizan los struct. Hay de dos tipos: los que, al puro
estilo array solo se accede a sus datos por posición (y se recomienda para los datos de uno o dos
valores), u otro en el que si se especifica cada valor y su tipo.
*/

//del primer tipo mencionado, también se conocen como tuple struct
struct Celsius(i64);

//del segundo tipo
struct Personaje {
    nombre: String,
    inteligencia: i8,
    fuerza: f32,
    carisma: u16
}

fn prueba_struct() {
    let temperatura = Celsius(10);
    //Para acceder a los datos de las tuple struct se hace a través de su posición: tipo.posicion
    println!("temperatura: {}",temperatura.0);

    let tipo = Personaje {
        nombre: "Fulano".to_string(),
        inteligencia: 10,
        fuerza: 6.8,
        carisma: 100
    };

    //Para acceder los datos es a través del identificador: tipo.id
    println!("{} tiene fuerza {}", tipo.nombre, tipo.fuerza);
}
