fn main() {
    tauri_build::build();

    // tauri-build embeds the Windows resource into binary targets only
    // (`cargo:rustc-link-arg-bins`). That resource carries the
    // Common-Controls v6 manifest which the `TaskDialogIndirect` import
    // (pulled in by muda, the menu library) needs to resolve. Integration
    // tests link as separate executables, so without the manifest a test
    // that touches the menu aborts before main() with
    // STATUS_ENTRYPOINT_NOT_FOUND (0xC0000139). Link the same resource into
    // test targets so `cargo test` runs on Windows.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is set by cargo");
        let resource = std::path::Path::new(&out_dir).join("resource.lib");
        if resource.exists() {
            println!("cargo:rustc-link-arg-tests={}", resource.display());
        }
    }
}
