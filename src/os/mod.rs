pub mod wifi;

use std::process::Command;
use actix_web::{ResponseError, HttpResponse, body};
use thiserror::Error;

pub mod locale {
    use std::process::Command;
    use actix_web::{ResponseError, HttpResponse, body};
    use unic_langid::LanguageIdentifier;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("Got unexpected output from localectl: {0}")]
        UnexpectedOutput(String),

        #[error("Failed to run localectl: {0}")]
        Localectl(#[from] std::io::Error),

        #[error("Locale set error: {0}")]
        Set(String),

        #[error("Failed to parse locale: {0}")]
        ParseLocale(#[from] unic_langid::LanguageIdentifierError),

        #[error("Is not UTF-8")]
        IsNotUtf8
    }

    impl ResponseError for Error {
        fn error_response(&self) -> HttpResponse<body::BoxBody> {
            if let Error::ParseLocale(_) = self {
                HttpResponse::BadRequest().body(self.to_string())
            }
            else {
                HttpResponse::InternalServerError().body(self.to_string())
            }
        }
    }

    pub fn set(locale: &LanguageIdentifier) ->Result<(), Error> {
        let locale = locale.to_string().replace('-', "_");
        call_locale(&format!("LANG={}.UTF-8", locale))?;
        call_locale(&format!("LC_TIME={}.UTF-8", locale))
    }

    fn call_locale(locale: &str) -> Result<(), Error> {
        let s = Command::new("/usr/bin/localectl")
            .arg("set-locale")
            .arg(locale)
            .output()?;

        if !s.status.success() {
            return Err(Error::Set(std::str::from_utf8(&s.stdout).unwrap_or("ERROR: CONTAINS NON-UNICODE, REPORT THIS").to_string()));
        }
        
        Ok(())
    }

    pub fn current() -> Result<LanguageIdentifier, Error> {
        let o = Command::new("/usr/bin/localectl").output()?;
        let locale_out = std::str::from_utf8(&o.stdout).map_err(|_| Error::IsNotUtf8)?;

        const LANG_START: &str = "   System Locale: LANG=";
        if locale_out.starts_with(LANG_START) {
            let pos = locale_out.find('.').unwrap_or_else(||locale_out.find('\n').unwrap_or(locale_out.len()));
            let l = locale_out[LANG_START.len()..pos].parse().expect("Shoudln't happen, report this");
            Ok(l)
        }
        else {
            Err(Error::UnexpectedOutput(locale_out.to_string()))
        }

        
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to execute systemctl: {0}")]
    Systemctl(#[from] std::io::Error),

    #[error("Systemctl didn't find device-setup")]
    DeviceSetupNotFound,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        HttpResponse::InternalServerError().body(self.to_string())
    }
}

pub fn stop_self() -> Result<(), Error> {
    if Command::new("/usr/bin/systemctl").arg("stop").arg("device-setup.service").status()?.success() {
        Ok(())
    }
    else {
        Err(Error::DeviceSetupNotFound)
    }
}