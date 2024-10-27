mod crawlers;
mod models;
mod routes;
mod utils;
use actix_web::{
    get, middleware::Logger, web, web::Data, App, HttpResponse, HttpServer, Responder,
};
use crawlers::starknet_crawler::crawl_starknet;
use routes::health_route::health_check;
use routes::user_route::{
    add_wallets, create_user, delete_wallets, get_profile, get_wallets, update_wallets,
};
use tokio::time::{interval, Duration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    tokio::spawn(crawl_starknet(60));

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(health_check)
            .service(create_user)
            .service(add_wallets)
            .service(delete_wallets)
            .service(get_profile)
            .service(get_wallets)
            .service(update_wallets)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
