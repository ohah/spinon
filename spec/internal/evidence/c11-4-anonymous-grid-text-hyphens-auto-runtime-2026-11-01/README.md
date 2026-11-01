# C11.4 anonymous Grid text `hyphens:auto` candidate evidence

이 자료는 C11.4 `hyphens:auto`의 제한 opt-in fixture candidate를 기록한다. 일반 CSS author admission, 전체 영어 사전, WPT, glyph raster pixels 또는 C11 완료를 선언하지 않는다. C11.4/C11 checkbox와 공식 완료 `72/213`은 미완료 상태다.

## 고정 비교와 기대값

[구현 전 Chrome 156/Ahem 사전 비교](../c11-4-anonymous-grid-text-hyphens-auto-precomparison-2026-11-01/README.md)와 [구현 계획](../../../../plan/c11-4-anonymous-grid-text-hyphens-auto.md)이 입력 범위와 independent oracle을 정한다. 구현은 exact `en`, Ahem 10px direct text의 세 allowlist 단어만 Hypher `0.1.8` 영어 patterns로 분절한다.

| 원문 | Hypher segment | min-content | max-content | 50px 줄 advance |
| --- | --- | ---: | ---: | --- |
| `hyphenation` | `hy` / `phen` / `ation` | 50px | 110px | 30, 50, 50px |
| `internationalization` | `in` / `ter` / `na` / `tion` / `al` / `iza` / `tion` | 50px | 200px | 30, 40, 30, 50, 30, 40, 40px |
| `representation` | `rep` / `re` / `sen` / `ta` / `tion` | 40px | 140px | 40, 30, 40, 30, 40px |

자동 기회 U+00AD는 normalized layout text에만 들어간다. source text는 원문을 유지하고 marker의 UTF-16 range는 길이 0이다. Simulator capture runner는 모든 normalized source range와 원문 text를 검사한다. CSS Text 3은 특정 사전의 breakpoint를 정하지 않으므로 Chrome 관측과 Hypher 기대값은 별도 oracle이다.

## Rust 및 feature 경계

- `spinon-text-layout` feature tests: 96 passed.
- `spinon-style` C11.4 hyphens cascade/CSS adapter tests: 7 passed.
- `spinon-runtime` fixture-enabled package suite: 394 passed, 3 ignored.
- `spinon-ffi` fixture-enabled package suite: 188 passed, 2 ignored.
- Fixture module boundary checks: 32 passed. `cargo fmt --all -- --check`, Node syntax check 및 fixture-enabled FFI Clippy (`-D warnings`) 통과.
- [Archive feature 경계 근거](artifacts/README.md)는 기본 product FFI release archive의 dependency/symbol/string 음성 대조와 fixture-test positive control을 기록한다. Archive 검사는 APK/IPA 전체 exclusion 검사가 아니다.

## Simulator 행렬

두 플랫폼의 각 matrix는 기존 `none|manual|initial` 회귀와 `auto` 9개 positive scene, Auto source와 잘못된 keyword expected rejection을 합친 30개다. 두 실행 모두 computed style, document revision, raw source, normalized UTF-16/UTF-8 source mapping, intrinsic min/max, line advance, frame, Columns/Rows 최종 track을 비교했다.

| Target | 결과 | 설치 artifact 일치 | 고정 증거 |
| --- | ---: | --- | --- |
| Android API 37 ARM64 AVD, `emulator-5556`, `sdk_gphone64_arm64` | 30/30 | APK/build SHA-256 `4b3076be328c421ad6b38535e426de442593ec656c816b6874cc80487c28c9dd` | [matrix](android-avd/c114-hyphens-android-avd-1793528793851-6f8a5878/matrix.tsv), [metadata](android-avd/c114-hyphens-android-avd-1793528793851-6f8a5878/simulator.json), [screen](android-avd/c114-hyphens-android-avd-1793528793851-6f8a5878/representative-screen.png) |
| iPhone 17 Pro / iOS 26.2 Simulator, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB` | 30/30 | executable/build SHA-256 `05e34b5d5a83b7944cdfe00c2cd116dec43647e823f079e569981bd1ca2199b2` | [matrix](ios-simulator/c114-hyphens-ios-simulator-1793528793851-3309ec23/matrix.tsv), [metadata](ios-simulator/c114-hyphens-ios-simulator-1793528793851-3309ec23/simulator.json), [screen](ios-simulator/c114-hyphens-ios-simulator-1793528793851-3309ec23/representative-screen.png) |

The iOS build result and emitted warnings are summarized in [build-summary.md](ios-simulator/build-summary.md); the byte-preserved Xcode output is available as [build.log.gz](ios-simulator/build.log.gz). The AVD is a software-emulated simulator; this matrix is not physical-device or hardware-GPU evidence. The iOS screenshot is a Simulator capture and no compositor-after-presentation pixels were sampled.

## 수정 이력과 남은 범위

실행 중 두 개의 harness oracle 오류를 발견해 고쳤다. 첫째, `hyphens:initial`의 computed 기본값 `manual`을 runner가 `initial`로 잘못 분류했다. 둘째, `representation`의 50px track에서 text layout의 natural line width 40px와 track width 50px를 혼동했다. Android/iOS 기대값과 capture oracle을 수정하고 각각 전체 30-scene 행렬을 다시 통과시켰다. 초기 실패 로그는 별도 run 디렉터리에 보존했으며 최종 matrix 근거에는 포함하지 않았다.

`lang` 없음/`en-US`/`fr`, allowlist 밖 단어와 authored U+00AD는 text-layout fail-closed tests에서 별도로 확인한다. 임의 language/문장·inline run·다른 글꼴·글자 크기, spelling-changing fallback, 일반 CSS admission, WPT, glyph pixels, hardware GPU와 실기기는 포함하지 않는다. [구현/runtime 실패 관점 20개](implementation-runtime-review-20.md)와 [실제 PR diff 실패 관점 30개](pr-diff-review-30.md)는 완료했다. 구현 PR [#982](https://github.com/ohah/spinon/pull/982)은 2026-11-01T10:38:00Z Rebase and merge로 병합됐고 merge SHA는 `b54d4853fe7cd5c70393c7083eea2c3d8f6f223f`다. GitHub check rollup은 비어 있어 CI 통과로 세지 않는다. 이 제한 fixture candidate는 C11.4/C11 완료가 아니며 공식 집계 `72/213`을 유지한다.
