# C11.4 · 반복 텍스트 row-span에 두 `fit-content()` 행 추가

- **상태:** 제한 구현 진행 중 · RuntimeSession 3,840/3,840 + min-content-floor/rejection controls 통과 · 기존 단일-fit row-span 1,920/4,320/960 scene 회귀 통과 · iOS Simulator visual smoke 통과 · Android Emulator 부팅 실패 · C11.4/C11 공식 체크와 `78/214` 유지
- **선행 구현:** [반복 text row-span `fit-content()` 위치 1–6](./c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6.md) · PR [#1306](https://github.com/ohah/spinon/pull/1306)
- **사전 비교:** [Chrome 157 3,840 scene](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-content-precomparison-2026-11-09/README.md)
- **계획 실패 관점 검토:** [서로 다른 20개](./c11-4-root-html-repeated-text-row-span-two-fit-content-plan-review-20.md)
- **계획 PR diff 검토:** [서로 다른 30개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-content-precomparison-2026-11-09/plan-pr-diff-review-30.md)

## 2026-11-09 구현 진행

- 구현은 두 fit-content member만 늘리고 list multiset, 네 tail order, 여섯 authored position, 두 row-span topology를 제한한다. `min-height:auto`는 이 두-fit explicit-span profile에서만 허용해 Chrome 157의 min-content floor가 cap보다 큰 대조를 보존한다.
- Chrome 관측 3,840 scene, min-content-floor control, three-fit atomic rejection/same-session recovery를 parsed Root HTML RuntimeSession에서 통과했다. 기존 single-fit row-span 위치 1–3의 1,920 scene, 4–6의 4,320 scene, 기존 960-scene row-span profile도 회귀 통과했다. 별도 RuntimeSession/layout/trace/clippy 세부 결과와 review는 [runtime evidence](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-content-runtime-2026-11-09/README.md)에 있다.
- iPhone 17 Pro/iOS 26.2 임시 Simulator에서 기본 Root HTML route의 layout-ready/draw-presented 화면 smoke를 확인하고 해당 임시 profile을 삭제했다. 기존 iPhone profiles는 보존했다. API 37.2 ps16k와 API 37.0 Google APIs temporary AVD는 앱을 설치하기 전에 QEMU 부팅 실패를 보여 삭제했다. 따라서 Android app runtime gate는 열려 있다.
- [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-content-runtime-2026-11-09/implementation-runtime-review-20.md)를 마쳤다. 구현 PR diff의 별도 30관점 검토, Android runtime 재시도, PR/merge 상태는 아직 남아 있다. 이 진행 기록은 이 bounded slice나 C11.4/C11을 완료 처리하지 않으며 공식 `78/214`를 변경하지 않는다.

## 결손과 목표

기본 parsed Root HTML `RootHtmlFitContentAutoRowsV1`의 repeated-text row-span profile은 여섯-member `grid-auto-rows` list에서 `fit-content()` 하나를 받는다. 앞선 제한 구현은 그 한 track의 authored 위치 1–6을 지원한다. 이 slice는 고정 list multiset에서 두 `fit-content()` member를 허용하고, 두 cap이 서로 다른 implicit row의 span contribution·growth limit·1/64px remainder 보정에 독립적으로 적용되는지 확인한다.

지원할 list multiset은 `fit-content(cap) × 2`, `min-content × 2`, `max-content × 1`, `auto × 1`로 고정한다. 두 cap은 현재 `fit_content_cap_is_supported` 경계대로 각각 finite non-negative direct px 또는 direct percentage다. 사전 비교는 zero, positive px/px, positive px/percentage, positive percentage/percentage와 over-container percentage를 대표하는 cap pair를 사용한다. 두 fit track은 여섯 authored position 중 임의의 서로 다른 두 위치에 놓는다. 나머지 네 track은 사전 비교에 고정한 네 tail order만 지원한다. 일반 track-list permutation 전체, 임의 text/span 또는 일반 CSS admission을 넓히지 않는다.

## 고정 입력과 독립 비교 모델

- 기본 parsed Root HTML route, definite `40×220px` Grid, `40px` explicit column, `10px` explicit row, LTR, visible overflow, `row-gap:0`, `column-gap:0`을 사용한다. 모든 item은 `grid-column:1 / 2`에 놓아 implicit column 생성을 막는다.
- Track-list matrix는 두 fit 위치 조합 15개, representative cap pair 4종과 두 cap assignment, 네 고정 tail order로 구성한다. 사전 비교의 고유 authored list는 `15 × 4 × 2 × 4 = 480`개다.
- 대표 cap pair는 `(0px, 0%)`, `(10px, 20px)`, `(10px, 20%)`, `(20%, 120%)`이며 두 fit member에 대한 여덟 assignment를 모두 비교한다. definite 220px content block-size에서 직접 percentage의 resolved limit은 `0px`, `44px`, `264px`이다. 이 pair는 사전 비교 표본이지 지원값 allowlist가 아니다. `calc()`, 음수, indefinite percentage basis는 허용하지 않는다.
- Topology는 ① row-span 2 다섯 item의 여섯-row profile, ② span-2 두 item 뒤 span-3 두 item의 여섯-row profile이다. 각 topology에서 DOM 정방향과 역방향 source order를 비교한다. Text payload/style은 기존 제한 grammar와 `10px monospace`, `10px` line-height를 그대로 사용한다.
- `align-content`는 `start|stretch`다. 전체 Chrome 157 Agent Browser 사전 비교는 `15 × 8 × 4 × 2 × 4 = 3,840` scene이다. HTML fixture, 전체 observation, screenshot, verifier, source hash는 evidence directory에 보존한다.
- 기준은 CSS Grid 2025 CRD [§7.2 track sizing functions](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing), [§7.6 implicit tracks](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-tracks)와 [§11.5 spanning-item intrinsic sizing](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)이다. 독립 관계는 authored track order 보존, span frame과 최종 점유 row 합의 일치, source-order reversal 불변성, `stretch`가 `auto` row에만 미치는 변화, 각 fit track의 cap 적용 위치다. `fit-content()`의 min-content floor 때문에 used size가 cap 이하라고 가정하지 않는다.

사전 비교는 3,840 scene 모두에서 explicit row 하나와 여섯 implicit row가 사용 행렬에 나타나고, 모든 row size가 finite non-negative이며, authored six-member track order와 span-frame/used-row sum이 일치함을 확인했다. Used implicit row의 최소 크기는 0px이다. Span 합 최대 오차는 `0.00025 CSS px`, 정방향·역방향 semantic frame 최대 차이는 `0 CSS px`, `stretch`로 인한 non-auto track 변화는 `0 CSS px`다. Cap variants에 따라 geometry가 바뀐 그룹은 480개 중 300개였다. 전체 used-track range는 최대 60px이며 fit track 관측 범위는 authored position 1–2에서 0–44px, 3–5에서 0–30px, 6에서 0–20px였다. 단일 auto track의 최대 stretch 변화는 130px다. 이는 Chrome 사전 비교 결과이며 Spinon runtime 구현 근거는 아니다.

## 지원 및 거부 경계

- 두 `fit-content()` track(각 cap은 finite non-negative direct px 또는 direct percentage), 여섯 위치 조합 15개, 네 tail order, 두 고정 row-span topology, `start|stretch`, 기존 direct-text grammar만 허용한다.
- 단일/세 개 fit track, 잘못된 multiplicity나 list 길이, 사전 비교 밖 tail order, 중복 authored position, `calc()`·음수·비유한 cap, indefinite percentage basis, 7개 이상 implicit row, 다른 span/column placement, auto/named/negative line, 다른 font·text style·Unicode, 추가 Grid owner, nested/inline/replaced child, unsupported alignment와 stale revision은 계속 거부한다.
- 실패 시 layout/render revision과 published frames는 원자적으로 유지하고 같은 RuntimeSession에서 valid input을 다시 처리한다.
- 일반 author CSS admission, 전체 CSS Grid track ordering, 임의 text shaping, WPT, surface pixel parity, 실기기/GPU parity, 전체 C11.4/C11 완료를 뜻하지 않는다. 신규 공개 API·FFI 또는 내부 계약 숫자 변경은 없다.

## 구현 및 검증 관문

1. 제품 코드 변경 전에 이 계획의 실패 관점 20개와 계획 PR 전체 diff의 별도 30개 관점 검토를 통과한다.
2. Style projection은 두 typed `fit-content()` 값·위치·track order를 보존하고 정확히 두 fit member만 허용한다. 기존 one-fit row-span과 auto-placement profile은 그대로 통과해야 한다.
3. Layout은 six-row track-sizing trace에서 각 fit track의 cap을 해당 row에 독립 적용한다. Span contribution, intrinsic minimum floor, growth limit, 1/64px remainder allocation이 다른 fit track의 cap을 공유하거나 덮지 않는지 확인한다. Trace·owner·revision의 누락/모순은 fail-closed다.
4. 기본 Root HTML RuntimeSession에서 3,840 scene을 Chrome observation 및 독립 row/span/order/alignment 관계와 비교한다. 각 scene의 parsed track order, used rows, text line count, item frames, cap assignment를 검사한다.
5. Negative matrix는 한/세 fit member, 반복 cap 목록의 잘못된 multiplicity, `calc()`·음수·비유한 cap, list/tail 변형, percentage basis 누락, 잘못된 span/placement, trace 누락·순서 불일치, stale owner/revision을 포함한다. Positive cap controls에는 사전 비교 pair 밖의 유효한 px/percentage 조합과 min-content floor가 cap보다 큰 case를 넣어 floor 동작을 확인한다. 실패 원자성 및 same-session valid recovery를 확인한다.
6. 기존 row-span 한 fit track 위치 1–6 4,320 scene, 앞선 위치 1–3 1,920 scene, auto-placement 위치 1–6 1,440 scene, repeated-text 810 scene, track-list/cap/parser regressions와 Grid track-sizing suite를 회귀시킨다. 영향 crate의 focused Rust tests, scoped Clippy `-D warnings`, `cargo fmt --all -- --check`를 실행한다. baseline 실패는 비교하기 전까지 분리한다.
7. 동일 source hash를 Android API 37.2 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route에서 실행한다. Early/adjacent/separated fit-position pair, cap assignment, 두 alignment와 두 topology를 대표하는 sentinel에서 layout-ready와 draw/present를 기록하고 화면을 보존한다. Screenshot은 visual smoke이며 PixelCopy/Metal drawable readback이 아니다.
8. Opt-in fixture FFI를 제품 실행 경로로 사용하지 않는다. default/release artifact 경계가 바뀌거나 marker/payload가 추가되면 flag-off artifact 부재와 fixture positive control을 확인한다.
9. Simulator와 Agent Browser의 시작 전 상태를 기록한다. 이 검증에서 띄운 앱 프로세스, 시뮬레이터와 Agent Browser 세션은 종료하고 앱 설치 artifact만 제거한다. 재사용 가능한 simulator profile은 보존한다.
10. 구현/runtime은 이전 계획 검토와 다른 20개 실패 관점, 실제 구현 PR 전체 diff는 별도의 30개 관점으로 검토한다. PR 본문에는 사용 가능한 Android/iOS 화면 링크를 제공한다. Rebase and merge 이후에도 C11.4/C11 체크, 공식 `78/214`, 내부 계약 `0.1.0`은 유지한다.

## 완료 판정

3,840-scene RuntimeSession 비교, 독립 sizing/alignment 관계, fit-cap별 floor·growth·rounding 동작, atomic rejection/recovery, 기존 profile regressions, 추가 유효 cap controls, focused Rust/Clippy/rustfmt, Android/iOS default-route sentinel, 별도 implementation/runtime 20관점 및 PR diff 30관점 검토, roadmap/spec sync와 merge state를 확인한 뒤에만 이 bounded slice를 구현 완료로 기록한다. 이 후보는 일반 CSS Grid, WPT 적합성 또는 C11.4/C11 전체 완료로 승격하지 않는다.
