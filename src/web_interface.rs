
pub mod render {
    use askama::Template;

    use crate::translations::Translator;

    pub fn setup_form(trans: &Translator) -> String {

        #[derive(Template)]
        #[template(path = "setup.html")]
        struct SetupForm {
            select_lang_title: String,
            network_name: String,
            password: String,
            previous_btn: String,
            next_btn: String,
            finish_btn: String
        }

        //let a = trans.translate( "miau", None);
        SetupForm {
            select_lang_title: trans.translate("select_lang_title", None),
            network_name: trans.translate("network_name", None),
            password: trans.translate("password", None),
            previous_btn: trans.translate("previous_btn", None),
            next_btn: trans.translate("next_btn", None),
            finish_btn: trans.translate("finish_btn", None),
        }.render().unwrap()
    }
}
