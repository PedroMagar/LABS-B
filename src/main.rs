#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

use rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket::http::Method;
use rocket::{get, post, put, delete, routes};
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
    "LABS Backend"
}

#[get("/", format = "json")]
fn amostra_ler(connection: db::Connection) -> Json<JsonValue> {
  Json(json!(Amostra::read(&connection)))
}

#[post("/", format = "json", data = "<amostra>")]
fn amostra_add(amostra: Json<Amostra>, connection: db::Connection) -> Json<JsonValue> {
    let insert = Amostra { id: None, ..amostra.into_inner() };
    Json(json!(Amostra::create(insert, &connection)))
}

#[put("/<id>", data = "<amostra>")]
fn amostra_update(id: i32, amostra: Json<Amostra>, connection: db::Connection) -> Json<JsonValue> {
    let update = Amostra { id: Some(id), ..amostra.into_inner() };
    Json(json!({
        "success": Amostra::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn amostra_delete(id: i32, connection: db::Connection) -> Json<JsonValue> {
  Json(json!({
      "success": Amostra::delete(id, &connection)
  }))
}

fn main() -> Result<(), Error> {
  // let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4200"]);
  let allowed_origins = AllowedOrigins::all();

///////////////////////////////////////////////////////////////////////////////
//                                   CORS                                    //
//                      You can also deserialize this                        //
/////////////////////////////////////////////////////////////////////////////// 
  let cors = rocket_cors::CorsOptions {                                      //
      allowed_origins,                                                       //
      allowed_methods: vec![Method::Get,Method::Post,Method::Put,Method::Delete].into_iter().map(From::from).collect(),
      // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Access-Control-Allow-Origin"]),
      allowed_headers: AllowedHeaders::all(),                                //
      allow_credentials: true,                                               //
      ..Default::default()                                                   //
  }.to_cors()?;                                                              //
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
//                            Launching ROCKET                               //
///////////////////////////////////////////////////////////////////////////////
  rocket::ignite()                                                           //
    .mount("/", routes![index])                                              //
    .mount("/amostra/read", routes![amostra_ler])                            //
    .mount("/amostra/add", routes![amostra_add])                             //
    .mount("/amostra/update", routes![amostra_update])                       //
    .mount("/amostra/delete", routes![amostra_delete])                       //
    .manage(db::connect())                                                   //
    .attach(cors)                                                            //
    .launch();                                                               //
///////////////////////////////////////////////////////////////////////////////

  Ok(())
}
