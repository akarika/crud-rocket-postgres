use diesel::{QueryDsl, ExpressionMethods};
use diesel::{PgConnection, RunQueryDsl};
use crate::schema::heroes::dsl::*;
use crate::schema::heroes;
use rocket_okapi::{JsonSchema};
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, JsonSchema)]
#[table_name = "heroes"]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero {
    pub fn create(hero: Hero, connection: &PgConnection) -> Hero {
        diesel::insert_into(heroes)
            .values(&hero)
            .get_result(connection)
            .expect("Error creating new hero")
        //heroes::table.order(heroes::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &PgConnection) -> Vec<Hero> {
        let results = heroes
            .limit(5)
            .load::<Hero>(connection)
            .expect("Error loading Hero");
        results
        //  heroes.
        //heroes::table.order(heroes::id.asc()).load::<Hero>(connection).unwrap()
    }

    pub fn update(i: i32, hero: Hero, connection: &PgConnection) -> bool {
        diesel::update(heroes.find(i))
            .set(hero)
            .execute(connection).is_ok()

        // diesel::update(heroes::table.find(id)).set(&hero).execute(connection).is_ok()
    }

    pub fn delete(i: i32, connection: &PgConnection) -> bool {
        diesel::delete(heroes.find(i)).execute(connection).is_ok()
        // diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
    }
}