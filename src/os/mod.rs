pub mod wifi_handler;

use std::process::Command;
use unic_langid::LanguageIdentifier;



pub fn set_locale(locale: &LanguageIdentifier)  {
    let s = Command::new("/usr/bin/localectl")
        .arg("set-locale")
        .arg(format!("LANG={}.UTF-8", locale))
        .status().unwrap();
    
    let s = Command::new("/usr/bin/localectl")
        .arg("set-locale")
        .arg(format!("LC_TIME={}.UTF-8", locale))
        .status().unwrap();
}