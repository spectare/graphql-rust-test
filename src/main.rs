use std::io;

#[macro_use]
extern crate juniper;
extern crate futures;
extern crate actix_service;

mod schema;
mod handlers;

use actix_web::middleware::cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};

use schema::create_schema;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            // .wrap(
            // Cors::new() // <- Construct CORS middleware builder
            //    .allowed_origin("*")
            //    .send_wildcard()
            //    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            //    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            //    .allowed_header(http::header::CONTENT_TYPE)
            //    .max_age(3600))
            .service(web::resource("/graphql").route(web::post().to_async(handlers::graphql)))
            .service(web::resource("/graphiql").route(web::get().to(handlers::graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}