#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate juniper;

use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod gql;
mod models;
mod queries;
mod schema;

use crate::gql::{create_schema, Schema};

// const IP: &str = "127.0.0.1:8080";
const IP: &str = "172.27.176.158:8080";

/// GraphiQL endpoint
async fn graphiql() -> HttpResponse {
    let endpoint: String = format!("http://{}/graphql", IP);
    let html = graphiql_source(endpoint.as_str(), None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

/// GraphQL endpoint
async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute_sync(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::permissive()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(format!("{}", IP))?
    .run()
    .await
}
