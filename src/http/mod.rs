mod api;
pub mod translations;
mod web_interface;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};


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