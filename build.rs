use rustc_version::{version, Version};

fn main() {
    println!("cargo::rustc-check-cfg=cfg(RUSTC_LINT_REASONS_IS_STABLE)");
    println!("cargo::rustc-check-cfg=cfg(RUSTC_NEW_UNINIT_IS_STABLE)");
    println!("cargo::rustc-check-cfg=cfg(RUSTC_RAW_REF_OP_IS_STABLE)");
    println!("cargo::rustc-check-cfg=cfg(CONFIG_RUSTC_HAS_UNSAFE_PINNED)");
    if version().unwrap() >= Version::parse("1.81.0").unwrap()
        || version().unwrap() >= Version::parse("1.81.0-nightly").unwrap()
    {
        println!("cargo:rustc-cfg=RUSTC_LINT_REASONS_IS_STABLE");
    }
    if version().unwrap() >= Version::parse("1.82.0").unwrap() {
        println!("cargo:rustc-cfg=RUSTC_NEW_UNINIT_IS_STABLE");
    }
    if version().unwrap() >= Version::parse("1.89.0-nightly").unwrap() {
        println!("cargo:rustc-cfg=CONFIG_RUSTC_HAS_UNSAFE_PINNED");
    }
    if version().unwrap() >= Version::parse("1.82.0").unwrap() {
        println!("cargo:rustc-cfg=RUSTC_RAW_REF_OP_IS_STABLE");
    }
}
