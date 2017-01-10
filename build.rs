extern crate serde_codegen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let types_src = Path::new("src/types.in.rs");
    let types_dst = Path::new(&out_dir).join("types.rs");

    let args_src = Path::new("src/args.in.rs");
    let args_dst = Path::new(&out_dir).join("args.rs");

    serde_codegen::expand(&types_src, &types_dst).unwrap();
    serde_codegen::expand(&args_src, &args_dst).unwrap();
}