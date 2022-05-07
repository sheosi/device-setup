mod api;
mod os;
mod translations;
mod web_interface;

use crate::web_interface::render;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, http::header::ContentType};


#[get("/")]
async fn web_interface_fn( data: web::Data<api::AppState>) -> impl Responder {
    
    let mut t = data.translations.lock().unwrap();
    let translator = t.get(&data.lang.lock().unwrap());

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(render::setup_form(translator, &data.lang.lock().unwrap()))
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