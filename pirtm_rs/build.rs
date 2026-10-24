fn main() {
    println!("cargo::rustc-check-cfg=cfg(sovereign_core)");
    println!("cargo::rustc-check-cfg=cfg(sovereign_build_health_20260716)");
    println!("cargo:rustc-cfg=sovereign_core");
    println!("cargo:rustc-cfg=sovereign_build_health_20260716");
    println!("cargo:rustc-env=SPF_SOVEREIGN_BUILD_HEALTH=pirtm_rs");
    println!("cargo:rerun-if-changed=build.rs");
}
