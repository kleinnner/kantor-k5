# KANTOR K5 — Post-Quantum Archive Hash (Anticloud FZ LLE)

`K5 = SHA3-256(archive_sha3 ‖ size_le64 ‖ project_name_utf8 ‖ NULL ‖ timestamp_iso)` — USPTO filing in progress.

- `SPEC.md` — full specification · `CRYPTANALYSIS.md` · `SECURITY.md`
- `k5-py/` — Python reference · `k5-core/` — Rust core (`cargo publish` twin on crates.io)

Every `*.k5hash` sidecar across the 862-project corpus was produced by this construction. Verify any archive independently with the formula above.

Apache-2.0 + Enterprise commercial dual (Anticommons 0.1.0). Issues/discussions disabled — read-only mirror; contact lois@0-1.gg.
