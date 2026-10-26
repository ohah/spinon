# C11.4 generic direct-text collector 구현 test 관찰

**범위:** 이 문서의 초기 fixture-test 관찰은 기준 `HEAD` `040a5360be92f73e141797aa7dcd4ef00ab24942`의 historical candidate 기록이다. 그때의 Rust unit test 결과는 app build/runtime을 포함하지 않았다. 현재 app runtime follow-up은 아래 마지막 절과 [runtime evidence](../c11-4-generic-direct-text-intrinsic-runtime-2026-10-26/README.md)를 참조한다.

## 수정 전 관찰

수정 전 focused command는 `7 passed; 1 failed`였다. 유일한 실패는 계획에 있던 `hyphens:none`의 `UnsupportedInlineAuthorProperty` 예상이다. Stylo 0.22 parser는 해당 declaration을 `UnknownProperty` diagnostic으로 기록한 뒤 `ComputedStyleSnapshot`을 반환한다. 같은 run에서 `visibility:collapse`와 `font-variant-ligatures:none`은 각각 정확한 item `NodeId`와 property를 가진 `CssCascadeError::UnsupportedInlineAuthorProperty`를 반환했다. `hyphens`를 기존 preflight에 강제로 포함시키는 코드는 추가하지 않았다.

## 범위 보정 후 검증

재현 명령:

```sh
cargo fmt --all
cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test generic_direct_text
```

결과: **9 passed; 0 failed; 236 filtered out**. 테스트는 다음을 고정한다.

- 여섯 scene 각각에서 실제 `compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets` 결과를 사용해 computed CSS→`ParagraphStyle` 변환, raw intrinsic sizes, contribution candidates, Ahem registry identity, final `StyleLayoutOutput.layout` C15 geometry를 대조했다.
- anonymous/direct-descendant/mixed-image/foreign-namespace/multiple-text 및 3개 이상 text candidate source를 fallback 없이 거부했다. unsupported item/grid styles, non-grid/legacy fallback selection, textless block sibling, 다른 HostDocument의 stale computed snapshot도 확인했다.
- `visibility:collapse`와 `font-variant-ligatures:none` inline controls는 cascade snapshot 반환 전에 exact `UnsupportedInlineAuthorProperty`로 실패한다.
- `hyphens:none`은 별도 parser observation으로만 두며 `UnknownProperty` diagnostic 및 cascade snapshot을 assert한다. 이것은 C11.4 rejection error나 hyphens 계산 지원을 입증하지 않는다.
- 동일 numeric font registry revision을 가진 독립 TextLayoutEngine의 opaque token이 같다고 취급되지 않는 것을 supported Ahem registration API로 확인했다.

이 결과는 local Rust fixture test다. Agent Browser 재실행, simulator/app build/install, renderer·glyph paint, WPT, remote PR/CI는 포함하지 않는다. 공식 C11.4 상태와 `72/213` 집계는 바꾸지 않는다.

## 추가 회귀·정적 검사

- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114`: **19 passed; 0 failed**.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c11_4`: **30 passed; 0 failed**. 이 범위에는 기존 13개 `TEXT_CASES` matrix 고유성 검사와 generic direct-text test가 포함된다.
- `cargo fmt --all -- --check`: 통과.
- `cargo clippy --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test --all-targets -- -D warnings`: 통과.

위 검사는 로컬 Rust fixture/runtime test와 정적 검사다. simulator, app artifact, PR 또는 CI 검증을 대신하지 않는다.

## 2026-10-26 책임별 test module 분리 후 재검증

`generic_direct_text_tests.rs`의 기존 테스트를 다음 모듈로 재배치했다. Assertion와 fixture 값은 변경하지 않았다.

- `generic_direct_text_tests.rs`: 8-line thin module root, child module declarations only.
- `generic_direct_text_tests/cases.rs`: scene expectation/case data, 293 lines.
- `generic_direct_text_tests/test_fixture.rs`: HostDocument/Stylo builders and shared assertion helpers, 264 lines.
- `generic_direct_text_tests/positive.rs`: six-scene collector/output test, 263 lines.
- `generic_direct_text_tests/negative.rs`: source/style/parser/fallback controls, 314 lines.

재실행 명령과 결과:

- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test generic_direct_text`: **9 passed; 0 failed**.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c11_4`: **30 passed; 0 failed**. 이 selector는 13개 `TEXT_CASES` matrix 고유성 검사를 포함하지만 13개 각 scene의 runtime fallback 성공은 증명하지 않는다.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114`: **19 passed; 0 failed**.
- `cargo fmt --all -- --check`: 통과.
- `cargo clippy --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test --all-targets -- -D warnings`: 통과.
- `git diff --check`: 통과.

## 2026-10-26 rebase 후 app/runtime 후속

Rust focused test는 현재 code에 대해 `generic_direct_text` 9/9 통과했고, FFI scene source test는 1/1 통과했다. `c11_4` selector는 새 reject controls를 포함해 34 passed, `c114`는 19 passed다. `cargo fmt --all -- --check`, targeted runtime Clippy와 `git diff --check`도 통과했다.

Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 fixture-test Debug app을 빌드하고 canonical `generic-root-normal`을 설치·실행했다. 양쪽에서 layout ready/3 boxes, exact frames, direct Text inputs, Columns/Rows trace 및 draw/present가 통과했다. 이는 한 앱 장면의 evidence이며 나머지 다섯 scene과 negative controls는 Rust tests에 머문다. PNGs는 glyph paint나 pixel equality의 증거가 아니다. 앱 빌드와 두 simulator 로그·screenshots는 [runtime report](../c11-4-generic-direct-text-intrinsic-runtime-2026-10-26/README.md)에 연결했다. PR/CI와 Stage A 전체 acceptance는 별도 남아 있고 C11.4 공식 `72/213`은 그대로다.

이 검증은 Rust runtime/fixture test만 포함한다. 앱 FFI scene과 Android/iOS allowlist는 아직 연결되지 않았고 simulator/app artifact를 검증하지 않았다. shared FFI 및 C0410 app 파일은 image lane의 merge/rebase 이후 통합한다. 공식 C11.4 상태와 `72/213`은 유지한다.
