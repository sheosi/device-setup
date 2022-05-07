use std::collections::HashMap;

use fluent::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};
use serde::Serialize;

pub const DEF_LANG: LanguageIdentifier = langid!("en-US");

pub const LANGS: [LangData; 2] = [
    LangData{value: "es-ES", name: "Español (España)", ftl: include_str!("../i18n/es-ES.ftl")}, 
    LangData{value: "en-US", name: "English (United States)", ftl: include_str!("../i18n/en-US.ftl")}
];

#[derive(Clone, Serialize)]
pub struct LangData {
    pub value: &'static str,
    pub name: &'static str,
    #[serde(skip_serializing)]
    pub ftl: &'static str
}


enum TranslationState {
    Raw(&'static str),
    Compiled(Translator)
}

impl TranslationState {
    pub fn compile(&mut self, lang_id: &LanguageIdentifier)  {
        if let Self::Raw(str) = self {
            let res = FluentResource::try_new(str.to_string()).expect("Failed to parse FTL");
            let mut bundle = FluentBundle::new(vec![lang_id.clone()]);
            bundle.add_resource(res).expect("Failed to add FTL resource");

            *self = Self::Compiled(Translator{inner: bundle})
        }
    }

    pub fn get(&mut self, lang_id: &LanguageIdentifier) -> &Translator {
        self.compile(lang_id); // Changes itself to Translation::Compiled

        if let Self::Compiled(translator) = self {
            translator
        }
        else {
            panic!("We just compiled, yet it's not available as compiled, report this");
        }
    }
}

pub struct Translator {
    inner: FluentBundle<FluentResource>
}

impl Translator {
    pub fn translate(&self, resource: &str, args: Option<&FluentArgs>) -> String {
        let val = self.inner
            .get_message(resource).expect("Resource does not exist")
            .value().expect("Has no value");
            
        let mut errors = vec![];
        self.inner.format_pattern(val, args, &mut errors).to_string()
    }
}


pub struct Translations {
    inner: HashMap<LanguageIdentifier, TranslationState>
}

impl Translations {
    pub fn new(current: &LanguageIdentifier) -> Self {
        let mut result = HashMap::new();

        // Insert langs
        for l in LANGS {
            result.insert(l.value.parse().expect("Used an invalid identifier"), TranslationState::Raw(l.ftl));
        }

        // Compile current lang
        if let Some(l) = result.get_mut(current) {
            l.compile(current);
        }
        else {
            result.get_mut(&DEF_LANG).expect("Report this").compile(&DEF_LANG);
        }
    
        Self {inner: result}
    }

    pub fn get_or_def(&mut self, lang: &LanguageIdentifier, def: &LanguageIdentifier) -> &Translator {
        if self.inner.contains_key(lang) {
            self.inner.get_mut(lang).map(|t|t.get(lang)).expect("Report this")
        }
        else {
            self.inner.get_mut(def).map(|t|t.get(def)).expect("Report this")
        }
    }
}