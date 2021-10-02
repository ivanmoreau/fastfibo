use std::env;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=dylib=c");
}