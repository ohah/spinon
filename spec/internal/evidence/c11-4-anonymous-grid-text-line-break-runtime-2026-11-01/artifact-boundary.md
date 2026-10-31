# C11.4 computed `line-break` FFI archive boundary

The two release archives were built from the same checkout with Rust 1.96.1
for the default host target. The first used no features; the second enabled
only the existing `c04-runtime-gpu-fixture-test` feature.

| Build | Archive SHA-256 | C11.4 fixture host symbol | New scene marker | Full Noto Sans KR bytes |
| --- | --- | --- | --- | --- |
| `cargo build --locked --release -p spinon-ffi --lib` | `428755b886d8eab1c1a94a24449b4a566c18261b80e776eb72bd2654957c9d25` | absent | absent | absent |
| `cargo build --locked --release -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` | `309e18b0d73e933e140605f3797d4dafae6b2b09b0386953a89213c679b85f8b` | present | present | present |

The symbol was checked with the `llvm-nm` shipped in Rust 1.96.1's
`llvm-tools` component. `strings` found
`line-break-ja-normal-min-content` only in the fixture archive. A byte search
for `tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf` (SHA-256
`9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76`)
found the full 10,415,420-byte font at offset 12,626,914 in the fixture
archive and no match in the default archive.

This proves the static FFI archive boundary for these artifacts. It does not
claim that a complete Android APK or iOS app bundle excludes every fixture
asset under all build configurations. The temporary archives were copied to
`/tmp` for comparison and are not repository artifacts.
