
pub mod render {
    use askama::Template;

    use crate::translations::Translator;

    pub fn setup_form(trans: &Translator) -> String {

        #[derive(Template)]
        #[template(path = "setup.html")]
        struct SetupForm {

        }

        //let a = trans.translate( "miau", None);
        SetupForm {}.render().unwrap()
    }
}
