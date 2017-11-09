#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate rocket;
extern crate rocket_contrib;

use rand::Rng;
use rocket::request::Form;
use rocket_contrib::{Template, Json};

const FRASES: &[&'static str] = &[
    "La fuerza estará contigo siempre",
    "Frase 1",
    "Frase 2",
    "Frase 3",
    "Frase 4",
];

fn frase_aleatoria(frases : &[&'static str]) -> &'static str {
    let mut generador_aleatorio = rand::thread_rng();
    let indice = generador_aleatorio.gen_range(0, FRASES.len());

    frases.get(indice).expect("No se pudo obtener una frase")
}

#[derive(Serialize)]
struct ContextoPlantilla{
    frase: String,
    nombre: Option<String>,
}

#[get("/")]
fn index() -> Template {
    let contexto = ContextoPlantilla{
        frase: frase_aleatoria(&FRASES).to_string(),
        nombre: None,
    };
    return Template::render("index",&contexto);
}

#[derive(FromForm)]
struct Formulario {
    nombre: String,
}

#[post("/", data="<formulario>")]
fn index_personalizado(formulario: Form<Formulario>) -> Template {
    let form = formulario.get();
    let nombre = form.nombre.clone();
    let contexto = ContextoPlantilla {
        frase: frase_aleatoria(&FRASES).to_string(),
        nombre: Some(nombre),
    };
    return Template::render("index",&contexto);
}


#[get("/frases")]
fn api_frases() ->Json<&'static [&'static str]> {
    Json(FRASES)
}

fn main() {
    rocket::ignite()
    .mount("/",routes![index,index_personalizado])
    .mount("/api/v1",routes![api_frases])
    .attach(Template::fairing())
    .launch();
}
