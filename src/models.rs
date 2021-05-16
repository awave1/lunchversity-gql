use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A Type of user we have on the platform")]
pub struct User {
  pub id: String,
  pub name: String,
  pub email: String,
  pub password: String,
  pub ucid: String,
  pub points_id: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "Vendor object/potentially can be a user type")]
pub struct Vendor {
  pub id: String,
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone: String,
}
