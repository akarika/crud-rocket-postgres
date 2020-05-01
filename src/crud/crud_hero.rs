#![feature(plugin)]


use crate::models::hero::Hero;
use rocket_contrib::json::{Json, JsonValue};
use crate::db;
use rocket_okapi::{openapi, JsonSchema};
use diesel::result::Error;
use rocket::http::Status;
use crate::errors::CrudError;
#[derive(serde::Serialize, JsonSchema)]
pub struct ResponseRequest {
    reply: bool,
}


#[openapi]
#[post("/", data = "<hero>")]
pub fn create(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Json(Hero::create(insert, &connection))
}

#[openapi]
#[get("/")]
pub fn read(connection: db::Connection) -> Result<Json<Vec<Hero>>, Json<String>> {
    Hero::read(&connection)
        .map(|value|Json(value))
        .map_err(|error|Json(CrudError::DBError(error).to_string()))
}
#[openapi]
#[get("/<id>")]
pub fn show(id: i32, connection: db::Connection) -> Result<Json<Hero>, Json<String>> {
    Hero::show(id, &connection)
        .map(|value|Json(value))
        .map_err(|error|Json(CrudError::DBError(error).to_string()))

}

#[openapi]
#[put("/<id>", data = "<hero>")]
pub fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> Json<ResponseRequest> {
    let update = Hero { id: Some(id), ..hero.into_inner() };
    Json(ResponseRequest {
        reply: Hero::update(id, update, &connection)
    })
}

#[openapi]
#[delete("/<id>")]
pub fn delete(id: i32, connection: db::Connection) -> Json<ResponseRequest> {
    Json(ResponseRequest {
        reply: Hero::delete(id, &connection)
    })
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
