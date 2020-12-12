use std::env;
use std::env::consts;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(_) = pkg_config::probe_library("ubus") {
        if let Some(lib_path) = env::var_os("LIBUBUS_DIR") {
            // I have no idea if this branch works
            let lib_dir = Path::new(&lib_path);
            let dylib_name = format!("{}ubus{}", consts::DLL_PREFIX, consts::DLL_SUFFIX);
            if lib_dir.join(dylib_name).exists()
                || lib_dir.join("libubus.a").exists()
                || lib_dir.join("ubus.lib").exists()
            {
                println!(
                    "cargo:rustc-link-search=native={}",
                    lib_path.into_string().unwrap()
                );
                println!("cargo:rustc-link-lib=ubus");
            }
        } else {
            let json_c = cmake::build("vendor/json-c");

            cc::Build::new()
                .file("vendor/ubus/libubus.c")
                .file("vendor/ubus/libubus-req.c")
                .file("vendor/ubus/libubus-io.c")
                .file("vendor/ubus/libubus-obj.c")
                .file("vendor/ubus/libubus-sub.c")
                .file("vendor/libubox/avl.c")
                .file("vendor/libubox/utils.c")
                .file("vendor/libubox/uloop.c")
                .file("vendor/libubox/blob.c")
                .file("vendor/libubox/usock.c")
                .file("vendor/libubox/blobmsg.c")
                .file("vendor/libubox/blobmsg_json.c")
                .include("vendor")
                .include("vendor/ubus")
                .include("vendor/libubox")
                .include(json_c.join("include/json-c"))
                .define("UBUS_MAX_MSGLEN", "1048576")
                .define("UBUS_UNIX_SOCKET", "\"/var/run/ubus.sock\"")
                .define("JSONC", "1")
                .shared_flag(true)
                .compile("libubus.so");
        }
    }

    let bindings = bindgen::Builder::default()
        .header("vendor/ubus/libubus.h")
        .header("vendor/libubox/blobmsg_json.h")
        .clang_args(vec!["-Ivendor", "-target", "mips-unknown-linux"])
        .whitelist_function("ubus.*")
        .whitelist_type("ubus.*")
        .whitelist_var("ubus.*")
        .whitelist_function("blob.*")
        .whitelist_var("blob.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs");
}
