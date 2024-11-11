mod crawlers;
mod models;
mod routes;
mod services;
mod utils;
use actix_cors::Cors;
use actix_web::{
    get, http, middleware::Logger, web, web::Data, App, HttpResponse, HttpServer, Responder,
};
use crawlers::starknet_crawler::crawl_starknet;

use routes::admin_routes::{
    create_network, delete_network, get_all_network, get_last_scanned_block,
    set_last_scanned_block, update_network_chain_id,
};
use routes::health_route::health_check;
use routes::user_route::{
    add_wallet, create_user, delete_wallet, get_all_users, get_all_users_via_network,
    get_all_wallets_via_network, get_profile, get_user_via_email, get_users_via_wallet,
    get_wallets, update_user_email, update_wallets,
};
use services::db::Database;
use tokio::time::{interval, Duration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    let db = Database::init().await;

    tokio::spawn(crawl_starknet(db.clone(), 60));
    let db_data = Data::new(db);
    HttpServer::new(move || {
        // let logger = Logger::default();
        App::new()
            .app_data(db_data.clone())
            // .wrap(logger)
            .wrap(
                Cors::default()
                    // .allowed_origin("http://localhost:3000") // Allow requests from your frontend
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        http::header::CONTENT_TYPE,
                        http::header::AUTHORIZATION,
                    ])
                    .supports_credentials(),
            )
            .service(health_check)
            .service(create_user)
            .service(add_wallet)
            .service(delete_wallet)
            .service(get_profile)
            .service(get_wallets)
            .service(update_wallets)
            .service(get_user_via_email)
            .service(get_all_users)
            .service(update_user_email)
            .service(create_network)
            .service(update_network_chain_id)
            .service(get_all_network)
            .service(delete_network)
            .service(get_last_scanned_block)
            .service(set_last_scanned_block)
            .service(get_all_wallets_via_network)
            .service(get_users_via_wallet)
            .service(get_all_users_via_network)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
