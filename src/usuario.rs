use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::dsl::{delete, insert_into};
use super::Pool;
use crate::schema::usuarios;
use serde::{Deserialize, Serialize};
use actix_web::{web, Error};
use super::schema::usuarios::dsl::*;

#[table_name = "usuarios"]//Serialize, Deserialize, 
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
pub struct Usuario {
    pub id: Option<i32>,
    pub usuario: String,
    pub senha: String,
    pub email: String,
    pub telefone: String,
    pub cargo: i32,
}

/*
#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,  <-------------------------------------- IMPORTANTE
}*/

impl Usuario {
    pub fn create(db: web::Data<Pool>, item: web::Json<Usuario>) -> Result<Usuario, diesel::result::Error> {
        let conn = db.get().unwrap();
        let new_usuario = Usuario { id: None, ..item.into_inner() };
        diesel::insert_into(usuarios::table)
            .values(&new_usuario)
            .execute(&conn)
            .expect("Error creating new usuario");
        let res = usuarios::table.order(usuarios::id.desc()).first(&conn).unwrap();
        Ok(res)
    }

    pub fn read(pool: web::Data<Pool>, user_id: i32) -> Result<Usuario, diesel::result::Error> {
        let conn = pool.get().unwrap();
        usuarios.find(user_id).get_result::<Usuario>(&conn)
    }

    pub fn read_all(pool: web::Data<Pool>) -> Result<Vec<Usuario>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let items = usuarios.load::<Usuario>(&conn)?;
        Ok(items)
    }

    pub fn update(
        db: web::Data<Pool>,
        item: web::Json<Usuario>,
    ) -> Result<Usuario, diesel::result::Error> {
        let conn = db.get().unwrap();
        let new_usuario = Usuario { ..item.into_inner() };
        diesel::update(usuarios::table.find(new_usuario.id))
            .set(&new_usuario)
            .execute(&conn)
            .expect("Error creating new usuario");
        let res = usuarios::table.first(&conn).unwrap();
        Ok(res)
    }

    pub fn delete(db: web::Data<Pool>, usuario_id: i32) -> Result<usize, diesel::result::Error> {
        let conn = db.get().unwrap();
        let count = delete(usuarios.find(usuario_id)).execute(&conn)?;
        Ok(count)
    }
}