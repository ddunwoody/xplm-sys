/*
 * Copyright © 2023 David Dunwoody.
 *
 * All rights reserved.
 */

fn main() {
    println!("cargo:rerun-if-env-changed=XPLANE_SDK");
    let sdk_path = std::path::Path::new(env!("XPLANE_SDK"));

    configure(&sdk_path);

    #[cfg(feature = "generate-bindings")]
    generate_bindings(&sdk_path);
}

fn configure(sdk_path: &std::path::Path) {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target == "macos" {
        println!(
            "cargo:rustc-link-search=framework={}",
            sdk_path.join("Libraries/Mac").display()
        );
        println!("cargo:rustc-link-lib=framework=XPLM");
        println!("cargo:rustc-link-lib=framework=XPWidgets");
    } else if target == "windows" {
        println!(
            "cargo:rustc-link-search={}",
            sdk_path.join("Libraries/Win").display()
        );
        println!("cargo:rustc-link-lib=XPLM_64");
        println!("cargo:rustc-link-lib=XPWidgets_64");
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(sdk_path: &std::path::Path) {
    println!("cargo:rerun-if-changed=xplm.h");
    bindgen::Builder::default()
        .clang_args([
            "-fparse-all-comments",
            "-DXPLM200",
            "-DXPLM210",
            "-DXPLM300",
            "-DXPLM301",
            "-DXPLM303",
            "-DXPLM400",
            "-DXPLM410",
            "-DLIN", // doesn't matter for bindings, but we need to be defined to be able to compile
            &format!("-I{}", sdk_path.join("CHeaders/XPLM").display()),
        ])
        .header("xplm.h")
        .allowlist_function("XPLM.*")
        .allowlist_type("XPLM.*")
        .allowlist_var("XPLM.*")
        .allowlist_var("xplm.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings");
}
