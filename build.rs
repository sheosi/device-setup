use std::fs::{read_to_string, write};
use std::path::Path;

use html_minifier::HTMLMinifier;

fn get_arch() -> &'static str {
    match std::env::consts::ARCH {
        "x86_64" => "x64",
        "arm" => "armv7",
        "aarch64" => "arch64",
        _ => panic!("Unsupported host arch")
    }
}

fn get_os() -> &'static str {
    match std::env::consts::OS {
        "linux" => "linux",
        "windows" => "windows",
        "macos" => "macos",
        _ => panic!("Unsupported host os")
    }
}

fn get_file_ext() -> &'static str {
    match std::env::consts::OS {
        "windows" => ".exe",
        _ => ""
    }
}

fn call_sys(cmd: &str, args:&[&str]) {
    let cmd_out = String::from_utf8(std::process::Command::new(cmd).args(args).output().unwrap().stdout).unwrap();
    println!("{cmd_out}");
}

const HTML_FILES: [&str; 1] = ["setup"];

// Minify html file
fn main() {
    println!("cargo:rerun-if-changed=templates/setup.html");
    
    let mut html_minifier = HTMLMinifier::new();
    let file_ext = get_file_ext();
    let tailwindcss_name = format!("build-deps/tailwindcss{}", file_ext);
    if !Path::new(&tailwindcss_name).exists() {
        std::fs::create_dir("build-deps").unwrap();
        let h_os = get_os();
        let h_arch = get_arch();
        call_sys("curl" ,&["-sL",&format!("https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-{h_os}-{h_arch}{file_ext} -o build-deps/tailwindcss{file_ext}")]);
        call_sys("chmod", &["+x", &format!("build-deps/tailwindcss{file_ext}")])
    }

    for html_file in HTML_FILES {
        html_minifier.digest(read_to_string(format!("templates/{}.html", html_file)).unwrap()).unwrap();
        write("templates/setup.min.html", html_minifier.get_html()).unwrap();
    }

    call_sys("./build-deps/tailwindcss", &["-c", "tailwind.config.js"])
}
