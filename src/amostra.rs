use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::dsl::{delete, insert_into};
use super::Pool;
use crate::schema::amostras;
use serde::{Deserialize, Serialize};
use actix_web::{web, Error};
use super::schema::amostras::dsl::*;

#[table_name = "amostras"]//Serialize, Deserialize, 
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
pub struct Amostra {
    pub id: Option<i32>,
    pub nome: String,
    pub matriz: String,
    pub dopante: String,
    pub autor: String,
    pub local: String
}

#[derive(Insertable, Debug)]
#[table_name = "amostras"]
pub struct NewAmostra<'a> {
    pub nome: &'a str,
    pub matriz: &'a str,
    pub dopante: &'a str,
    pub autor: &'a str,
    pub local: &'a str,
}


impl Amostra {
    /*pub fn create(amostra: Amostra, connection: &MysqlConnection) -> Amostra {
        diesel::insert_into(amostras::table)
            .values(&amostra)
            .execute(connection)
            .expect("Error creating new amostra");

        amostras::table.order(amostras::id.desc()).first(connection).unwrap()
    }*/
    pub fn create(db: web::Data<Pool>, item: web::Json<Amostra>) -> Result<Amostra, diesel::result::Error> {
        let conn = db.get().unwrap();
        let new_amostra = Amostra { id: None, ..item.into_inner() };
        diesel::insert_into(amostras::table)
            .values(&new_amostra)
            .execute(&conn)
            .expect("Error creating new amostra");
        let res = amostras::table.order(amostras::id.desc()).first(&conn).unwrap();
        Ok(res)
    }

    /*pub fn read(connection: &MysqlConnection) -> Vec<Amostra> {
        amostras::table.order(amostras::id.asc()).load::<Amostra>(connection).unwrap()
    }*/
    pub fn read(pool: web::Data<Pool>, user_id: i32) -> Result<Amostra, diesel::result::Error> {
        let conn = pool.get().unwrap();
        amostras.find(user_id).get_result::<Amostra>(&conn)
    }

    pub fn read_all(pool: web::Data<Pool>) -> Result<Vec<Amostra>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let items = amostras.load::<Amostra>(&conn)?;
        Ok(items)
    }

    /*pub fn update(amostra: Amostra, connection: &MysqlConnection) -> Amostra {
        diesel::update(amostras::table.find(amostra.id))
            .set(&amostra)
            .execute(connection)
            .expect("Error creating new amostra");
        amostras::table.first(connection).unwrap()
    }*/
    pub fn update(
        db: web::Data<Pool>,
        item: web::Json<Amostra>,
    ) -> Result<Amostra, diesel::result::Error> {
        let conn = db.get().unwrap();
        let new_amostra = Amostra { ..item.into_inner() };
        diesel::update(amostras::table.find(new_amostra.id))
            .set(&new_amostra)
            .execute(&conn)
            .expect("Error creating new amostra");
        let res = amostras::table.first(&conn).unwrap();
        Ok(res)
    }

    /*pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(amostras::table.find(id)).execute(connection).is_ok()
    }*/
    pub fn delete(db: web::Data<Pool>, amostra_id: i32) -> Result<usize, diesel::result::Error> {
        let conn = db.get().unwrap();
        let count = delete(amostras.find(amostra_id)).execute(&conn)?;
        Ok(count)
    }
}