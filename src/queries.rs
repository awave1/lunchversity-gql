use crate::models::{Points, User, Vendor};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::{FieldError, FieldResult};
use sha2::{Digest, Sha256};

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
      .expect("Error loading users")
  }

  fn get_user(email: String, password: String) -> FieldResult<User> {
    use crate::schema::users::dsl::users as users_dsl;
    let mut hasher = Sha256::new();
    hasher.update(password);
    let password_hash: String = format!("{:X}", hasher.finalize());

    let connection = establish_connection();
    let users = users_dsl
      .limit(100)
      .load::<User>(&connection)
      .expect("Error loading users");

    Ok(
      users
        .into_iter()
        .find(|u| u.email == email && u.password.to_lowercase() == password_hash.to_lowercase())
        .unwrap(),
    )
  }

  fn vendors() -> Vec<Vendor> {
    use crate::schema::vendors::dsl::vendors as vendors_dsl;
    let connection = establish_connection();
    vendors_dsl
      .limit(10)
      .load::<Vendor>(&connection)
      .expect("Error loading vendors")
  }

  fn points(user_id: i32, vendor_id: i32) -> Points {
    use crate::schema::points::dsl::points as points_dsl;
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
