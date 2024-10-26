mod routes;
mod utils;
use actix_web::{
    get, middleware::Logger, web, web::Data, App, HttpResponse, HttpServer, Responder,
};
use routes::health_route::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new().wrap(logger).service(health_check)
        // .service(submit_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
