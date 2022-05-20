use std::collections::HashMap;

use fluent::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};
use serde::Serialize;

pub const DEF_LANG: LanguageIdentifier = langid!("en-US");

pub const LANGS: [LangData; 2] = [
    LangData{value: "es-ES", name: "Español (España)", path: "i18n/es-ES.ftl"}, 
    LangData{value: "en-US", name: "English (United States)", path: "i18n/en-US.ftl"}
];

#[derive(Clone, Serialize)]
pub struct LangData {
    pub value: &'static str,
    pub name: &'static str,
    #[serde(skip_serializing)]
    pub path: &'static str
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
            result.insert(l.value.parse().expect("Used an invalid identifier"), l.path.to_string());
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