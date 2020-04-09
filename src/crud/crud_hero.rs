#![feature(plugin)]


use crate::models::hero::Hero;
use rocket_contrib::json::{Json, JsonValue};
use crate::db;


#[post("/", data = "<hero>")]
pub fn create(hero: Json<Hero>, connection:  db::Connection) -> Json<Hero> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Json(Hero::create(insert, &connection))
}

#[get("/")]
pub fn read(connection: db::Connection) -> Json<JsonValue> {
    Json(json!(Hero::read(&connection)))
}

#[put("/<id>", data = "<hero>")]
pub fn update(id: i32, hero: Json<Hero>,connection: db::Connection) -> Json<JsonValue> {
    let update = Hero { id: Some(id), ..hero.into_inner() };
    Json(json!({
        "success": Hero::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
pub fn delete(id: i32,connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": Hero::delete(id, &connection)
    }))
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
