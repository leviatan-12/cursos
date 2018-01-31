#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

/*#[get("/")]
fn inicio() -> &'static str {
    "Hola mundo"
}
*/
#[get("/<name>")]
fn inicio(name: String) -> String {
    format!("Hola, {}", name)
}


fn main() {
    rocket::ignite().mount("/", routes![inicio]).launch();
}
