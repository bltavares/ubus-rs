fn main() {
    println!("cargo:rustc-link-lib=dylib=ubus");
    println!("cargo:rustc-link-lib=dylib=ubox");
    println!("cargo:rustc-link-lib=dylib=json-c");
    println!("cargo:rustc-link-lib=dylib=blobmsg_json");
}
