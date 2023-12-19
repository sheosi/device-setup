mod api;
pub mod translations;
mod web_interface;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};
use local_ip_address::local_ip;
use tracing::{event, Level};


#[get("/")]
async fn web_interface_fn(data: web::Data<api::AppState>) -> impl Responder {
    
    let translator = data.current.lock().unwrap();
    let lang = data.lang.lock().unwrap();

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(web_interface::render::setup_form(&translator, &lang))
}

async fn captive_portal() -> impl Responder {
    HttpResponse::Found()
    .append_header(("Location", "/"))
    .finish()
}

pub async fn device_setup_server() -> Result<(), std::io::Error>{
    const PORT: u16 = 8080;

    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let ip_addr = local_ip().unwrap();
    assert!(!ip_addr.is_loopback());
    let ip_addr_str = ip_addr.to_string();
    event!(Level::INFO, "Running on {ip_addr_str}");

    HttpServer::new(|| {
    App::new()
        .app_data(web::Data::new(api::AppState::new()))
        .service(web_interface_fn)
        .service(api::scope())
        .default_service(web::to(captive_portal))
    })
    .bind((ip_addr_str, PORT))?
    .workers(1) // We only expect one client to be present
    .run()
    .await
}