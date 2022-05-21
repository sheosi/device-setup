use unic_langid::{langid, LanguageIdentifier};

/// Default language
pub const DEF_LANG: LanguageIdentifier = langid!("en-US");

/// Languages supported by the current appliance
pub const LANGS: [&str; 2] = [
    "es-ES", "en-US"
];

/// Human names for the locales
pub const LANG_NAMES: [(&str, &str); 2] = [
    ("es-ES", "Español (España)"),
    ("en-US", "English (United States)")
];