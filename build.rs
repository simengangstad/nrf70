use std::{fs::File, path::PathBuf};

fn main() {
    let nrf_wifi_path = PathBuf::from("thirdparty/nrf_wifi")
        .canonicalize()
        .expect("Cannot canonicalize path");

    let bindings = bindgen::Builder::default()
        .header("thirdparty/wrapper.h")
        .use_core()
        .ignore_functions()
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .prepend_enum_name(false)
        .layout_tests(false)
        .clang_arg("-DNRF70_STA_MODE")
        .clang_arg(format!("-I{}", nrf_wifi_path.join("fw_if/umac_if/inc").display()))
        .clang_arg(format!("-I{}", nrf_wifi_path.join("fw_if/umac_if/inc/fw").display()))
        .clang_arg(format!(
            "-I{}",
            nrf_wifi_path.join("fw_if/umac_if/inc/common").display()
        ))
        .clang_arg(format!(
            "-I{}",
            nrf_wifi_path.join("fw_if/umac_if/inc/system").display()
        ))
        .clang_arg(format!("-I{}", nrf_wifi_path.join("hw_if/hal/inc/").display()))
        .clang_arg(format!("-I{}", nrf_wifi_path.join("hw_if/hal/inc/common").display()))
        .clang_arg(format!("-I{}", nrf_wifi_path.join("hw_if/hal/inc/system").display()))
        .clang_arg(format!("-I{}", nrf_wifi_path.join("os_if/inc").display()))
        .clang_arg(format!("-I{}", nrf_wifi_path.join("bus_if/bal/inc").display()))
        .raw_line("#[cfg(feature = \"defmt\")]")
        .raw_line("use defmt::Format;")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("thirdparty").join("bindings.rs");
    File::create(&out_path).expect("Failed to create output file");
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}
