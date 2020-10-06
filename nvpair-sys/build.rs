extern crate bindgen;
use std::{env, fs, path::PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=nvpair");
    println!("cargo:rustc-link-lib=uutil"); // brings in symbols for aok
    let bindings_path = out_path.join("bindings.rs");

    let builder = bindgen::Builder::default()
        .header_contents(
            "bindings.h",
            "
#include <libnvpair.h>
        ",
        )
        .derive_debug(true)
        .derive_default(true)
        .clang_args(vec!["-I/usr/include/libzfs", "-I/usr/include/libspl"]);

    let bindings = builder
        .whitelist_function("f?nv(pair|list)_.*")
        .rustified_enum(".*")
        .generate()
        .expect("Unable to generate nvpair_sys bindings.")
        .to_string();

    // insert strum macros
    let mut edited_bindings = String::new();
    for line in bindings.lines() {
        if line.starts_with("pub enum ") {
            edited_bindings.push_line(
                "#[derive(EnumIter, IntoStaticStr, EnumString, FromPrimitive, ToPrimitive)]",
            );
        }

        edited_bindings.push_line(line);
    }

    fs::write(&bindings_path, edited_bindings).expect("couldn't write nvpair_sys bindings");
}

trait PushLine {
    fn push_line(&mut self, s: &str);
}

impl PushLine for String {
    fn push_line(&mut self, s: &str) {
        self.push_str(s);
        self.push('\n');
    }
}
