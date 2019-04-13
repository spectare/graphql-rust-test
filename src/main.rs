use std::io;

#[macro_use]
extern crate juniper;
extern crate futures;
extern crate actix_service;

mod schema;
mod handlers;

use actix_web::{middleware, web, App, HttpServer};

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
            .service(web::resource("/graphql").route(web::post().to_async(handlers::graphql)))
            .service(web::resource("/graphiql").route(web::get().to(handlers::graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}