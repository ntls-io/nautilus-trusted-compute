use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=SGX_SDK");
    println!("cargo:rerun-if-env-changed=SGX_MODE");

    let sdk_dir = env::var("SGX_SDK").unwrap_or_else(|_| "/opt/sgxsdk".to_string());
    let is_sim = env::var("SGX_MODE").unwrap_or_else(|_| "HW".to_string());

    link_enclave_u();

    println!("cargo:rustc-link-search=native={}/lib64", sdk_dir);
    match is_sim.as_ref() {
        "SW" => println!("cargo:rustc-link-lib=dylib=sgx_urts_sim"),
        "HW" => println!("cargo:rustc-link-lib=dylib=sgx_urts"),
        _ => println!("cargo:rustc-link-lib=dylib=sgx_urts"), // Treat undefined as HW
    }
}

/// Link the untrusted C EDL static library.
/// Fail with a helpful message if it does not exist.
fn link_enclave_u() {
    // Match LIB and NAME_U_D in the Makefile.
    const LIB: &str = "../build/lib";
    const NAME_U_D: &str = "libEnclave_u.a";

    // Resolve paths relative to the local Cargo.toml
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = PathBuf::from(cargo_dir).join(LIB);

    assert!(
        lib_dir.join(NAME_U_D).exists(),
        r#"
    Could not find the untrusted C EDL static library.
    Run "make {}/{}" or "make" to generate it.
"#,
        LIB,
        NAME_U_D,
    );

    std::println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=Enclave_u");
}
