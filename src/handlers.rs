//use actix_web::Responder;
use actix_web::{web, Error, HttpResponse};

use super::amostra;
use super::usuario;
//use super::schema::usuarios::dsl::*;
//use super::schema::amostras::dsl::*;
//use super::models::{NewUser, User};
//use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

use amostra::{Amostra};
use usuario::{Usuario};

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              AUTENTICAÇÃO                                               //
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/*pub async fn get_users() -> impl Responder {
    format!("hello from get users")
}

pub async fn get_user() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

pub async fn update_user() -> impl Responder {
    format!("hello from update user")
}

pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}*/

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Usuario::read_all(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Usuario::read(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<Usuario>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Usuario::create(db, item))
        .await
        .map(|usuario| HttpResponse::Created().json(usuario))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn update_user(
    db: web::Data<Pool>,
    item: web::Json<Usuario>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Usuario::update(db, item))
            .await
            .map(|usuario| HttpResponse::Ok().json(usuario))
            .map_err(|_| HttpResponse::InternalServerError())?
        )
}

pub async fn delete_user(
    db: web::Data<Pool>,
    usuario_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Usuario::delete(db, usuario_id.into_inner()))
            .await
            .map(|usuario| HttpResponse::Ok().json(usuario))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////
//                                                 AMOSTRA                                                 //
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub async fn get_amostras(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Amostra::read_all(db))
        .await
        .map(|amostra| HttpResponse::Ok().json(amostra))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_amostra(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Amostra::read(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn add_amostra(
    db: web::Data<Pool>,
    item: web::Json<Amostra>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Amostra::create(db, item))
        .await
        .map(|amostra| HttpResponse::Created().json(amostra))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn update_amostra(
    db: web::Data<Pool>,
    item: web::Json<Amostra>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Amostra::update(db, item))
            .await
            .map(|amostra| HttpResponse::Ok().json(amostra))
            .map_err(|_| HttpResponse::InternalServerError())?
        )
}

pub async fn delete_amostra(
    db: web::Data<Pool>,
    amostra_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Amostra::delete(db, amostra_id.into_inner()))
            .await
            .map(|amostra| HttpResponse::Ok().json(amostra))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}
