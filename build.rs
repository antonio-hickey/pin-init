use rustc_version::{version_meta, Channel, Version};

fn main() {
    println!("cargo::rustc-check-cfg=cfg(RUSTC_USE_FEATURE)");
    println!("cargo::rustc-check-cfg=cfg(CONFIG_RUSTC_HAS_UNSAFE_PINNED)");

    let meta = version_meta().unwrap();

    let use_feature = meta.channel == Channel::Nightly || std::env::var("RUSTC_BOOTSTRAP").is_ok();
    if use_feature {
        println!("cargo:rustc-cfg=RUSTC_USE_FEATURE");
    }

    if meta.semver >= Version::parse("1.89.0-nightly").unwrap() && use_feature {
        println!("cargo:rustc-cfg=CONFIG_RUSTC_HAS_UNSAFE_PINNED");
    }
}
