# C11.4 parsed Root HTML auto-row `row-gap` 제한 구현 후보

**상태:** 제한 구현 candidate. C11.4/C11 완료 체크와 공식 `78/214`는 유지한다.

## 변경 범위

기존 `RootHtmlAutoRowStretchV1` 고정 shape에서 computed finite non-negative px `row-gap`을 `0..=7.5px`로 제한해 허용한다. `column-gap`은 0px 그대로다. 대상은 하나의 40×40px Grid, 40px explicit column, 10px explicit row, auto implicit rows 두 개, fixed empty item 하나, direct text `A`/`B`, `align-items:normal` 및 앞선 `align-content` 10개 allowlist다. 새 CSS profile, public API, FFI 또는 일반 author CSS admission은 추가하지 않았다.

기준 입력과 독립 기대값은 [Chrome 157 40-scene precomparison](../c11-root-html-auto-row-content-alignment-gap-precomparison-2026-11-07/README.md)에 있다. row gaps `[0,2.5,5,7.5]px`에서 정렬 전 free space는 `[10,5,0,-5]px`다. `end`는 첫 row y=-5px, `center`는 y=-2.5px로 Grid 상단 밖에 놓인다. 40-scene RuntimeSession 검사는 Chrome/독립 기대 JSON의 item y와 used rows를 직접 사용하고, Layout 검사는 gutter 계산·track trace·frame geometry를 별도로 확인한다.

지원 밖 percentage, `calc()`, negative gap, 7.5px 초과, non-zero `column-gap`은 layout을 publish하지 않고 거부한다. 거부 후 partial layout이 없는 것도 확인했다.

## Android/iOS Simulator 실행

Android API 37 ARM64 AVD (`emulator-5554`, ANGLE/Vulkan/SwiftShader)와 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 Root HTML app route를 같은 scene HTML로 실행했다. 각 pass에서 Android APK와 iOS app의 embedded `index.html` SHA가 source fixture SHA와 일치한다. 네 pass 모두 `layout=ready`, 6 boxes, 두 runtime text records, draw/present 및 설치 앱·빌드 산출물 복원 결과 `PASS`다.

| Scene | Source SHA-256 | Android screenshot | iOS screenshot |
| --- | --- | --- | --- |
| `normal`, gap `0px` | `e50a131376fc3c33fffeca30f57a8591778b3c39e4f0778c2d159f988fba76d3` | [pass01](pass01/android/simulator.png) | [pass01](pass01/ios/simulator.png) |
| `space-between`, gap `2.5px` | `89edf9742b8f15d8615069c91c8f92107212c5b7ddce940a0e4229b4013961a3` | [pass02](pass02/android/simulator.png) | [pass02](pass02/ios/simulator.png) |
| `end`, gap `7.5px` | `5c345b5f9b7add464751df994cdbd28aa7fa9f9d02c30e43e0d5da7d2e2a7745` | [pass03](pass03/android/simulator.png) | [pass03](pass03/ios/simulator.png) |
| `center`, gap `7.5px` | `82b1547d5aad97a306c1deea82e324afa2c0439ce64910306c6f3bc9a899bdfb` | [pass04](pass04/android/simulator.png) | [pass04](pass04/ios/simulator.png) |

Screenshots are visual smoke. The negative y positions are proven by Rust layout/runtime assertions; the top edge of each simulator viewport clips the part above the Grid. Android PixelCopy and iOS Metal drawable readback were not taken, so these captures are not surface-pixel parity evidence. Each run's app build, runtime, install, screenshot and restoration logs are under `pass01/` through `pass04/`. Original Xcode/runtime and focused Rust test logs with tool-generated trailing whitespace are gzip-compressed byte-for-byte; `gzip -dc <file>.gz` reproduces each original. The previous Android installed APK hash `48db6dd5748c5bb7717d20ce4b3bc023ebe93acb1dc202b3d973164544c5bc04` and iOS executable hash `eca866e291d5ffbbe7756c354532bb69bca9d2fed264bc8181d02c61a772a929` were restored after every run. No simulator app or Agent Browser process remains running.

`run-simulators.sh` is the restoration-guarded harness; `prepare-fixtures.mjs` creates the four fixed sources from the prior merged zero-gap fixture; `verify-simulator-evidence.mjs` checks embedded source hashes, runtime markers, screenshots and restoration records. The capture builds enable the C04 runtime GPU path and disable all C11 fixture features.

## Rust verification

| Gate | Result |
| --- | ---: |
| Style projection finite-px boundary unit test | 1/1 |
| Layout gap × alignment matrix | 40 scenes in 1/1 focused test |
| RuntimeSession oracle-backed gap × alignment matrix | 40 scenes in 1/1 focused test |
| Unsupported gaps and atomic layout rejection | 1/1 focused test |
| Recovery after a rejected alignment | 1/1 focused test |
| Changed crates all-target Clippy, `-D warnings` | pass |
| `cargo fmt --all -- --check` | pass |
| Simulator evidence verifier | 4/4 pass |

Raw focused command outputs are in [`verification/`](verification/). The default product app used no C11 fixture feature or new FFI. This bounded slice does not claim PixelCopy/Metal surface equality, physical-device or hardware-GPU coverage, WPT conformance, general Grid support or C11.4/C11 completion.
