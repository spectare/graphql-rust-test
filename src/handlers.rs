use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::schema::Schema;

pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("Data: {:?}", data);
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    //use futures::IntoFuture;
    use actix_service::Service;
    use actix_web::{test, App, http::StatusCode};
    use bytes::Bytes;
    use crate::schema::create_schema;

    #[test]
    fn test_route_graphiql() {
        let mut app = test::init_service(
            App::new() 
                .service(web::resource("/").to(graphiql))
        );
        let req = test::TestRequest::with_header("CONTENT_TYPE", "text/plain").to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        println!("Response: {:?}", &resp)
    }

    #[test]
    fn test_route_graphql() {
        let schema = std::sync::Arc::new(create_schema());

        let mut app = test::init_service(
            App::new()
                .data(schema.clone()) 
                .service(web::resource("/graphql").route(web::post().to_async(graphql)))
        );
        //let obj = serde_json::from_slice::<MyObj>(&body)?;

        let req = test::TestRequest::post()
            .header("CONTENT_TYPE", "application/json")
            .uri("/graphql")
            .set_payload(Bytes::from_static(b"{\"query\": \"{hero{name}}\"}"))
            //.set_payload(r#"{"query": "{hero{name}}"}"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        println!("Response: {:?}", &resp);
        assert_eq!(resp.status(), StatusCode::OK);
    }

}