use crate::models::User;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
  fn user(_id: String) -> FieldResult<User> {
    // TODO: Database lookup
    Ok(User {
      id: "1234".to_owned(),
      name: "Brenda Galoc".to_owned(),
      ucid: "60010001".to_owned(),
      email: "brenda@email.com".to_owned(),
      password: "password".to_owned(),
      points_id: "1".to_owned(),
    })
  }

  fn users() -> Vec<User> {
    vec![]
  }
}
