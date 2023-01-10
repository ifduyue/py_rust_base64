fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CFLAGS");
    println!("cargo:rerun-if-env-changed=LDFLAGS");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");
    println!("cargo:rerun-if-env-changed=ORJSON_DISABLE_YYJSON");

    let py_cfg = pyo3_build_config::get();
    py_cfg.emit_pyo3_cfgs();
    pyo3_build_config::add_extension_module_link_args();
}

