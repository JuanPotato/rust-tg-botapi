extern crate serde_codegen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("src/types.in.rs");
    let dst = Path::new(&out_dir).join("types.rs");

    let src2 = Path::new("src/method_return_types.in.rs");
    let dst2 = Path::new(&out_dir).join("method_return_types.rs");

    serde_codegen::expand(&src, &dst).unwrap();
    serde_codegen::expand(&src2, &dst2).unwrap();
}