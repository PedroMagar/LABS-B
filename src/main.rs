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
    .mount("/amostra/add", routes![amostra_add])
    .manage(db::connect())
    .attach(cors)
    .launch();

    Ok(())
}