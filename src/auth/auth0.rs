use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::users;

#[derive(Serialize, Deserialize)]
struct Auth0JWTPayload {
    email: String,
    user_id: String,
    exp: i64,
    iss: String,
    aud: String,
}

impl Auth0JWTPayload {
    /// Creates a Auth0JWTPayload from a subset of fields returned as json
    /// from the /oauth/token endpoint.
    pub fn from_json(json: &Value) -> Result<Auth0JWTPayload, Error> {
        match (
            json.get("email"),
            json.get("user_id"),
            json.get("exp"),
            json.get("iss"),
            json.get("aud"),
        ) {
            (Some(email), Some(user_id), Some(exp_str), Some(iss), Some(aud)) => {
                Ok(Auth0JWTPayload {
                    email: email.as_str().unwrap().to_string(),
                    user_id: user_id.as_str().unwrap().to_string(),
                    exp: exp_str.as_i64().unwrap(),
                    iss: iss.as_str().unwrap().to_string(),
                    aud: aud.as_str().unwrap().to_string(),
                })
            }
            _ => Err(AuthError::MalformedJWT {
                repr: format!("{:?}", json.clone()),
            })?,
        }
    }
}
