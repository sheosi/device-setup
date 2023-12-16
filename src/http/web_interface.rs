pub mod render {
    use crate::{http::translations::{Translator, LANG_NAMES_DICT}, vars::LANGS};

    use sailfish::TemplateOnce;
    use unic_langid::LanguageIdentifier;
    use serde::Serialize;

    pub fn setup_form(trans: &Translator, curr_lang: &LanguageIdentifier) -> String {

        #[derive(TemplateOnce)]
        #[template(path = "setup.min.html")]
        struct SetupForm {
            select_lang_title: String,
            network_name: String,
            password: String,
            previous_btn: String,
            next_btn: String,
            finish_btn: String,
            langs: AllLangs
        }

        SetupForm {
            select_lang_title: trans.translate("select_lang_title", None),
            network_name: trans.translate("network_name", None),
            password: trans.translate("password", None),
            previous_btn: trans.translate("previous_btn", None),
            next_btn: trans.translate("next_btn", None),
            finish_btn: trans.translate("finish_btn", None),
            langs: AllLangs { all: LangData::get_all(), current:  curr_lang.to_string()}
        }.render_once().expect("Formatting failed, report this")
    }

    #[derive(Serialize)]
    pub struct AllLangs {
        all: Vec<LangData>,
        current: String
    }

    #[derive(Clone, Serialize)]
    pub struct LangData {
        pub value: &'static str,
        pub name: &'static str,
    }

    impl LangData {
        fn get_all() -> Vec<LangData> {
            LANGS.iter().map(|l|LangData{value: l, name: LANG_NAMES_DICT[l]}).collect()
        }
    }


}
