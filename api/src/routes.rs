use crate::handlers;

use actix_web::web::{self, ServiceConfig};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/send_transaction")
            .route(web::post().to(handlers::send_transaction))
    )
    .service(
        web::resource("/get_account/{address}")
            .route(web::get().to(handlers::get_account))
    )
    .service(
        web::resource("/get_block/{index}")
            .route(web::get().to(handlers::get_block))
    )
    .service(
        web::resource("/get_transaction_fee")
            .route(web::post().to(handlers::get_transaction_fee))
    )
    .service(
        web::resource("/get_blockchain_info")
            .route(web::get().to(handlers::get_blockchain_info))
    )
    .service(
        web::resource("/get_pending_transactions")
            .route(web::get().to(handlers::get_pending_transactions))
    )
    .service(
        web::resource("/submit_block")
            .route(web::post().to(handlers::submit_block))
    )
    .service(
        web::resource("/validate_chain")
            .route(web::get().to(handlers::validate_chain))
    )
    .service(
        web::resource("/get_account_history/{address}")
            .route(web::get().to(handlers::get_account_history))
    );
}

