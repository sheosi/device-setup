mod http;
mod os;
mod vars;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    http::device_setup_server().await
}