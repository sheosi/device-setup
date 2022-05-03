use std::sync::Mutex;

use crate::os::wifi_handler::{WifiHandler, get_wifi_handler};
use crate::os;
use crate::translations::Translations;

use actix_web::{post, web, Scope, Responder};
use serde::Deserialize;
use unic_langid::{langid, LanguageIdentifier};

pub struct AppState {
    pub wifi: Mutex<Box<dyn WifiHandler>>,
    pub lang: Mutex<LanguageIdentifier>,
    pub translations: Mutex<Translations>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            wifi: Mutex::new(get_wifi_handler()),
            lang: Mutex::new(langid!("en-US")),
            translations: Mutex::new(Translations::new())
        }
    }
}

pub fn scope() -> Scope {
    web::scope("/api")
}

#[derive(Deserialize)]
pub struct SetLangParams {
    lang: String
}


#[post("/api/setLang")]
async fn set_lang(data: web::Data<AppState>, params: web::Query<SetLangParams>) -> impl Responder {
    let lang = params.lang.parse().unwrap();
    os::set_locale(&lang);
    *data.lang.lock().unwrap() = lang;

    "This is the base".to_string()
}

#[derive(Deserialize)]
struct ConnectParams {
    ssid: String,
    password: String
}

#[post("/api/connectWifi")]
async fn connect_wifi(data: web::Data<AppState>, params: web::Query<ConnectParams>) -> impl Responder {
    let a = data.wifi.lock().unwrap().connect_to(&params.ssid, &params.password);
    "This is the base".to_string()
}