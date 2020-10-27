use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::usuarios;

//#[table_name = "usuarios"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Usuario {
    pub id: Option<i32>,
    pub usuario: String,
    pub senha: String,
    pub email: String,
    pub telefone: String,
}


impl Usuario {
    pub fn create(usuario: Usuario, connection: &MysqlConnection) -> Usuario {
        diesel::insert_into(usuarios::table)
            .values(&usuario)
            .execute(connection)
            .expect("Error creating new User");

        usuarios::table.order(usuarios::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Usuario> {
        usuarios::table.order(usuarios::id.asc()).load::<Usuario>(connection).unwrap()
    }

    pub fn update(id: i32, usuario: Usuario, connection: &MysqlConnection) -> bool {
        diesel::update(usuarios::table.find(id)).set(&usuario).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(usuarios::table.find(id)).execute(connection).is_ok()
    }
}