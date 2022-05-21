use std::{collections::HashMap, path::Path};

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

    pub fn load_or_def(lang: LanguageIdentifier, def: &LanguageIdentifier) -> Self {
        let path = path_for(&lang);
        if Path::new(&path).exists() {
            Self::load(lang, &path)
        }
        else {
            Self::load(def.clone(), &path_for(def))
        }
    }

    pub fn translate(&self, resource: &str, args: Option<&FluentArgs>) -> String {
        let val = self.inner
            .get_message(resource).expect("Resource does not exist")
            .value().expect("Has no value");
            
        let mut errors = vec![];
        self.inner.format_pattern(val, args, &mut errors).to_string()
    }
}

fn make_lang_name_dict() -> HashMap<&'static str, &'static str> {
    let mut result = HashMap::new();
    for (name,lang) in LANG_NAMES {
        result.insert(name, lang);
    }

    result
}

fn path_for(lang: &LanguageIdentifier) -> String {
    format!("i18n/{}.ftl", lang)
}