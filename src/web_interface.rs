
pub mod render {
    use askama::Template;

    use crate::translations::Translator;

    pub fn language_select(trans: &Translator) -> String {

        #[derive(Template)]
        #[template(path = "language_select.html")]
        struct LangSelect {

        }

        //let a = trans.translate( "miau", None);
        LangSelect {}.render().unwrap()
    }

    pub fn wifi_connect(trans: &Translator) -> String {
        #[derive(Template)]
        #[template(path = "wifi_conf.html")]
        struct WifiConf {

        }

        WifiConf {}.render().unwrap()
    }
}
