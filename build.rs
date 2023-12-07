use std::env;
use std::path::PathBuf;

fn main() {
    let link_type;
    #[cfg(feature = "bundled")]
    {
        panic!("Bundling is currently not supported, as Mt-KaHyPar can not be built statically.");

        link_type = "static";

        // When bundling mt-kahypar, build it with CMake ...
        let build_dir = cmake::Config::new("mt-kahypar")
            .define("BUILD_SHARED_LIBS", "OFF")
            .build_target("install.mtkahypar")
            .build();

        // ... and set the path to the built library to be linked (statically).
        println!(
            "cargo:rustc-link-search={}",
            build_dir.join("lib64").display()
        );

        println!(
            "cargo:rustc-link-search={}",
            build_dir.join("lib").display()
        );

        // Set include path for other libraries depending on mt-kahypar.
        println!("cargo:include-dir={}", build_dir.join("include").display());
    }

    #[cfg(not(feature = "bundled"))]
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
