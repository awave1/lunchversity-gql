use juniper::FieldResult;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A Type of user we have on the platform")]
pub struct User {
  id: String,
  name: String,
  email: String,
  password: String,
  ucid: String,
  points_id: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "Vendor object/potentially can be a user type")]
pub struct Vendor {
  id: String,
  name: String,
  email: String,
  password: String,
  phone: String,
}

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
}

// TODO: mutations
// pub struct MutationRoot;
// #[juniper::graphql_object]
// impl MutationRoot {
//   fn create_human(new_human: NewHuman) -> FieldResult<Human> {
//     Ok(Human {
//       id: "1234".to_owned(),
//       name: new_human.name,
//       appears_in: new_human.appears_in,
//       home_planet: new_human.home_planet,
//     })
//   }
// }

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
