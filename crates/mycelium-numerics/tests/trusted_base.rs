//! Trusted-base memory-safety side-condition (RFC-0034 §13 clause (c), decomposition split).
//!
//! ADR-014 names `mycelium-core`/`-cert`/`-numerics`/`-vsa` as the trusted base carrying a
//! source-level `#![forbid(unsafe_code)]`. In the component-repo split each repo checks the
//! sources it carries (course-correction Phase B, 2026-07-18 — see the runtime repo's
//! `crates/mycelium-cert/tests/conformance.rs` delegation note). Never-silent guard (VR-5/G2).

#[test]
fn trusted_base_forbids_unsafe() {
    let crates_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/mycelium-numerics has a parent (crates/)");
    for krate in ["mycelium-numerics", "mycelium-vsa"] {
        let lib = crates_dir.join(krate).join("src/lib.rs");
        let src =
            std::fs::read_to_string(&lib).unwrap_or_else(|e| panic!("read {}: {e}", lib.display()));
        assert!(
            src.contains("#![forbid(unsafe_code)]"),
            "{krate}/src/lib.rs must carry `#![forbid(unsafe_code)]` — the checked basis for the \
             RFC-0034 §3.3 memory-safety clause (Proven). Its removal would un-ground the claim."
        );
    }
}
