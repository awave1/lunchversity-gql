use diesel::Queryable;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject, Queryable)]
#[graphql(description = "A Type of user we have on the platform")]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
  pub ucid: String,
}

#[derive(GraphQLObject, Queryable)]
#[graphql(description = "Vendor object/potentially can be a user type")]
pub struct Vendor {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone: String,
}

#[derive(GraphQLObject, Queryable)]
pub struct Points {
  pub id: i32,
  pub user_id: i32,
  pub vendor_id: i32,
  pub amount: i32,
}
