# C11.4 anonymous Grid text `white-space:pre` 구현/runtime 실패 관점 검토 20개

계획 검토 20개와 분리해 현재 구현, fixture 경계, Android/iOS runtime 관찰을 검토했다. 일반 CSS 지원·전체 C11.4·glyph paint는 범위 밖으로 판정했다.

1. **공백 보존 mode 분리 — PASS.** `WhiteSpaceMode::Pre`는 `NoWrap`, `PreLine`, `PreWrap`, `BreakSpaces`와 독립 variant다. 인접 mode 의미를 재사용하지 않는다. 근거: `crates/spinon-text-layout/src/model.rs`, `engine/intrinsic.rs`.
2. **soft-wrap 금지 — PASS.** `Pre`는 `TextWrapMode::NoWrap`으로 변환된다. 고정 폭 final layout에서도 한 줄 text advance가 root 폭을 넘는 조건을 unit test로 검사했다. 근거: `engine/intrinsic.rs`, `tests/c11_4_pre.rs`.
3. **반복 공백 보존 — PASS.** normalization은 `Pre`의 공백을 collapse하지 않으며 `XX  XX` intrinsic width 60px를 검사한다. 근거: `normalize.rs`, `tests/c11_4_pre.rs`, HostDocument max-content test.
4. **space 경계 min-content — PASS.** `XX XX`의 min/max contribution이 각 50px이고 soft wrap이 생기지 않는 값을 text engine과 Grid host test에서 확인했다.
5. **forced segment break — PASS.** LF와 form-feed는 보존 mode에서 forced line break로 정규화하며 두 줄의 20px advance를 검사했다. 근거: `normalize.rs`, `tests/c11_4_pre.rs`.
6. **CRLF source mapping — PASS.** CRLF는 한 paragraph LF로 정규화하되 양 source code unit과 paragraph UTF-8 범위가 원본에 매핑되는지 test로 확인했다.
7. **tab 부분 처리 — PASS.** `Pre` tab 입력은 normalization 시작 시 구체적인 `UnsupportedPreTabProfile`로 거부하고 intrinsic/final layout 모두 부분 결과를 반환하지 않는다.
8. **HostDocument owner/tree integrity — PASS.** 익명 paragraph가 Grid parent의 direct text이고 owner, source node, unique source ordering, non-empty run 및 text profile이 맞아야 한다. 범위 밖 topology는 fail closed다. 근거: `crates/spinon-layout/src/compute.rs`.
9. **root HTML mapping 조합 — PASS.** `preserve + nowrap`만 `Pre`가 되며 `preserve + wrap`은 거부되고 `collapse + nowrap`은 기존 `NoWrap`을 유지한다. 전용 runtime unit test로 확인했다.
10. **computed text profile gate — PASS.** runtime validator는 Ahem 10px/10px, weight/style, direction, line-break, spacing 등 고정 longhand와 `white-space-collapse`/`text-wrap-mode`를 검사한다. 일반 font/style 입력으로 확장되지 않는다.
11. **scene source 고정성 — PASS.** FFI source matrix는 고정 scene ID 31개와 source/content/style tuple을 검사한다. 신규 `pre` 장면은 다섯 개이며 추가 source를 임의로 받아들이지 않는다.
12. **max-content track — PASS.** `XX  XX`가 `[60]` column, `[10]` row를 만드는 기대값을 Android/iOS native validator와 Rust host test가 검사한다.
13. **min-content track — PASS.** `XX XX`가 soft-wrap 없이 `[50]` column, `[10]` row를 만드는 기대값을 양 플랫폼 validator가 검사한다.
14. **forced-break track — PASS.** `XX\nXX`가 `[20]` column, `[20]` row와 두 줄 text contribution을 만드는 기대값을 양 플랫폼에서 검사한다.
15. **fixed-width control 분리 — PASS.** Normal과 Pre fixed-width scene 모두 25px item/root frame을 보존하면서 row height와 text intrinsic width를 따로 확인한다. Pre text line advance는 50px이고 clipping/실제 glyph pixels를 주장하지 않는다.
16. **Android scene/trace 검사 — PASS.** API 37 ARM64 AVD의 cold-boot 최종 source 행렬은 31/31 scene의 root/item frame, tracks, trace chunk completion, draw/present와 retry 0을 기록했다. [Android 결과](android-avd-c114-anonymous-grid-text-android-avd-20261030T225732284Z-090ad9bf/result.json).
17. **iOS scene/trace 검사 — PASS.** iPhone 17 Pro/iOS 26.2 Simulator의 동일 source 행렬도 31/31을 통과했고 같은 geometry/track/trace/draw-presentation 필드를 검사했다. [iOS 결과](ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T225830613Z-41a55d90/result.json).
18. **기존 mode 회귀 — PASS.** 두 행렬의 26개 control/regression scene이 함께 통과해 normal, nowrap, pre-line, pre-wrap, break-spaces 관찰이 신규 mapping에 의해 바뀌지 않았다.
19. **opt-in/API 경계 — PASS.** fixture module boundary test는 31 scene catalog와 Android/iOS validators를 확인하고, user-facing CSS admission이나 public ABI 확장을 검사하지 않는다. [boundary test](../../../../tools/css-reference/fixture-module-boundary.test.mjs).
20. **binary boundary와 증거 범위 — PASS.** macOS ARM64 static archive의 default build에는 신규 marker 0/5 및 host export 0개, fixture positive control에는 marker 5/5 및 export 2개다. Android는 software-rendered AVD, iOS 캡처는 compositor 제출 전이며 현재 app path에서 C15.3 glyph paint가 꺼져 있어 glyph ink pixel equality는 증명하지 않는다. [archive 결과](artifact-boundary/README.md).

검사한 focused test와 전체 workspace Clippy/rustfmt 결과는 상위 [runtime evidence](README.md)에 정리한다. `cargo test --workspace`나 Spinon WPT 적합성 결과를 이 검토에 포함하지 않는다.
