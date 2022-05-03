mod api;
mod translations;
mod web_interface;
mod wifi_handler;

use crate::web_interface::render;

use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;


#[derive(Deserialize)]
struct WebInterfaceParams {
    step: WebInterfaceStep,
}

#[derive(Deserialize)]
enum WebInterfaceStep {
    LanguageSelect,
    WifiConnect
}

#[get("/")]
async fn web_interface_fn(state: web::Query<WebInterfaceParams>, data: web::Data<api::AppState>) -> impl Responder {
    
    let mut t = data.translations.lock().unwrap();
    let translator = t.get(&data.lang.lock().unwrap());

    match state.step {
        WebInterfaceStep::LanguageSelect => render::language_select(translator),
        WebInterfaceStep::WifiConnect => render::wifi_connect(translator)
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(api::AppState::new()))
            .service(web_interface_fn)
            .service(api::scope())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1) // We only expect one client to be present
    .run()
    .await
}