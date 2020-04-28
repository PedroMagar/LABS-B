#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

use rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket::http::Method;
use rocket::{get, routes};
use rocket_cors;

extern crate r2d2;
extern crate r2d2_diesel;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod db;
mod schema;
mod amostra;

use amostra::{Amostra};

use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};


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


fn main() -> Result<(), Error> {
  let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4200"]);


  // You can also deserialize this
  let cors = rocket_cors::CorsOptions {
      allowed_origins,
      allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
      allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
      allow_credentials: true,
      ..Default::default()
  }.to_cors()?;

  rocket::ignite()
    .mount("/", routes![index])
    .mount("/hello", routes![hello])
    .mount("/amostra", routes![amostra])
    .mount("/amostra/ler", routes![amostra_ler])
    .manage(db::connect())
    .attach(cors)
    .launch();

    Ok(())
}