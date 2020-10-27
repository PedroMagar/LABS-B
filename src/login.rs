use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::logins;

//#[table_name = "login"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Login {
    pub id: Option<i32>,
    pub password: String,
    pub email: String
}


impl Login {
    pub fn create(login: Login, connection: &MysqlConnection) -> Login {
        diesel::insert_into(logins::table)
            .values(&login)
            .execute(connection)
            .expect("Error creating new Login");

        logins::table.order(logins::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Login> {
        logins::table.order(logins::id.asc()).load::<Login>(connection).unwrap()
    }

    pub fn update(id: i32, login: Login, connection: &MysqlConnection) -> bool {
        diesel::update(logins::table.find(id)).set(&login).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(logins::table.find(id)).execute(connection).is_ok()
    }
}