fn get_cpp_link_stdlib() -> Option<String> {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("msvc") {
        None
    } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        Some("c++".to_string())
    } else {
        Some("stdc++".to_string())
    }
}

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");

    let dst = cmake::build("ffi").join("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ffi-shim");
    println!("cargo:rustc-link-lib=static=footprint");
    println!("cargo:rustc-link-lib=static=astc_utils");
    println!("cargo:rustc-link-lib=static=astc-codec");

    if let Some(cpp) = get_cpp_link_stdlib() {
        println!("cargo:rustc-flags=-l dylib={}", cpp);
    }
}
