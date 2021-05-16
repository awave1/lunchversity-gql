use crate::models::{Points, User, Vendor};
use crate::schema::points::dsl::points as points_dsl;
use crate::schema::users::dsl::users as users_dsl;
use crate::schema::vendors::dsl::vendors as vendors_dsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::{FieldError, FieldResult};

pub struct QueryRoot;

fn establish_connection() -> PgConnection {
  dotenv().ok();
  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

#[juniper::graphql_object]
impl QueryRoot {
  fn users() -> Vec<User> {
    let connection = establish_connection();
    users_dsl
      .limit(100)
      .load::<User>(&connection)
      .expect("Error loading users")
  }

  fn vendors() -> Vec<Vendor> {
    let connection = establish_connection();
    vendors_dsl
      .limit(10)
      .load::<Vendor>(&connection)
      .expect("Error loading vendors")
  }

  fn points(user_id: i32, vendor_id: i32) -> Points {
    let connection = establish_connection();
    points_dsl
      .limit(100)
      .load::<Points>(&connection)
      .expect("Error loading points")
      .into_iter()
      .find(|item| item.user_id == user_id && item.vendor_id == vendor_id)
      .unwrap()
  }
}
