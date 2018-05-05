extern crate pkg_config;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let config = pkg_config::Config::new().atleast_version("0.10").probe("zbar").unwrap();
    if config.version.parse::<f64>().unwrap() >= 0.2 {
        println!("cargo:rustc-cfg=feature=\"zbar_fork\"");
    }

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=zbar");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
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
