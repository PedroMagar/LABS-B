use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::amostras;

//#[table_name = "amostras"]
#[derive(Serialize, Deserialize, Queryable)]
pub struct Amostra {
    pub id: Option<i32>,
    pub nome: String,
    pub matriz: String,
    pub dopante: String,
    pub autor: String,
    pub local: String
}


impl Amostra {
    /*pub fn create(hero: Hero, connection: &MysqlConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero");

        heroes::table.order(heroes::id.desc()).first(connection).unwrap()
    }*/

    pub fn read(connection: &MysqlConnection) -> Vec<Amostra> {
        amostras::table.order(amostras::id.asc()).load::<Amostra>(connection).unwrap()
    }

    /*pub fn update(id: i32, hero: Hero, connection: &MysqlConnection) -> bool {
        diesel::update(heroes::table.find(id)).set(&hero).execute(connection).is_ok()
    }*/

    /*pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
    }*/
}