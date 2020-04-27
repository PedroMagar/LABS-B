#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

extern crate r2d2;
extern crate r2d2_diesel;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod db;
mod schema;
mod amostra;

use amostra::{Amostra};

#[get("/")]
fn index() -> &'static str {
    "LABS, backend!"
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}! Wellcome to LABS Backend!", age, name)
}

#[get("/")]
fn amostra() -> &'static str {
    "data = [{
        id: 1,
        Nome: 'TZ Eu Ag',
        Matriz: 'TZO',
        Dopante: 'Eu Ag',
        Autor: 'Augusto',
        Local: 'Gaveta',
      }, {
        id: 2,
        Nome: 'TZ Eu',
        Matriz: 'TZO',
        Dopante: 'Eu',
        Autor: 'Augusto',
        Local: 'Gaveta',
      }, {
        id: 3,
        Nome: 'TZN Eu',
        Matriz: 'TZNO',
        Dopante: 'Eu',
        Autor: 'Augusto',
        Local: 'Gaveta',
      }, {
        id: 4,
        Nome: 'TZ Eu Ag',
        Matriz: 'TZO',
        Dopante: 'Eu Ag',
        Autor: 'Augusto',
        Local: 'Gaveta',
      }, {
        id: 5,
        Nome: 'GP Eu',
        Matriz: 'GPO',
        Dopante: 'Eu',
        Autor: 'Augusto',
        Local: 'Gaveta',
      }, {
        id: 6,
        Nome: 'GP Eu Ag',
        Matriz: 'GPO',
        Dopante: 'Eu Ag',
        Autor: 'Augusto',
        Local: 'Gaveta',
      }, {
        id: 7,
        Nome: 'GP Eu Au',
        Matriz: 'GPO',
        Dopante: 'Eu Au',
        Autor: 'Augusto',
        Local: 'Gaveta',
      }, {
        id: 8,
        Nome: 'TZA Eu',
        Matriz: 'TZAO',
        Dopante: 'Eu',
        Autor: 'Augusto',
        Local: 'Gaveta',
      },
    
    ];"
}



#[get("/", format = "json")]
fn amostra_ler(connection: db::Connection) -> Json<JsonValue> {
  Json(json!(Amostra::read(&connection)))
}
/*fn amostra_ler() -> Json<JsonValue> {
  Json(json!([
      "amostra 1", 
      "amostra 2"
  ]))
}*/

fn main() {
    /*let am01 = Amostra {
        id: i32 = 1,
        Nome: String = "TZ Eu Ag",
        Matriz: String = "TZO",
        Dopante: String = "Eu Ag",
        Autor: String = "Augusto",
        Local: String = "Gaveta"
    };*/

    rocket::ignite()
    .mount("/", routes![index])
    .mount("/hello", routes![hello])
    .mount("/amostra", routes![amostra])
    .mount("/amostra/ler", routes![amostra_ler])
    .manage(db::connect())
    .launch();
}