pub mod wifi_handler;

use std::process::Command;
use unic_langid::LanguageIdentifier;

pub fn set_locale(locale: &LanguageIdentifier) ->Result<(), ()> {
    let locale = locale.to_string().replace('-', "_");
    call_locale(&format!("LANG={}.UTF-8", locale))?;
    call_locale(&format!("LC_TIME={}.UTF-8", locale))
}

fn call_locale(locale: &str) -> Result<(), ()> {
    let s = Command::new("/usr/bin/localectl")
        .arg("set-locale")
        .arg(locale)
        .status().unwrap();
    
    Ok(())
}

