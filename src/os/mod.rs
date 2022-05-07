pub mod wifi_handler;


pub mod locale {
    use std::process::Command;
    use unic_langid::LanguageIdentifier;

    pub fn set(locale: &LanguageIdentifier) ->Result<(), ()> {
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

    pub fn current() -> Result<LanguageIdentifier, ()> {
        let o = Command::new("/usr/bin/localectl").output().unwrap();
        let locale_out = std::str::from_utf8(&o.stdout).unwrap();

        const LANG_START: &str = "System Locale: LANG=";
        if locale_out.starts_with("System Locale: LANG=") {
            let pos = locale_out.find('.').unwrap_or_else(||locale_out.find('\n').unwrap_or(locale_out.len()));
            let l = locale_out[LANG_START.len()..pos].parse().unwrap();
            Ok(l)
        }
        else {
            Err(())
        }

        
    }
}




