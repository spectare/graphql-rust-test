//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
use std::io;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;

use crate::schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_rt::main]
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
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App};
    use serde_json::json;
    use serde_json::Value;
    use std::str;

    #[actix_rt::test]
    async fn test_route_grahql() -> Result<(), Error> {
        std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();

        let schema = std::sync::Arc::new(create_schema());

        let graphql_query = json!({ "query": "{ human(id: \"1234\") { name } }" });
      
        let mut app = test::init_service(
          App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
        )
        .await;

        let req = test::TestRequest::post()

        .uri("/graphql")
            .set_json(&graphql_query)
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        let body_str = match str::from_utf8(&response_body) {
            Ok(v) => v,
            Err(_e) => "Error with parsing result from bytes to string",
        };

        let p: Value = serde_json::from_str(body_str).unwrap();
        println!("Value : {:?}", p);
        
        assert_eq!(p["data"]["human"]["name"], json!("Luke"));

        Ok(())
    }
}