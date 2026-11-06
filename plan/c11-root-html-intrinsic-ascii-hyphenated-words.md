# C11.4 parsed Root HTML ASCII 하이픈 단어 intrinsic sizing

**상태:** 구현 후보를 작업 중이다. direct-text 허용 문법과 normalized line-break marker가 들어갔고, Style-to-layout 12/12, RuntimeSession 기본/fixture-test 각 16/16, text-layout line-break 4/4와 hyphens:auto 3/3이 통과했다. Android·iOS Simulator, 최종 검토와 PR은 진행 중이다. C11.4/C11 체크 및 공식 완료 `78/214`는 유지한다.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 ASCII 영숫자 단어 slice](./c11-root-html-intrinsic-ascii-alphanumeric-words.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**사전 비교:** [Chrome 관측·독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-ascii-hyphenated-precomparison-2026-11-06/README.md) · [9-scene fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-ascii-hyphenated-precomparison.html) · [Agent Browser 화면](../spec/internal/evidence/c11-root-html-intrinsic-ascii-hyphenated-precomparison-2026-11-06/chrome-agent-browser.png)

**계획 실패 관점 검토:** [20개 항목](./c11-root-html-intrinsic-ascii-hyphenated-words-plan-review-20.md)

## 결손과 목표

기본 parsed Root HTML `grid-auto-rows:min-content` direct-text 경로는 ASCII 영숫자 단어와 접힌 ASCII whitespace를 허용하지만 U+002D HYPHEN-MINUS를 포함한 `A-A`, `12-3`, `AA-BB`는 거부한다. 이 후보는 같은 profile에서 token grammar만 `[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*`로 넓힌다. 각 하이픈은 비어 있지 않은 ASCII 영숫자 run 사이에만 올 수 있다.

기존 Parley line breaker는 U+002D 뒤 letter에서는 줄을 나누지만, letter/digit 뒤 U+002D 다음에 digit이 오는 ASCII source에서는 Chrome 157과 다른 intrinsic width를 냈다. 지원 profile에서만 normalized layout buffer에 U+200B를 넣어 그 break opportunity를 전달한다. 표식은 앞선 하이픈의 원 UTF-16 source range에 매핑하고, HostDocument Text·node identity·source order는 변경하지 않는다. 기존 Grid track-sizing 절차와 intrinsic contribution 경로는 그대로 사용한다.

## 규범 기준과 독립 oracle

- [CSS Text Level 3, 2026-08-14 CRD §5.2](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/)는 U+002D 뒤에 digit이 오는 경우 앞 codepoint가 letter/digit이 아니면 break 기회를 두지 않는 것이 상호운용에 바람직할 수 있다고 안내한다. 이는 UA 상호운용 guidance이며 보편적 conformance 보장은 아니다. 이 작업에서는 Chrome 157 관측값을 구체적인 line-break oracle로 쓴다. 후보 grammar도 하이픈 양쪽에 alphanumeric run을 요구해 leading minus를 제외한다.
- [CSS Grid Level 1, 2025-03-26 CRD §§11.4–11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)의 min-/max-content contribution과 track sizing 절차를 적용한다.
- [고정 사전 비교](../spec/internal/evidence/c11-root-html-intrinsic-ascii-hyphenated-precomparison-2026-11-06/README.md)는 Chrome 관측값과 Canvas advance를 사용한 독립 계산을 대조한다. 기대값 계산은 used Grid track/row를 읽지 않는다. 아홉 scene 모두 line count가 같고 폭·높이가 1/64 CSS px 단위 이내에서 일치한다.
- 이 Chrome 기준은 제한된 desktop oracle이며 WPT·제품 지원 범위의 증거가 아니다.

## 제한 profile과 거부 경계

1. 허용 source token은 `[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*`; token 사이와 source 앞뒤는 기존 U+0020 SPACE, TAB, LF, CR만 허용한다. 원문을 trim·normalize·rewrite하지 않는다.
2. 기존 `RuntimeRootHtmlTextPaintV1`의 visible direct `<div>` item, definite 40×40px Grid, 단일 10px explicit row, `grid-auto-rows:min-content`, monospace 10px/10px, `lang="en"`, normal whitespace/wrapping profile를 유지한다.
3. leading/trailing/repeated `-`, Unicode dash/minus, underscore·slash 등 다른 punctuation, NBSP, U+000B/U+000C, 빈/whitespace-only source, 다른 font/style/owner/topology는 거부한다. 거부는 새 revision의 일부 frame이나 draw를 publish하지 않아야 한다.
4. 각 Text node의 원문과 identity, source-order 역전, revision key, track contribution trace를 보존한다. Layout buffer에만 break 표식을 추가하고 원문 source 및 HostDocument text에는 쓰지 않는다.
5. 임의 CSS admission, 자동/언어별 hyphenation, soft hyphen, 다른 Unicode punctuation, WPT, 일반 text compatibility, 전체 C11.4/C11은 범위 밖이다.

## Acceptance

- 9-scene expected JSON을 style-to-layout과 RuntimeSession default 경로에서 재생한다. min/max/auto/fixed column, letter/digit 혼합, 여러 hyphen, whitespace, reverse order, line count, item frames, track contribution과 원문 Text identity를 확인한다.
- `-A`, `A-`, `A--B`, U+2010 HYPHEN, U+2212 MINUS SIGN, `A_B`, `A/B`를 negative control로 추가하고 기존 revision atomic rejection을 확인한다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 route에서는 oracle의 `12345678-12` scene을 실행해 하이픈 줄바꿈, layout-ready, draw/present와 DOM source 보존을 확인한다. 9-scene Rust 행렬에는 `AA-BB`와 `12-3`도 포함한다. 두 simulator는 같은 fixture SHA를 사용한다. 화면 캡처는 실행 smoke로 기술하며 PixelCopy/Metal sample 주장은 수집한 표본에 한정한다.
- 변경 crate focused tests, 기존 direct-text intrinsic regression, `cargo fmt --all -- --check`, changed-crate Clippy를 기록한다. fixture-test artifact 경계가 바뀌면 default artifact negative와 fixture-test positive control을 모두 확인한다. Base에서 재현되는 unrelated 실패는 별도 표기한다.
- 구현 뒤 새 구현/runtime 실패 관점 20개와 PR diff 적대 검토 20개를 작성한다. 이번 계획 PR의 diff도 별도 20개 관점으로 검토한다.

이 계획은 구현 완료나 C11 완료 판정이 아니다. 내부 계약 `0.1.0`을 유지한다.
