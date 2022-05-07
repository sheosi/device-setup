use crate::translations::{LangData, LANGS};
use serde::Serialize;

#[derive(Serialize)]
pub struct AllLangs {
    all: Vec<LangData>,
    current: String
}

pub mod render {
    use askama::Template;
    use unic_langid::LanguageIdentifier;

    use crate::{translations::Translator, web_interface::AllLangs};

    pub fn setup_form(trans: &Translator, curr_lang: &LanguageIdentifier) -> String {

        #[derive(Template)]
        #[template(path = "setup.html")]
        struct SetupForm {
            select_lang_title: String,
            network_name: String,
            password: String,
            previous_btn: String,
            next_btn: String,
            finish_btn: String,
            langs: super::AllLangs
        }

        SetupForm {
            select_lang_title: trans.translate("select_lang_title", None),
            network_name: trans.translate("network_name", None),
            password: trans.translate("password", None),
            previous_btn: trans.translate("previous_btn", None),
            next_btn: trans.translate("next_btn", None),
            finish_btn: trans.translate("finish_btn", None),
            langs: AllLangs { all: super::LANGS.to_vec(), current:  curr_lang.to_string()}
        }.render().unwrap()
    }
}
