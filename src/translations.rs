use std::collections::HashMap;

use fluent::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

enum TranslationState {
    Raw(&'static str),
    Compiled(Translator)
}

impl TranslationState {
    pub fn compile(&mut self, lang_id: &LanguageIdentifier)  {
        if let Self::Raw(str) = self {
            let res = FluentResource::try_new(str.to_string()).unwrap();
            let mut bundle = FluentBundle::new(vec![lang_id.clone()]);
            bundle.add_resource(res).unwrap();

            *self = Self::Compiled(Translator{inner: bundle})
        }
    }

    pub fn get(&mut self, lang_id: &LanguageIdentifier) -> &Translator {
        println!("{:?}", lang_id);

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
        let val = self.inner.get_message(resource).unwrap().value().unwrap();
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
        result.insert(langid!("en-US"), TranslationState::Raw(include_str!("../i18n/en-US.ftl")));
        result.insert(langid!("es-ES"), TranslationState::Raw(include_str!("../i18n/es-ES.ftl")));

        // Compile current lang
        if let Some(l) = result.get_mut(current) {
            l.compile(current);
        }
        else {
            let en_US = langid!("en-US");
            result.get_mut(&en_US).unwrap().compile(&en_US);
        }
    
        Self {inner: result}
    }

    pub fn get(&mut self, lang: &LanguageIdentifier) -> &Translator {
        self.inner.get_mut(lang).unwrap().get(lang)
    }
}