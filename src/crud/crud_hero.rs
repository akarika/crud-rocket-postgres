use crate::models::hero::Hero;
use crate::db;
use crate::errors::CrudError;
use rocket_contrib::json::{Json, JsonValue};
use rocket_okapi::{openapi};


#[openapi]
#[post("/", data = "<hero>")]
pub fn create(hero: Json<Hero>, connection: db::Connection) -> Result<Json<Hero>, CrudError> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Hero::create(insert, &connection)
        .map(|value|Json(value))
        .map_err(|error|CrudError::DBError(error.to_string()))
}

#[openapi]
#[get("/")]
pub fn read(connection: db::Connection) -> Result<Json<Vec<Hero>>, CrudError> {
    Hero::read(&connection)
        .map(|value|Json(value))
        .map_err(|error|CrudError::DBError(error.to_string()))
}

#[openapi]
#[get("/<id>")]
pub fn show(id: i32, connection: db::Connection) -> Result<Json<Hero>, CrudError> {
    Hero::show(id, &connection)
        .map(|value|Json(value))
        .map_err(|error|CrudError::DBError(error.to_string()))

}

#[openapi]
#[put("/<id>", data = "<hero>")]
pub fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> Result<Json<usize>, CrudError> {
    let update = Hero { id: Some(id), ..hero.into_inner() };
    Hero::update(id, update, &connection)
        .map(|value|Json(value))
        .map_err(|error|CrudError::DBError(error.to_string()))
}

#[openapi]
#[delete("/<id>")]
pub fn delete(id: i32, connection: db::Connection) -> Result<Json<usize>, CrudError> {
     Hero::delete(id, &connection)
         .map(|value|Json(value))
         .map_err(|error|CrudError::DBError(error.to_string()))
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
