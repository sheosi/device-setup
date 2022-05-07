mod api;
mod os;
mod translations;
mod web_interface;

use crate::web_interface::render;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};

const ROUTES: [&str; 1] = [
    "/"
];

pub async fn web_interface_fn( data: web::Data<api::AppState>) -> impl Responder {
    
    let mut t = data.translations.lock().unwrap();
    let lang = data.lang.lock().unwrap();
    let translator = t.get_or_def(&lang, &translations::DEF_LANG);

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(render::setup_form(translator, &data.lang.lock().unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let a = App::new()
            .app_data(web::Data::new(api::AppState::new()))
            .service(api::scope());

        a.route("/", web::get().to(web_interface_fn))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1) // We only expect one client to be present
    .run()
    .await
}