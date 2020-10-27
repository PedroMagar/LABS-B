use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use actix_web::{http, HttpRequest, HttpResponse};
use actix_web::{http::header, middleware::Logger};
use actix_cors::Cors;

#[macro_use] extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
//extern crate r2d2;
//extern crate r2d2_diesel;



// module declaration here
//mod errors;
mod handlers;
//mod models;
mod schema;
mod db;
mod amostra;
mod usuario;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {                                                           //
        App::new()                                                                      //
            .wrap(                                                                      // Adding CORS policy
                Cors::default()                                                         //
                    .allowed_origin("http://localhost:4200")                            //
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])              //
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])       //
                    .allowed_header(header::CONTENT_TYPE)                               //
                    .supports_credentials()                                             //
                    .max_age(3600),                                                     //
            )                                                                           //
            // .wrap(Logger::default())
            .data(pool.clone())                                                         // Enables the handler functions to interact with the database independently
            .route("/usuarios/read", web::get().to(handlers::get_users))                // 
            .route("/usuarios/read/{id}", web::get().to(handlers::get_user))            // 
            .route("/usuarios/add", web::post().to(handlers::add_user))                 // 
            .route("/usuarios/update/{id}", web::put().to(handlers::update_user))       // 
            .route("/usuarios/delete/{id}", web::delete().to(handlers::delete_user))    // 
            .route("/amostras/read", web::get().to(handlers::get_amostras))             // 
            .route("/amostras/read/{id}", web::get().to(handlers::get_amostra))         // 
            .route("/amostras/add", web::post().to(handlers::add_amostra))              // 
            .route("/amostras/update/{id}", web::put().to(handlers::update_amostra))    // 
            .route("/amostras/delete/{id}", web::delete().to(handlers::delete_amostra)) // 
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
