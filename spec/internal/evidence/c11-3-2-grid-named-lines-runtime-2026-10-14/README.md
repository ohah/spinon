# C11.3.2 named grid lines 후보 실행 근거 · 2026-10-14

후보 branch `feat/c11-3-2-named-grid-lines`, base `ed089f8` (`origin/main`). 이 실행은 named-line 제한 profile의 재현 자료다. 공개 CSS 지원이나 Spinon WPT 적합성 선언이 아니다.

## 검증 결과

- Rust workspace: `SPINON_V8_DIR=... cargo test --locked --workspace` — 679 passed·4 ignored·0 failed — [로그](./workspace-test.log).
- named-line projection 단위 검사 — [로그](./named-line-test.log); Stylo→layout integration 및 다음 style revision 검사 — [로그](./named-line-integration-test.log).
- CSS reference: 400/400 통과, skip 0 — [로그](./css-reference-full-test.log).
- named-line 전용 CSS 고정 fixture 6/6, 표면 pixel 비교기 5/5, fixture module boundary 5/5 — [CSS fixture](./c1132-css-focused-test.log), [pixel negative controls](./c1132-css-pixels-test.log), [feature boundary](./fixture-boundary-test.log).
- `spinon-ffi` 기본 feature: 18 통과 — [로그](./ffi-default-test.log).
- `spinon-ffi` fixture feature 조합 `c04-runtime-gpu-test-hooks,c11-2-3-grid-fixture-test,c11-grid-metal-surface-pixels-test`: 52 통과·2 ignored — [로그](./ffi-fixture-test.log).
- workspace Clippy와 fixture feature Clippy `-D warnings` 통과 — [workspace](./workspace-clippy.log), [fixture](./ffi-fixture-clippy.log).
- `cargo fmt --all -- --check`, `git diff --check` 통과 — [rustfmt](./rustfmt-check.log), [diff](./diff-check.log).

## 고정 기준과 모바일 runtime

- Chrome for Testing `154.0.8037.92`; reference는 15 scene·30 node, DPR 1·2다. [Chrome capture와 WPT oracle 기록](../c11-3-2-named-lines-precomparison-2026-10-14/README.md).
- pinned WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 named-line 후보는 Chrome에서 test 2개 PASS, unknown-line test의 17 subtests PASS, skip 0이다. 이는 Chrome oracle만 뜻한다.
- Android 16/API 36 SM-S731N 실기기 PixelCopy는 성공/기대 거부 각 1 scene, 4 HostDocument node, 성공 geometry `160×120` grid와 `80×40 @ (80,0)` item, RGBA 표본 2/2 exact다. [실행](./android-physical-c1132-android-physical-20261013T182250660Z-6c98e33d/README.md).
- iPhone 17 Pro/iOS 26.2 Simulator Metal drawable도 같은 scene/geometry 및 2/2 exact다. Pixel readback은 OS compositor 제출 전이다. 최신 source 재실행 [run](./ios-simulator-c1132-ios-simulator-20261013T214737423Z-a26930a7/README.md)과 이전 올바른 실행 [run](./ios-simulator-c1132-ios-simulator-20261013T183654060Z-61e173b6/README.md)을 보존했다.
- 양쪽 플랫폼의 좌표 allowlist 변조 negative control은 surface readback 전에 거부됐다. 실패한 초기 실행 로그는 원인과 함께 보존했다. 이 결과를 전체 15-scene 모바일 실행, iOS 실기기, compositor 이후 픽셀로 확대하지 않는다.
- iterator 기반 resolver 정리 뒤 iOS Simulator에서 현재 production source를 다시 빌드했다. 최신 run `c1132-ios-simulator-20261013T214737423Z-a26930a7`도 2/2 exact였고, 앱 SHA-256은 `6db1b7af7f2903c90b4097c8d6e09c4b250b9a5c5ef48210b6b5de0890eb77d9`다. [최신 iOS 실행](./ios-simulator-c1132-ios-simulator-20261013T214737423Z-a26930a7/README.md).
- Android 실기기는 이 후속 빌드 때 `adb devices -l`에 연결되어 있지 않았다. 기록된 SM-S731N PixelCopy run은 resolver의 동등한 index loop→iterator 구조 정리 전 실행이다. 현재 source의 named-line unit/integration/workspace 검사는 다시 통과했지만, Android physical PixelCopy를 최신 source로 재실행하지 못한 점은 남은 검증 경계다.

## Fixture 산출물 경계

[static library feature 대조](./fixture-artifact-boundary.log)는 macOS debug `spinon-ffi` static library만 검사한다. 기본 feature 빌드에서 C11.3.2 host symbol과 fixture payload는 없고, `c11-grid-metal-surface-pixels-test` positive control에서는 있다. release APK/IPA dead stripping, bundle size 또는 release binary는 이 로그의 범위가 아니다.

## 원본 자료

- [구현 실패 관점 20개](../c11-3-2-grid-named-lines-implementation-review-2026-10-14.md)
- [Chrome/WPT 기준과 scene 산출물](../c11-3-2-named-lines-precomparison-2026-10-14/README.md)
- [내부 계약 0059](../../0059-c11-grid-layout.md) · [C11.3 단계 계획](../../../../plan/c11-3-template-repeat-named-lines-areas.md)

## 로그 보존

추가된 텍스트 log에서 trailing whitespace와 중복 EOF blank line을 정규화해 diff 검사를 통과시켰다. 바이트 단위 원본은 같은 위치의 `.raw.log.gz`에 보존하고 SHA-256은 [manifest](./raw-log-sha256.txt)에 기록했다.
