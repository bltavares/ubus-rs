use std::env;
use std::env::consts;
use std::path::{Path, PathBuf};

fn main() {
    let mut bindings = bindgen::Builder::default().derive_debug(false);
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    if let Some(lib_path) = env::var_os("LIBUBUS_DIR") {
        // I have no idea if this branch works
        let lib_dir = Path::new(&lib_path).display().to_string();
        println!("cargo:rustc-link-search=native={}/lib", lib_dir);
        bindings = bindings.clang_arg(format!("-I{}/include", lib_dir));
    } else {
        let _libjson_c = cmake::Config::new("vendor/json-c")
            .define("BUILD_LUA", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .build();

        env::set_var(
            "PKG_CONFIG_PATH",
            format!(
                "$PKG_CONFIG_PATH:{}",
                out_path.join("lib").join("pkgconfig").display().to_string()
            ),
        );
        let _libubox = cmake::Config::new("vendor/libubox")
            .define("BUILD_LUA", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .build();
        let _libubus = cmake::Config::new("vendor/ubus")
            .define("BUILD_LUA", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .build();
        println!("cargo:rustc-link-search=native={}/lib", out_path.display());
        bindings = bindings.clang_arg(format!("-I{}/include", out_path.display()));
    }

    if let Ok(bindgen_target) = env::var("BINDGEN_TARGET") {
        bindings = bindings.clang_arg(format!("--target={}", bindgen_target));
    }

    println!("cargo:rustc-link-lib=dylib=ubus");
    println!("cargo:rustc-link-lib=dylib=ubox");
    println!("cargo:rustc-link-lib=dylib=json-c");
    println!("cargo:rustc-link-lib=dylib=blobmsg_json");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindings
        .header("wrapper.h")
        .whitelist_function("ubus.*")
        .whitelist_type("ubus.*")
        .whitelist_var("ubus.*")
        .whitelist_function("blob.*")
        .whitelist_var("blob.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs");
}
