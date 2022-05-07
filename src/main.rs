mod api;
mod os;
mod translations;
mod web_interface;

use crate::web_interface::render;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};

const SELF_IP: &str = "192.168.4.1";

#[get("/")]
pub async fn web_interface_fn( data: web::Data<api::AppState>) -> impl Responder {
    
    let mut t = data.translations.lock().unwrap();
    let lang = data.lang.lock().unwrap();
    let translator = t.get_or_def(&lang, &translations::DEF_LANG);

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(render::setup_form(translator, &lang))
}

pub async fn captive_portal() -> impl Responder {
    HttpResponse::Found()
    .append_header(("Location", format!("http://{}", SELF_IP)))
    .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(api::AppState::new()))
            .service(web_interface_fn)
            .service(api::scope())
            .default_service(web::to(captive_portal))
    })
    .bind(("0.0.0.0", 8080))?
    //.bind(("0.0.0.0", 80))?
    .workers(1) // We only expect one client to be present
    .run()
    .await
}