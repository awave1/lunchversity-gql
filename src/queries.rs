use crate::models::User;
use crate::schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::FieldResult;

pub struct QueryRoot;

fn establish_connection() -> PgConnection {
  dotenv().ok();
  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

#[juniper::graphql_object]
impl QueryRoot {
  fn users() -> Vec<User> {
    use crate::schema::users::dsl::users as users_dsl;

    let connection = establish_connection();
    users_dsl
      .limit(100)
      .load::<User>(&connection)
      .expect("Error loading members")
  }
}
