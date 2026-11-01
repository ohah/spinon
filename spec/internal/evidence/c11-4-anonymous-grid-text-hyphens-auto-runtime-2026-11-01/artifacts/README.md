# `hyphens:auto` release archive feature boundary

Both archives were built on macOS ARM64 with `--release --no-default-features`; only the Cargo feature changed. The default/product command was:

```sh
cargo build --locked -p spinon-ffi --release --no-default-features --features c04-runtime-gpu
```

The fixture positive-control command was:

```sh
cargo build --locked -p spinon-ffi --release --no-default-features --features c04-runtime-gpu-fixture-test
```

| Build | Size | SHA-256 | Archive observation |
| --- | ---: | --- | --- |
| Product `c04-runtime-gpu` | 72 MiB | `103001c4faecdba456a340c1aa72a189345532586d7f383a915e10f231cf856c` | `nm -g` and `strings` had no `hypher`, `auto_hyphen`, `hyphenation`, `internationalization`, or `representation` matches. `cargo tree -i hypher` reported no matching package in this feature graph. |
| Fixture `c04-runtime-gpu-fixture-test` | 88 MiB | `5b0e78d315af9d19ad8e10e4731918cfcb35640cbd8c45df9ed16dc6d8b73376` | Positive controls include `hypher::hyphenate`, `spinon_text_layout::normalize::auto_hyphenated_source`, and the three allowlist scene/text strings. `cargo tree -i hypher` resolves exact `hypher v0.1.8` through the fixture feature. |

This verifies only the macOS ARM64 Rust static archives. It does not establish full APK/IPA fixture exclusion or release-bundle size.
