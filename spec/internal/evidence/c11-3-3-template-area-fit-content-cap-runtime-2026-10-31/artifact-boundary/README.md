# C11.3.3 fit-content cap fixture static archive boundary

- Scope: macOS `spinon-ffi` Rust static archive only; this does not establish APK/IPA or full mobile package exclusion.
- Default Release build: `cargo build --locked -p spinon-ffi --lib --release`; all three new scene markers were absent, archive SHA-256 `9d099e820a2e66a08acfb7177139d28977a856e7cb4ccd026f1131d46b4564bf` (60,299,504 bytes).
- Opt-in positive control: `cargo build --locked -p spinon-ffi --lib --release --features c04-runtime-gpu-fixture-test`; all three markers were present, archive SHA-256 `f93fc742a90402e17b2d4deb6d5f53e11b91f610258ce2f4ebbc2d5a6f883b22` (81,221,080 bytes).
- Source commit: `b815f483a253072cbbca1c0cd473d519fc95a220` (tree `8e74b832259990dc6cc3513a0a119efea9fafe8d`).
- [Source-bound default build log](default-source-bound-build.log) · [source-bound opt-in build log](opt-in-source-bound-build.log).
- [Machine-readable comparison](result.json) · [default build log](default-build.log) · [opt-in build log](opt-in-build.log).
