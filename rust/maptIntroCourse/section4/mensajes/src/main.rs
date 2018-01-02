use std::sync::mpsc;
use std::sync::mpsc::Receiver;

type TipoToken = i32;

struct Msg {
    tipo: TipoToken,
    val: String,
}

fn hacer_canal() -> Receiver<Msg> {
    let (emisor,receptor) = mpsc::sync_channel(1);
    let nuevo_mensaje = Msg {
            tipo: 42,
            val: "Esto es un mensaje de prueba".to_string(),
    };
    let _ = emisor.send(nuevo_mensaje);
    return receptor;
}

fn main() {
    let mensaje_recibido = hacer_canal();
    if let Some(msg) = mensaje_recibido.recv().ok() {
        println!("Mensaje recibido", );
        println!("Tipo de mensaje: {}", msg.tipo);
        println!("Contenido:", );
        println!("{}", msg.val);
    }
}
