use crate::models::{User, Vendor};
use crate::queries::QueryRoot;
use juniper::FieldResult;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

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
