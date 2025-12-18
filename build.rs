use std::{
    fs::{self, OpenOptions},
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

    // Read and fix OpenAPI JSON file
    let json_content = fs::read_to_string("./docs/openapi.json").unwrap();

    // Fix cases where type is "string" but enum contains numbers
    // Use precise regex: only convert numbers in "enum" arrays that follow "type": "string"
    let fixed_content =
        regex::Regex::new(r#"("type"\s*:\s*"string"[^}]*?"enum"\s*:\s*\[)([^\]]+)(\])"#)
            .expect("Failed to compile OpenAPI type-enum regex pattern")
            .replace_all(&json_content, |caps: &regex::Captures| {
                let before = &caps[1];
                let enum_content = &caps[2];
                let after = &caps[3];

                // Replace bare numbers with strings only in enum content
                let fixed_enum = regex::Regex::new(r#"\b(\d+)\b"#)
                    .expect(r"Failed to compile regex pattern '\b(\d+)\b'")
                    .replace_all(enum_content, r#""$1""#);

                format!("{}{}{}", before, fixed_enum, after)
            });

    let content = prettyplease::unparse(
        &syn::parse2(
            Generator::default()
                .generate_tokens(&serde_json::from_str(&fixed_content).unwrap())
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
