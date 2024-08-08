use actix_web::{web, App, HttpServer};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use std::sync::Mutex;
use core::blockchain::Blockchain;
use graphql::schema::{MySchema, QueryRoot};
use common::config::{INITIAL_DIFFICULTY, TARGET_BLOCK_TIME, MAX_BLOCK_SIZE, DIFFICULTY_ADJUSTMENT_WINDOW, NETWORK_ID};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = web::Data::new(Mutex::new(Blockchain::new(
        "./db",
    )));

    // Create GraphQL schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(blockchain.clone())
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn graphql_handler(schema: web::Data<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
