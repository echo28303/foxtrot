use actix_web::{web, App, HttpServer, guard};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, WSSubscription};
use std::sync::Mutex;
use core::blockchain::Blockchain;
use graphql::schema::{MySchema, QueryRoot, SubscriptionRoot};
use common::config::{INITIAL_DIFFICULTY, TARGET_BLOCK_TIME, MAX_BLOCK_SIZE, NETWORK_ID};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = web::Data::new(Mutex::new(Blockchain::new(
        "./db",
    )));

    // Create GraphQL schema with subscription support
    let schema = Schema::build(QueryRoot, async_graphql::EmptyMutation, SubscriptionRoot)
        .data(blockchain.clone())
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
            .service(
                web::resource("/ws")
                    .guard(guard::Get())
                    .to(ws_handler)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn graphql_handler(schema: web::Data<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn ws_handler(schema: web::Data<MySchema>, req: actix_web::HttpRequest, payload: actix_web::web::Payload) -> Result<actix_web::HttpResponse, actix_web::Error> {
    WSSubscription::start(Schema::clone(&*schema), &req, payload)
}
