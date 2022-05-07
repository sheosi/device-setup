use std::sync::Mutex;

use crate::translations::{Translations, DEF_LANG};
use crate::os::{self, wifi_handler::{get_wifi_handler, WifiHandler}};

use actix_web::{post, web, Scope, Responder};
use serde::Deserialize;
use unic_langid::LanguageIdentifier;

pub struct AppState {
    pub wifi: Mutex<Box<dyn WifiHandler>>,
    pub lang: Mutex<LanguageIdentifier>,
    pub translations: Mutex<Translations>
}

impl AppState {
    pub fn new() -> Self {
        let curr_lang = os::locale::current().unwrap_or(DEF_LANG);
        Self {
            wifi: Mutex::new(get_wifi_handler()),
            translations: Mutex::new(Translations::new(&curr_lang)),
            lang: Mutex::new(curr_lang)
        }
    }
}

pub fn scope() -> Scope {
    web::scope("/api")
        .service(set_lang)
        .service(do_setup)
}


#[post("/lang")]
async fn set_lang(data: web::Data<AppState>, params: web::Query<api_impl::SetLangParams>) -> impl Responder {
    api_impl::set_lang(data, &params).unwrap();
    "Ok"
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
    use crate::os::{self, wifi_handler::WifiError};

    use serde::Deserialize;
    use actix_web::web;

    #[derive(Deserialize)]
    pub struct SetLangParams {
        lang: String
    }

    pub fn set_lang(data: web::Data<AppState>, params: &SetLangParams) -> Result<(),()> {
        let lang = params.lang.parse().unwrap();
        os::locale::set(&lang)?;
        *data.lang.lock().unwrap() = lang;

        Ok(())
    }

    #[derive(Deserialize)]
    pub struct ConnectParams {
        ssid: String,
        password: String
    }

    pub async fn connect_wifi(data: web::Data<AppState>, params: &ConnectParams) -> Result<(), WifiError> {
        data.wifi.lock().unwrap().connect_to(&params.ssid, &params.password).await
    }
}