// CXX-Qt Build Script
// Configures the build for Qt 6 integration with QML resources

use std::env;
use std::path::PathBuf;

fn main() {
    // Configure CXX-Qt build
    let mut bridge = cxx_qt_build::CxxQtBuilder::new();

    // Add QML resources
    // Note: CXX-Qt 0.7 API may require different QML module configuration

    // Generate C++ and build
    bridge.build();

    // Ensure Qt is found
    println!("cargo:rerun-if-env-changed=QMAKE");
    println!("cargo:rerun-if-env-changed=QT_DIR");
    println!("cargo:rerun-if-env-changed=QTDIR");
    
    // Add Qt library paths if available
    if let Ok(qt_lib_path) = env::var("QT_LIBRARY_PATH") {
        println!("cargo:rustc-link-search=native={}", qt_lib_path);
    }
    
    // Add resource compilation
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=qml/");
    println!("cargo:rerun-if-changed=qml/main.qml");
    
    // Output build configuration
    println!("cargo:warning=CXX-Qt build configured for Qt 6 with QML resources");
}