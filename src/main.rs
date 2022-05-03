mod api;
mod wifi_handler;

use actix_web::{get, web, App, HttpServer, Responder};
use fluent::{FluentBundle, FluentResource};
use serde::Deserialize;

#[derive(Deserialize)]
struct WebInterfaceParams {
    step: WebInterfaceStep
}

#[derive(Deserialize)]
enum WebInterfaceStep {
    LanguageSelect,
    WifiConnect
}

#[get("/")]
async fn web_interface(state: web::Query<WebInterfaceParams>) -> impl Responder {
    match state.step {
        WebInterfaceStep::LanguageSelect => {"Select Lang".to_string()},
        WebInterfaceStep::WifiConnect => {"Connect to wifi".to_string()}
    }
    
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(api::AppState::new()))
            .service(web_interface)
            .service(api::scope())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1) // We only expect one client to be present
    .run()
    .await
}