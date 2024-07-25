use actix_web::{middleware::Logger, HttpServer, App};
use std::env;

mod api;
mod service;
mod model;

use crate::api::apis::{root, date_formated};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(root)
            .service(date_formated)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}