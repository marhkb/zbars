extern crate bindgen;
#[cfg(unix)]
extern crate pkg_config;

use std::{
    borrow::Cow,
    env,
    path::PathBuf,
};

fn main() {

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(link())
        .rustified_enum(".*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(windows)]
fn link() -> Cow<'static, str> {
    println!("cargo:rustc-link-search={}", env!("ZBAR_LIB_DIR"));
    println!("cargo:rustc-link-lib=libzbar64-0");
    Cow::Owned(format!("{}", PathBuf::from(env!("ZBAR_INCLUDE_DIR")).join("zbar.h").to_str().unwrap()))
}

#[cfg(unix)]
fn link() -> Cow<'static, str> {
    if pkg_config::Config::new().atleast_version("0.10").probe("zbar").unwrap().version.parse::<f64>().unwrap() >= 0.2 {
        if cfg!(feature = "zbar_fork_if_available") {
            println!("cargo:rustc-cfg=feature=\"zbar_fork\"");
        }
    }
    Cow::Borrowed("wrapper.h")
}
