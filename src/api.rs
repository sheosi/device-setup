use std::sync::Mutex;

use crate::translations::Translations;
use crate::os::wifi_handler::{get_wifi_handler, WifiHandler};

use actix_web::{post, web, Scope, Responder, get};
use serde::{Deserialize, Serialize};
use unic_langid::{langid, LanguageIdentifier};

#[derive(Clone, Serialize)]
pub struct LangData {
    value: &'static str,
    name: &'static str
}

const LANGS: [LangData; 2] = [
    LangData{value: "es-ES", name: "Español (España)"}, 
    LangData{value: "en-US", name: "English (United States)"}
];

#[derive(Serialize)]
pub struct AllLangs {
    all: Vec<LangData>,
    current: String
}

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
        .service(get_langs)
        .service(set_lang)
        .service(do_setup)
}


#[post("/lang")]
async fn set_lang(data: web::Data<AppState>, params: web::Query<api_impl::SetLangParams>) -> impl Responder {
    api_impl::set_lang(data, &params).unwrap();
    "Ok"
}

#[get("/lang/all")]
async fn get_langs(data: web::Data<AppState>) -> impl Responder {
    web::Json(AllLangs{ all: LANGS.to_vec(), current: data.lang.lock().unwrap().to_string()})
}

#[derive(Deserialize)]
struct DoSetupParams {
    #[serde(flatten)]
    lang: api_impl::SetLangParams,
    #[serde(flatten)]
    wifi: api_impl::ConnectParams
}

#[post("/do-setup")]
async fn do_setup(data: web::Data<AppState>, params: web::Form<DoSetupParams>) -> impl Responder {
    api_impl::set_lang(data.clone(), &params.lang).unwrap();
    api_impl::connect_wifi(data, &params.wifi).await.unwrap();

    "Ok"
}

mod api_impl {
    
    use super::AppState;
    use crate::os;

    use serde::Deserialize;
    use actix_web::web;

    #[derive(Deserialize)]
    pub struct SetLangParams {
        lang: String
    }

    pub fn set_lang(data: web::Data<AppState>, params: &SetLangParams) -> Result<(),()> {
        let lang = params.lang.parse().unwrap();
        os::set_locale(&lang);
        *data.lang.lock().unwrap() = lang;

        Ok(())
    }

    #[derive(Deserialize)]
    pub struct ConnectParams {
        ssid: String,
        password: String
    }

    pub async fn connect_wifi(data: web::Data<AppState>, params: &ConnectParams) -> Result<(), ()> {
        let a = data.wifi.lock().unwrap().connect_to(&params.ssid, &params.password);
        
        Ok(())
    }
}