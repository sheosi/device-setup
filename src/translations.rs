use std::collections::HashMap;

use fluent::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

enum TranslationState {
    Raw(&'static str),
    Compiled(Translator)
}

impl TranslationState {
    pub fn compiled(lang_id: &LanguageIdentifier, str: &'static str) -> Self {
        let mut trans = Self::Raw(str);
        trans.compile(lang_id);

        trans
    }
    pub fn compile(&mut self, lang_id: &LanguageIdentifier)  {
        if let Self::Raw(str) = self {
            let res = FluentResource::try_new(str.to_string()).unwrap();
            let mut bundle = FluentBundle::new(vec![lang_id.clone()]);
            bundle.add_resource(res).unwrap();

            *self = Self::Compiled(Translator{inner: bundle})
        }
    }

    pub fn get(&mut self, lang_id: &LanguageIdentifier) -> &Translator {

        self.compile(lang_id); // Changes itself to Translation::Compiled

        if let Self::Compiled(translator) = self {
            translator
        }
        else {
            panic!("We just compiled, jet it's not available as compiled, report this");
        }
    }
}

pub struct Translator {
    inner: FluentBundle<FluentResource>
}

impl Translator {
    pub fn translate(&self, resource: &str, args: Option<&FluentArgs>) -> String {
        let val = self.inner.get_message(resource).unwrap().value().unwrap();
        let mut errors = vec![];
        self.inner.format_pattern(val, args, &mut errors).to_string()
    }
}


pub struct Translations {
    inner: HashMap<LanguageIdentifier, TranslationState>
}

impl Translations {
    pub fn new() -> Self {
        let mut result = HashMap::new();

        // Default lang
        let en_us = langid!("en-US");

        result.insert(en_us.clone(), TranslationState::compiled(&en_us, include_str!("../i18n/es-ES.ftl")));
    
        // Rest of langs
        result.insert(langid!("es-ES"), TranslationState::Raw(include_str!("../i18n/es-ES.ftl")));
    
        Self {inner: result}
    }

    pub fn get(&mut self, lang: &LanguageIdentifier) -> &Translator {
        self.inner.get_mut(lang).unwrap().get(lang)
    }
}