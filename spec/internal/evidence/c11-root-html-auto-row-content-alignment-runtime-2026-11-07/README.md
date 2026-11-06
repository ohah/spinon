# C11.4 Root HTML auto-row content alignment runtime candidate

**상태:** 제한 구현 candidate. C11.4와 C11 공식 완료 항목은 미완료이며 공식 집계 `78/214`를 유지한다.

## 구현 범위

고정된 parsed Root HTML Grid shape에서 computed `align-content` 10개 값을 연결한다: `normal`, `stretch`, `start`, `flex-start`, `end`, `flex-end`, `center`, `space-between`, `space-around`, `space-evenly`. 첫 두 값만 두 implicit auto rows에 free-space stretch를 적용하고, 나머지 8개는 row bases `[10,10,10]px`를 보존한 채 track group을 정렬한다. 새 positioning/distribution 값은 fixed empty item 하나와 direct text `A`, `B` 두 행의 정확한 세-item 형상에서만 허용한다.

새 CSS parser, public CSS admission, runtime profile, API 또는 FFI는 추가하지 않았다. 기존 single-text auto-row profile은 positional value를 계속 거부한다. `baseline`, safety modifier, unknown computed value 및 shorthand를 통한 `justify-content` 축 확장은 계속 거부한다.

## Geometry와 source 기준

Chrome 157 사전 관측과 독립 계산은 [precomparison evidence](../c11-root-html-auto-row-content-alignment-precomparison-2026-11-07/README.md)에 있다. 40×40px content box에는 10px explicit row와 10px씩인 두 implicit auto rows가 있고 free space는 10px이다. `normal|stretch`는 `[10,15,15]px` rows와 y `[0,10,25]`를 만든다. 위치·분배 값은 `[10,10,10]px` rows를 유지하며 각각 `start/flex-start [0,10,20]`, `end/flex-end [10,20,30]`, `center [5,15,25]`, `space-between [0,15,30]`, `space-around [5/3,15,85/3]`, `space-evenly [2.5,15,27.5]`의 child y 좌표를 만든다. `space-around`는 layout quantization 허용치 `1/32 CSS px` 안에서 비교한다.

Simulator fixture와 SHA-256:

- `start`: `align-content-start.html`, `8a4a16ac9ce7df1aefe91d563532c28e9a17091408aff05d86a027dd60ccad71`
- `space-between`: `align-content-space-between.html`, `cc431376dc18c17aa6f94da59e1250dcc837a67280272e680feddf5a6ee735f5`
- 두 platform에서 같은 scene의 fixture SHA가 일치한다. [source revision 기록](source-revision.txt)에는 simulator 실행 시 HEAD와 미커밋 Rust diff SHA를 따로 기록했다.

## Runtime 시뮬레이터 근거

Android API 37 ARM64 AVD (`emulator-5554`)와 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 Root HTML app route로 `start`와 `space-between`을 각각 실행했다. 네 실행 모두 `layout=ready`, `boxes=6`, `runtime_text_count=2`, `glyph_batch_draw_count=2`, draw/present를 기록했다. Geometry의 정확 판정은 별도 Rust layout/RuntimeSession assertions가 담당한다.

| Scene | Android screenshot | iOS screenshot | Android APK SHA-256 | iOS executable SHA-256 |
|---|---|---|---|---|
| `start` | [PNG](pass01/android/simulator.png), `7ab50d8da62f48012ea0014770709d4fb65fde9774cec24039a4345caf8747be` | [PNG](pass01/ios/simulator.png), `c6453a05be2a2c88f849b30ccddbe99fa41b4154aa9c9f9c82d9e0f2f4a6d26a` | `d30b98b8f9aa004c78f1ce32a78d9be548e8385cf8cee518b00b7bf95d2d8d38` | `0fc2bd740c0cb03654eb5c8141007ef165170c180b6825c9527c51e9d6c8d863` |
| `space-between` | [PNG](pass02/android/simulator.png), `2959b08bad19f39c236dd8a18bbd06d7a71c024b44a60deb2c7764dca5bd5942` | [PNG](pass02/ios/simulator.png), `2213ce49df4e95d29fd1e988015af38fe5e787ce45fe887bd0105df24c5048b1` | `9749301f977812650cd0d088a1d81227797f805993316906c256b1129e169650` | `0fc2bd740c0cb03654eb5c8141007ef165170c180b6825c9527c51e9d6c8d863` |

Screenshots are visual smoke evidence only. Android used the API 37 AVD software rendering path; no PixelCopy sample was taken. iOS is a Simulator capture and no post-compositor Metal readback was taken. These runs are not physical-device, hardware-GPU, WPT or pixel-parity evidence. The run script and raw logs are preserved under `pass01/`, `pass02/`; both report `asset_and_simulator_restoration=PASS`. The previously installed Android APK was restored at SHA-256 `48db6dd5748c5bb7717d20ce4b3bc023ebe93acb1dc202b3d973164544c5bc04`; the previous iOS executable was restored at SHA-256 `eca866e291d5ffbbe7756c354532bb69bca9d2fed264bc8181d02c61a772a929`.

## Rust verification

| Gate | Result |
|---|---:|
| `root_html_two_implicit_auto_rows_respect_supported_content_alignment` | 1/1 |
| Grid auto-placement tests | 92/92 |
| Root HTML multiple auto-row RuntimeSession | 5/5 |
| Existing single auto-row rejection/recovery | 3/3 |
| Root HTML auto-row group | 24/24 |
| Style computed-value allowlist | 1/1 |
| Changed-crate all-target Clippy with `-D warnings` | pass |
| `cargo fmt --all -- --check` | pass |

Each command's captured output is in [`verification/`](verification/). Xcode and selected test logs whose original whitespace triggers `git diff --check` are gzip-compressed without altering their bytes; `gzip -dc <file>.gz` reproduces the original log. The broad layout suite is non-green on both candidate and base HEAD: 468 passed and the unrelated C15 `host_document_collects_direct_text_and_inherited_spans_in_source_order` assertion failed. The broad style-to-layout suite is also non-green on both: candidate 435 passed and base HEAD 434 passed, each with the same 10 C07/C11 failing test names in aspect-ratio, existing content-alignment, grid-item-alignment, explicit-placement and percentage-gap tests. The pass-count difference is this change's one new allowlist unit test. The exact base runs are in `verification/base-layout-full.log` and `verification/base-style-full.log`; candidate runs are `verification/layout-full.log` and `verification/style-full.log`. These broad failures are not counted as passes and this change does not modify those assertions.

## Default artifact boundary

Release `spinon-runtime` and `spinon-ffi` artifacts were rebuilt with default product features. The new unit-test marker `parsed_root_html_positional_and_distributed_align_content_match_geometry` is absent from `libspinon_runtime.rlib` and `libspinon_ffi.a`, while present in the unit-test executable as a positive control. Exact paths, hashes and scan results are in [`source-revision.txt`](source-revision.txt). The simulator app builds had all C11 fixture flags disabled. This verifies this change did not add a product fixture ABI or ship its unit-test marker; it is not a package-size or release-bundle audit.

## Review records and remaining scope

- [Implementation/runtime failure review, 20 independent checks](implementation-runtime-review-20.md)
- [Implementation PR diff hostile review, 30 independent checks](pr-diff-review-30.md)

C11.4 as a whole, C11 completion, arbitrary CSS admission, non-zero gutters, overflow variants, other writing modes, WPT, physical devices and hardware GPU remain outside this candidate.
