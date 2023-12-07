use std::env;
use std::path::PathBuf;

fn main() {
    let link_type;

    {
        link_type = "dylib";

        // When using an already built version, build all possible paths to look for ...
        let dir = env::var("MTKAHYPAR_DIR").map(PathBuf::from).ok();

        let include_dir = dir.clone().map(|dir| dir.join("include"));
        let lib_dir = dir.clone().map(|dir| dir.join("lib"));
        let lib64_dir = dir.map(|dir| dir.join("lib64"));

        // ... and include them for the library search.
        if let Some(dir) = include_dir {
            println!("cargo:include-dir={}", dir.display());
        }

        if let Some(dir) = lib64_dir {
            println!("cargo:rustc-link-search={}", dir.display());
        }

        if let Some(dir) = lib_dir {
            println!("cargo:rustc-link-search={}", dir.display());
        }

        println!("rerun-if-env-changed=MTKAHYPAR_DIR");
    }

    // Finally, link the library.
    println!("cargo:rustc-link-lib={}=mtkahypar", link_type);
}
