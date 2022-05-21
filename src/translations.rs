use std::collections::HashMap;

use crate::vars::*;

use fluent::{FluentArgs, FluentBundle, FluentResource};
use lazy_static::lazy_static;
use unic_langid::LanguageIdentifier;

lazy_static! {
    pub static ref LANG_NAMES_DICT: HashMap<&'static str, &'static str> = make_lang_name_dict();
}

pub struct Translator {
    inner: FluentBundle<FluentResource>
}

impl Translator {
    pub fn load(lang_id: LanguageIdentifier, path: &str) -> Self {
        let contents = std::fs::read_to_string(path).unwrap();
        let res = FluentResource::try_new(contents).expect("Failed to parse FTL");
        let mut bundle = FluentBundle::new(vec![lang_id]);
        bundle.add_resource(res).expect("Failed to add FTL resource");

        Self {inner: bundle}
    }

    pub fn translate(&self, resource: &str, args: Option<&FluentArgs>) -> String {
        let val = self.inner
            .get_message(resource).expect("Resource does not exist")
            .value().expect("Has no value");
            
        let mut errors = vec![];
        self.inner.format_pattern(val, args, &mut errors).to_string()
    }
}


pub struct Translations {
    inner: HashMap<LanguageIdentifier, String>,
    current: Translator
}

impl Translations {
    pub fn new(current: &LanguageIdentifier) -> Self {
        let mut result:HashMap<_, String> = HashMap::new();

        // Insert langs
        for l in LANGS {
            let path = format!("i18n/{}.ftl", l);
            result.insert(l.parse().expect("Used an invalid identifier"), path.to_string());
        }

        // Compile current lang
        let current = if let Some(l) = result.get(current) {
            Translator::load(current.clone(), l)
        }
        else {
            Translator::load(DEF_LANG, result.get(&DEF_LANG).expect("Report this"))
        };
    
        Self {inner: result, current}
    }

    pub fn set(&mut self, new_lang: &LanguageIdentifier) {
        // Compile current lang
        let current = if let Some(l) = self.inner.get(new_lang) {
            Translator::load(new_lang.clone(), l)
        }
        else {
            Translator::load(DEF_LANG, self.inner.get(&DEF_LANG).expect("Report this"))
        };

        self.current = current;
    }

    pub fn get(&mut self) -> &Translator {
        &self.current
    }

}

fn make_lang_name_dict() -> HashMap<&'static str, &'static str> {
    let mut result = HashMap::new();
    for (name,lang) in LANG_NAMES {
        result.insert(name, lang);
    }

    result
}