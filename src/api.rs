use std::sync::Mutex;

use crate::os::{self, wifi};
use crate::translations::{Translations, DEF_LANG};

use actix_web::{post, web, Scope, Responder, ResponseError, HttpResponse, body};
use serde::Deserialize;
use thiserror::Error;
use unic_langid::LanguageIdentifier;

pub struct AppState {
    pub wifi: Mutex<Box<dyn wifi::Handler>>,
    pub lang: Mutex<LanguageIdentifier>,
    pub translations: Mutex<Translations>
}

#[allow(clippy::new_without_default)]
impl AppState {
    pub fn new() -> Self {
        let curr_lang = os::locale::current().unwrap_or(DEF_LANG);
        Self {
            wifi: Mutex::new(wifi::get_handler()),
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
async fn set_lang(data: web::Data<AppState>, params: web::Query<api_impl::SetLangParams>) -> Result<impl Responder, os::locale::Error> {
    api_impl::set_lang(data, &params)?;
    Ok("Ok")
}

#[derive(Deserialize)]
struct DoSetupParams {
    #[serde(flatten)]
    lang: api_impl::SetLangParams,
    #[serde(flatten)]
    wifi: api_impl::ConnectParams

}

#[derive(Debug, Error)]
enum SetupError {
    #[error("{0}")]
    Wifi(#[from] wifi::Error),

    #[error("{0}")]
    Locale(#[from] os::locale::Error),

    #[error("{0}")]
    Other(#[from] os::Error)
}

impl ResponseError for SetupError {
    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        match self {
            SetupError::Wifi(e) => e.error_response(),
            SetupError::Locale(e) => e.error_response(),
            SetupError::Other(e) => e.error_response()
        }
    }
}

#[post("/do-setup")]
async fn do_setup(data: web::Data<AppState>, params: web::Form<DoSetupParams>) -> Result<impl Responder, SetupError> {
    api_impl::set_lang(data.clone(), &params.lang)?;
    api_impl::connect_wifi(data, &params.wifi).await?;
    os::stop_self()?;

    Ok("Ok")
}

mod api_impl {
    use super::AppState;
    use crate::os::{self, wifi};

    use serde::Deserialize;
    use actix_web::web;

    #[derive(Deserialize)]
    pub struct SetLangParams {
        lang: String
    }

    pub fn set_lang(data: web::Data<AppState>, params: &SetLangParams) -> Result<(), os::locale::Error> {
        let lang = params.lang.parse()?;
        os::locale::set(&lang)?;
        data.translations.lock().unwrap().set(&lang);
        *data.lang.lock().unwrap() = lang;

        Ok(())
    }

    #[derive(Deserialize)]
    pub struct ConnectParams {
        ssid: String,
        password: String
    }

    pub async fn connect_wifi(data: web::Data<AppState>, params: &ConnectParams) -> Result<(), wifi::Error> {
        data.wifi.lock().unwrap().connect_to(&params.ssid, &params.password).await
    }
}