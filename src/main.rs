mod api;
mod os;
mod translations;
mod vars;
mod web_interface;

use crate::web_interface::render;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};

#[get("/")]
pub async fn web_interface_fn(data: web::Data<api::AppState>) -> impl Responder {
    
    let translator = data.current.lock().unwrap();
    let lang = data.lang.lock().unwrap();

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(render::setup_form(&*translator, &lang))
}

pub async fn captive_portal() -> impl Responder {
    HttpResponse::Found()
    .append_header(("Location", "/"))
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