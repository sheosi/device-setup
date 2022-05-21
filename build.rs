use std::fs::{read_to_string, write};

use html_minifier::HTMLMinifier;


// Minify html file
fn main() {
    println!("cargo:rerun-if-changed=templates/setup.html");
    
    let mut html_minifier = HTMLMinifier::new();

    html_minifier.digest(read_to_string("templates/setup.html").unwrap()).unwrap();
    write("templates/setup.min.html", html_minifier.get_html()).unwrap();
}
