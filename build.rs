use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use progenitor::Generator;

fn join(root: &str, next: &str) -> String {
    Path::new(root)
        .join(next)
        .to_str()
        .unwrap()
        .replace("\\\\?\\", "")
        .replace("\\", "/")
        .to_string()
}

fn main() {
    println!("cargo:rerun-if-changed=../docs/openapi.json");

    let content = prettyplease::unparse(
        &syn::parse2(
            Generator::default()
                .generate_tokens(
                    &serde_json::from_reader(File::open("./docs/openapi.json").unwrap()).unwrap(),
                )
                .unwrap(),
        )
        .unwrap(),
    );

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(join(&std::env::var("OUT_DIR").unwrap(), "bindgen.rs"))
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}
