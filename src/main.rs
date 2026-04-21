use actix_web::{get, web, App, HttpServer, Responder};
use config::Config;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
}

impl AppConfig {
    fn load() -> Self {
        Config::builder()
            .set_default("server.host", "0.0.0.0")
            .expect("Failed to set default host")
            .set_default("server.port", 80)
            .expect("Failed to set default port")
            .add_source(config::File::with_name("config").required(false))
            .build()
            .expect("Failed to build config")
            .try_deserialize()
            .expect("Failed to deserialize config")
    }
}

#[get("/hello")]
async fn hello() -> impl Responder {
    web::Json(json!({"hello": "world"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = AppConfig::load();
    let host = cfg.server.host;
    let port = cfg.server.port;
    println!("Starting server on {host}:{port}");
    HttpServer::new(|| App::new().service(hello))
        .bind((host.as_str(), port))?
        .run()
        .await
}
