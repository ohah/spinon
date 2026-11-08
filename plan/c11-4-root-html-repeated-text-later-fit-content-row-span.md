# C11.4 parsed Root HTML 반복 text row span의 후속 fit-content 위치

- **상태:** 제한 구현 후보 · 구현 PR [#1299](https://github.com/ohah/spinon/pull/1299) · C11.4/C11 공식 체크와 `78/214` 유지
- **선행 구현:** [반복 intrinsic row direct-text span](./c11-4-root-html-repeated-text-row-span.md) · PR [#1295](https://github.com/ohah/spinon/pull/1295)
- **사전 비교:** [Chrome 157 1,920 scene](../spec/internal/evidence/c11-4-root-html-repeated-text-later-fit-content-row-span-precomparison-2026-11-09/README.md)
- **계획 실패 관점 검토:** [서로 다른 20개](./c11-4-root-html-repeated-text-later-fit-content-row-span-plan-review-20.md)
- **구현/runtime 근거:** [기본 Root HTML Android·iOS smoke와 화면](../spec/internal/evidence/c11-4-root-html-repeated-text-later-fit-content-row-span-implementation-2026-11-09/README.md)
- **구현/runtime 검토:** [실패 관점 20개](../spec/internal/evidence/c11-4-root-html-repeated-text-later-fit-content-row-span-implementation-2026-11-09/implementation-runtime-review-20.md) · [PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-root-html-repeated-text-later-fit-content-row-span-implementation-2026-11-09/implementation-pr-diff-review-30.md)

## 결손과 목표

기본 parsed Root HTML의 제한 `RootHtmlFitContentAutoRowsV1`은 여섯-member `grid-auto-rows` list에서 `fit-content()`를 첫 번째 위치에 둘 때만 repeated direct-text span profile을 허용한다. CSS Grid의 track-size list에서는 같은 member 집합을 유지한 채 순서를 바꿀 수 있다. 기존 span topology는 첫 세 implicit row를 모두 사용하므로 두 번째·세 번째 list 위치에 놓인 `fit-content()`의 cap과 minimum floor가 겹치는 span sizing에 미치는 결과를 검증하고, 그 두 위치를 기본 Root HTML 경로에 추가한다.

이번 bounded slice는 기존 여섯-member 목록의 multiplicity를 고정한다: `fit-content(cap)` 한 개, `min-content` 두 개, `max-content` 두 개, `auto` 한 개. 나머지 다섯 member의 30개 고유 순서를 유지하고 `fit-content()` 위치만 2 또는 3으로 이동해 총 60개 새 track order를 허용한다. Item topology, text grammar, row-span geometry, Grid dimensions와 Root HTML session path는 PR #1295와 동일하다.

## 고정 입력과 독립 판정 모델

- 기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1` route에서 definite `40×220px` Grid, `40px` explicit column, `10px` explicit row, 0 gap, LTR, visible overflow를 사용한다. 첫 item은 row `1 / 2`, column `1 / 2`의 빈 item이다. 나머지는 모두 column `1 / 2`에 명시적으로 배치한다.
- `grid-auto-rows`는 위 여섯 member와 multiplicity만 받는다. `fit-content()`는 list의 CSS position 2 또는 3에 있고 나머지는 기존 30개 permutation 중 하나다. Cap은 `20px`, `20%`, `0%`, `120%`다. definite 220px content block size에서 percentage cap의 독립 계산값은 각각 44px, 0px, 264px다.
- `align-content`는 `start|stretch`다. Text content는 기존 grammar와 10px monospace/10px line-height를 그대로 사용한다. A/B/C line count는 정확히 2/3/4다. Topology는 span-2 forward/reversed와 span-2+span-3 forward/reversed 네 가지다.
- Chrome 157 Agent Browser 사전 비교는 `2 fit positions × 30 tails × 4 caps × 2 alignments × 4 topologies = 1,920 scenes`다. HTML fixture, full CSSOM/frame observation, screenshot, source hash 및 verifier는 위 evidence directory에 고정한다.
- CSS Grid 2025 CRD [§11.5 intrinsic track sizing](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)와 [§10.5 post-sizing alignment](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-align)을 규범 기준으로 쓴다. 독립 판정은 여섯-member authored order 보존, explicit row와 implicit list cycle 분리, row-span frame이 점유 track 크기의 합과 일치, source-order reversal 불변성, stretch가 사용 중인 `auto` track만 늘리는 관계다. `fit-content()`의 min-content floor가 cap보다 클 수 있으므로 used size가 항상 cap 이하라고 가정하지 않는다.

사전 비교에서 1,920개 scene 모두 정확히 explicit row 1개와 positive implicit row 3개를 만들었다. Span frame과 해당 row track 합의 최대 차이는 0 CSS px, semantic item의 source-order reversal frame 차이도 0 CSS px였다. Computed `grid-auto-rows`는 여섯-member authored order를 보존했다. 자세한 브라우저 관측은 독립 기대 관계의 대체물이 아니라 고정 입력 교차 확인이다.

## 지원 및 거부 경계

- 기존 첫 번째 `fit-content()` 위치의 30개 order는 유지하고, 두 번째·세 번째 위치의 60개 order만 추가한다.
- `fit-content()` 위치 4–6, 다른 list 길이/multiplicity, 누락·중복 keyword, 다른 cap 형태, indefinite percentage basis, 네 번째 implicit row 이상을 요구하는 placement, `grid-column:auto`, shorthand/negative/named line, 다른 span topology, Unicode·다른 font·text style 변형, nested/inline/replaced child, 추가 Grid owner, stale revision은 계속 거부한다.
- 일반 author CSS admission, 전체 HTML image loading/painting, 임의 Grid row span, WPT conformance, 전체 C11.4/C11 완료를 의미하지 않는다. 새 공개 API·FFI와 내부 계약 숫자 변경은 없다.

## 구현 및 검증 관문

1. 이 계획의 실패 관점 20개 및 계획 PR 실제 diff 30개 검토가 통과한 뒤에만 제품 코드를 바꾼다.
2. Style projection은 기존 profile과 topology를 유지하면서 `fit-content()`의 위치를 정확히 검사한다. 두 허용 위치 각각에서 30 tail order × 4 cap form을 typed output으로 검증한다. Position 1의 기존 cases도 회귀시킨다.
3. Layout은 실제 Grid track sizing trace의 반복 순서·span contribution·used rows를 검사한다. 기존 four-row frame normalization을 그대로 사용하되, fractional row의 1/64px 보정이 fit-content minimum floor/cap과 겹치는-span trace를 지켜야 한다. row index 추정이나 item y차로 track 크기를 복원하지 않는다.
4. RootHtml RuntimeSession 기본 경로에서 1,920 scene을 Chrome 관측 및 독립 관계와 비교한다. 모든 item/payload frame, row track order, span range, line count, source-order invariance를 검사한다. 기존 960 position-1 span scene, 810 repeated-text scene, 127 Grid track-sizing tests, track-list/cap/parser boundaries와 같은 session의 rejection→valid recovery를 회귀시킨다.
5. Negative matrix는 position 4–6, duplicate/missing keyword, 잘못된 list 길이/multiplicity, cap 변형, row span/column 변형, overflow/hidden placement, stale owner/revision, 누락·모순 trace를 포함한다. 거부 전후 layout/render revision, partial frame 비공개와 다음 valid input 회복을 확인한다.
6. 영향을 받은 Layout·Style projection·RuntimeSession Rust tests, 변경 crate Clippy `-D warnings`, `cargo fmt --all -- --check`를 실행한다. 기존 broad-suite 실패는 baseline과 비교하기 전까지 새 변경 성공으로 흡수하지 않는다.
7. 동일 source hash로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route에서 position 2/3 × `start|stretch`를 smoke 실행한다. layout-ready, semantic item/frame 및 draw/present를 기록하고 screenshot을 보존한다. OS screenshot은 visual smoke이며 PixelCopy/Metal readback이라 부르지 않는다. 시작 전 simulator/AVD 상태를 저장하고 이번 작업에서 띄운 대상 및 Agent Browser만 종료한다.
8. 실제 default Root HTML 경로를 사용한다. opt-in fixture 성공 route나 fixture FFI로 우회하지 않는다. default/fixture-test artifact 경계가 변하지 않는지 확인하고, 새 fixture marker·payload·FFI가 추가되면 기본 artifact 제외와 positive control을 각각 검사한다.
9. 새 구현/runtime 동작은 이전 계획 검토와 다른 실패 관점 20개로, 구현 PR 전체 diff는 별도의 30개 관점으로 검토한다. PR 본문에 가능한 Android/iOS 화면 링크를 포함한다. Rebase and merge 뒤에도 C11.4/C11 체크와 공식 `78/214`는 유지한다.

## 완료 판정

1,920 Chrome scene, 독립 Grid sizing/alignment 관계, two-position Style/Layout/RuntimeSession positive matrix, atomic rejection/recovery, Android/iOS default-route smoke, focused tests/Clippy/rustfmt와 별도의 20/30 실패 관점 검토를 확인했다. 구현 PR의 최종 검토·병합 전까지 이 bounded slice는 구현 후보로 둔다. 병합 뒤에도 전체 CSS track-size order, arbitrary text/image, general row-span, public CSS conformance 또는 C11.4/C11 roadmap 완료를 주장하지 않는다.
