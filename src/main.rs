mod crawlers;
mod models;
mod routes;
mod services;
mod utils;
use actix_web::{
    get, middleware::Logger, web, web::Data, App, HttpResponse, HttpServer, Responder,
};
use crawlers::starknet_crawler::crawl_starknet;

use routes::health_route::health_check;
use routes::user_route::{
    add_wallet, create_user, delete_wallet, get_profile, get_user_via_email, get_wallets,
    update_wallets,
};
use services::db::Database;
use tokio::time::{interval, Duration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let db = Database::init().await;
    let db_data = Data::new(db);

    tokio::spawn(crawl_starknet(60));

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(db_data.clone())
            .wrap(logger)
            .service(health_check)
            .service(create_user)
            .service(add_wallet)
            .service(delete_wallet)
            .service(get_profile)
            .service(get_wallets)
            .service(update_wallets)
            .service(get_user_via_email)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
