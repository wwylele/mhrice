fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");

    cc::Build::new()
        .cpp(true)
        .file("ffi/ffi_shim.cpp")
        .file("ffi/bc7decomp.cpp")
        .compile("ffi-shim");
}
