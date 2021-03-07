fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");

    let dst = cmake::build("astc");
    let shim_path = dst.join("lib");
    let base_path = dst.join("build/astc-codec/src/decoder");

    println!("cargo:rustc-link-search=native={}", shim_path.display());
    println!("cargo:rustc-link-search=native={}", base_path.display());
    println!("cargo:rustc-link-lib=static=astc-shim");
    println!("cargo:rustc-link-lib=static=footprint");
    println!("cargo:rustc-link-lib=static=astc_utils");
    println!("cargo:rustc-link-lib=static=astc-codec");

    println!("cargo:rustc-flags=-l dylib=stdc++");
}
