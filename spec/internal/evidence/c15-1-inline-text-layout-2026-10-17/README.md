# C15.1 Rust text shaping/layout 내부 slice

이 evidence는 branch `feat/c15-1-inline-text-layout-20261017`에서 만든 내부 Rust slice의 범위와 검증을 기록한다. C15/C16/C19 상태를 완료로 올리지 않는다.

## 구현 범위

- 새 `spinon-text-layout` crate가 고정 Parley `0.12.0`으로 실제 shaping, line breaks, line/font metrics, glyph ID/origin/advance, UTF-16 source map과 inline fragment rect를 만든다.
- `white-space: normal | nowrap | pre-line`, `word-break: normal | break-all`의 고정 profile을 다룬다. collapsed/suppressed whitespace와 CRLF source 범위를 보존하며 줄 끝 hanging space는 visible fragment 폭에 포함하지 않는다.
- `spinon-layout`의 HostDocument 경로가 visible paragraph의 direct text와 plain one-level HTML span을 source order대로 모은다. 숨김 subtree는 제외한다. nested/decorated span, visible non-span child, invalid UTF-16, definite width가 아닌 측정은 fail closed다.
- Taffy measure와 final content width가 bit-exact하게 같아야 한다. `LayoutTextSnapshot`은 `LayoutInputRevision`, 원본 `ParagraphInput`, shaped `TextLayoutSnapshot`을 함께 갖는다.

## 검증

- `cargo test --locked -p spinon-text-layout`: 10 passed (`text-layout-tests.log`).
- `cargo test --locked -p spinon-layout`: 144 passed, 0 failed (`spinon-layout-tests.log`).
- `cargo test --locked --workspace`: 937 passed, 4 ignored, 0 failed (`workspace-tests.log`).
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `cargo build --locked -p spinon-layout --lib`: 모두 통과 (`clippy.log`, `rustfmt.log`, `cargo-build.log`).
- 기본 library artifact와 test positive control (`artifact-boundary.log`): default `spinon-layout` rlib에는 Ahem/Roboto 전체 payload가 없고, `spinon-text-layout` test binary에는 둘 다 들어 있다.
- `aarch64-linux-android` target compile (`android-target-check.log`)과 `aarch64-apple-ios-sim` target compile (`ios-simulator-target-check.log`)은 통과했다. 이는 target compile 확인이며 simulator 앱 실행이나 화면 검증은 아니다.
- Ahem, Roboto variable, OFL fixture SHA는 [C15 사전 비교 자료](../c15-inline-text-precomparison-2026-10-17/README.md)에 고정했다. 새 fixture copy를 만들지 않고 저장소의 기존 `tests/fixtures/resources/c15`를 사용한다.

## 구현 적대 검토에서 수정한 경계

- 기존 같은 family가 registry에 있을 때 다른 font bytes의 잘못된 등록이 성공할 수 있는 경로를 발견했다. 등록 후보 collection에 먼저 반영하고 새 등록에서 요청 family가 확인된 경우에만 live registry와 revision을 교체하도록 고쳤다. malformed font와 revision overflow도 상태를 바꾸지 않으며 `font_registration_is_transactional_and_checks_the_new_font_family`가 이를 확인한다.
- 일반 `LayoutStyle::default()`는 `display:flex`이므로 이를 plain inline span으로 간주하면 flex span을 paragraph text로 평탄화할 수 있었다. HostDocument용 `LayoutDisplay::Inline`을 명시해야만 plain span으로 받고, 기본 Flex 및 기타 visible display를 거부한다. 독립 `LayoutInput` box의 Inline도 거부한다.
- `0px` definite width가 유효한 Taffy measure임을 Chrome DOM 기준과 대조하고 unbreakable glyph advance를 parent box 바깥까지 보존했다. 음수·비유한 폭은 거부하고 `-0.0`은 `+0.0`으로 정규화한다.

[구현 실패 관점 20개 검토](./implementation-review-20.md)는 계획 검토와 다른 코드 경계·실패 입력을 다룬다. PR 전체 변경 검토 30개는 PR 생성 뒤 별도 파일로 기록한다.

## 브라우저 관찰과 남은 경계

꾸밈 없는 `XX XX` span의 Chrome reference는 [Agent Browser 관찰](../c15-inline-text-precomparison-2026-10-17/plain-inline-fragment-observation.md)에 있다. Chrome 155.0.8059.85 / Chromium `0e66b94549a8bd85f03fbdf189b752d6e7b63c32`, macOS 26.5.1, 320×480 CSS px, DPR 1에서 두 rect가 각각 20×10px였다. 이건 Chrome DOM reference이며 Spinon의 UI 화면은 아니다.

`spinon-style-to-layout`는 여전히 text node를 거부하고 `spinon-style-to-render` runtime/adapter/stacking도 text node를 거부한다. 따라서 이 Rust slice는 앱 runtime에 연결되지 않았다. Android API 37 AVD/iOS Simulator 앱, glyph painting, border 바깥 pixel/clip, 실기기는 실행하지 않았다. 다음 단계는 style cascade→paragraph profile 연결과 renderer 경계를 별도 slice로 정리하는 것이며, 이 결과로 C15.1/C15 전체를 완료 처리하지 않는다.

구현 변경에서 발견한 family 등록 오판, 기본 Flex span의 inline 평탄화, 0px 폭 경계와 수정 내용은 [구현 실패 관점 20개 검토](./implementation-review-20.md)에 기록했다. PR 생성 뒤 diff 전용 검토 30개를 별도 추가한다.
