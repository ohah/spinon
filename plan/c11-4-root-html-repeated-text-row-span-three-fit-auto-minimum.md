# C11.4 · repeated-text row-span three-fit `min-height:auto`

**상태:** 계획 완료 · 계획 PR [#1334](https://github.com/ohah/spinon/pull/1334) 병합 · 구현·RuntimeSession 회귀·시뮬레이터 visual smoke·구현/runtime 20관점 검토 완료 · implementation PR diff 30관점과 PR 병합 절차 진행 중 · C11.4/C11 미완료 · 공식 `78/214` 유지

## 구현 진행

- Three-fit row-span admission, source-order cap validation, `min-height:auto`, track remainder rounding, authored `calc()` cap rejection 및 RuntimeSession matrix/rejection tests가 구현됐다.
- Chrome 157 기준 11,520개 RuntimeSession scene을 다시 비교해 통과했다(테스트 686.80초; 전체 Cargo 실행 11분 24.96초). 이 실행은 winning specified `calc()` 값을 보존해 reject하는 변경 뒤의 결과다.
- 기존 회귀 중 one-fit later positions 4–6 4,320 scene, one-fit row-span auto minimum 5,760 scene, two-fit row-span 3,840 scene, two-fit auto-placement 및 auto-minimum 각 2,880 scene, mutation rejection/recovery가 통과했다. 추가로 two-fit `min-content` floor control과 four-fit mutation atomic rejection/same-session recovery도 통과했다. 기존 two-fit rejection은 이번 slice에서 새로 허용한 three-fit row-span list 자체를 거부하던 stale 입력이어서 four-fit mutation으로 바꿨다.
- 기존 회귀 one-fit auto-placement 1,440 scene과 repeated-text 810 scene도 통과했다. Layout/style/style-to-layout Clippy는 `-D warnings`로 통과했고, runtime 전체 Clippy는 변경하지 않은 기존 test helper 두 곳의 `too_many_arguments`에서만 실패했다. 그 lint만 명령에서 제외한 runtime-only Clippy는 통과했다. 네 package를 한 번에 선택하고 runtime fixture-test feature를 켠 combined Clippy는 변경하지 않은 `spinon-style` test helper의 `fixture_with_child_style`/`Fixture.child` 참조 불일치로 compile 단계에서 실패했다. 해당 파일은 이번 diff 밖이며 개별 package Clippy는 통과했다.
- 기존 row-span 960 scene 회귀도 통과했다. 그 테스트는 `rows` 관측치와 일치하지 않는 `rowValues` 접근 및 사용하지 않는 D/E color rule까지 sentinel 원문과 비교해 시작 전에 실패해, 이 두 테스트 harness 가정을 고쳤다.
- one-fit positions 4–6의 4,320 paired scene도 통과했다(403.82초). Android API 37.0 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 Root HTML fixture로 light/dark visual smoke를 통과했다. 앱 로그에서 fixture SHA, `layout=ready`, draw/present, OS appearance/night mode 변경에 따른 `dark=true` 및 environment revision 증가를 확인했다. 원본 asset과 기존 Android APK를 hash로 복원했고 두 시뮬레이터를 종료했다. [플랫폼 근거·캡처](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-runtime-2026-11-10/README.md)
- 계획 검토와 다른 구현/runtime 20관점 기록은 [검토표](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-runtime-2026-11-10/implementation-runtime-review-20.md)에 작성했다. Implementation PR diff의 별도 30관점 검토, PR/필수 CI/merge 및 merge 뒤 contract·status·roadmap·Tailnet 동기화는 남아 있다.
- `cargo fmt --all -- --check`와 `git diff --check`는 현재 변경에서 통과했다. 전체 공식 상태 대장·공개 roadmap 완료 수는 이 제한 범위로 변경하지 않는다.

**선행 범위:** [single-fit row-span positions 1–6](./c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6.md) · [two-fit row-span](./c11-4-root-html-repeated-text-row-span-two-fit-content.md) · [one-fit row-span `min-height:auto`](./c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum.md) · [two-fit row-span `min-height:auto`](./c11-4-root-html-repeated-text-row-span-two-fit-content.md)

**사전 비교:** [Chrome 157 11,520-scene 비교와 독립 verifier](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-precomparison-2026-11-10/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-three-fit-auto-minimum-precomparison.html) · [전체 관측](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-precomparison-2026-11-10/chrome157-observation.ndjson)

**계획 실패 관점 검토:** [서로 다른 20개](./c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-plan-review-20.md)

**계획 PR diff 검토:** [실제 변경분의 서로 다른 30개 관점](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-precomparison-2026-11-10/plan-pr-diff-review-30.md)

## 결손과 제한 목표

기본 parsed Root HTML의 제한 repeated-text row-span profile은 six-member `grid-auto-rows` list에서 one-fit과 two-fit track을 각각 지원하고, 이 입력에서는 text item의 `min-height:0|auto` profile을 구분한다. Three-fit multiset은 명시적으로 거부된다. 이 계획은 세 fit track의 independent cap, spanning contributions, automatic minimum과 1/64px row rounding이 기존 profile의 범위 안에서 맞는지 확인하고, 고정 six-row topology만 허용하도록 한다.

허용할 새 multiset은 정확히 `fit-content()` 3개, `min-content` 1개, `max-content` 1개, `auto` 1개다. 세 cap은 각각 finite non-negative direct `px` 또는 direct percentage여야 하며, 이 비교의 percentage basis는 definite 220px block size다. Three-fit auto-placement, four-fit, arbitrary track list, arbitrary span topology, 일반 author CSS admission과 C11.4/C11 전체 완료는 범위 밖이다.

## 구현 전 비교 모델

- 규범 출처는 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. §6.6에 따라 item이 non-scrollable이고 span이 auto minimum track을 포함하며 flexible track을 포함하지 않을 때만 content-based automatic minimum 대상이 된다. §7.2의 `fit-content()`와 `auto` minimum sizing function이 eligibility track을 결정한다. §11.5의 spanning contribution 순서는 fit cap별 used size와 item geometry의 비교 기준이다.
- Agent Browser HeadlessChrome `157.0.0.0`으로 [11,520개 scene](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-precomparison-2026-11-10/verification.json)을 관측했다. 5,760 paired keys에서 automatic minimum 후보 eligible item occurrence 24,576개, content-floor violation 0, source-order 최대 차이 0 CSS px, non-auto stretch 차이 0 CSS px를 확인했다. 이는 Chrome oracle 결과이지 제품 RuntimeSession 구현 결과가 아니다.
- 고정 입력은 definite `40×220px` Grid, `40px` explicit column, `10px` explicit row, gap 0, LTR, visible overflow, sparse row flow다. Text payload는 기존 ASCII direct-text grammar와 10px monospace/10px line-height다.
- six-member auto-row list는 three-fit plus one each of min-content, max-content and auto다. 모든 20 fit-position triples, 나머지 세 track의 six permutations, 세 direct cap의 six assignments(`0px`, `10px`, `20%`)를 포함한다.
- Topology는 span-2 five-item six-row family와 span-2/span-3 four-item six-row family다. 각 family는 DOM forward/reverse를 가진다. `align-content:start|stretch`와 `min-height:0|auto`를 교차해 총 `20×6×6×2×4×2 = 11,520` scene을 비교했다. 첫 explicit-row empty control은 두 mode에서 계속 `min-height:0`이다.
- 독립 관계는 authored track order 보존, 일곱 used row의 finite non-negative 값, item frame과 점유 used-row 합 일치, source-order reversal 불변성, stretch에서 auto row에만 추가 공간 배분, candidate의 eligible span item이 payload content floor 아래로 내려가지 않는지다. `fit-content()` used size가 cap을 넘지 않는다고 가정하지 않는다.

## 사전 비교 결과와 구현 시 주의

- 22,320개 text item frame이 `min-height:0`에서 `auto`로 바뀌었다. 최대 item-frame 변화 70 CSS px, 최대 track 변화 51.656 CSS px다. Candidate content-floor violation은 0이다.
- Span-frame 합 최대 차이는 0.00025 CSS px다. 5,760 source-order pair 최대 차이는 0 CSS px다. Stretch는 non-auto track을 바꾸지 않았고 auto row에는 최대 130 CSS px를 더했다.
- Cap permutation 1,920 groups 중 896개가 used geometry를 바꿨다. 이 비율은 고정 fixture의 결과이며 모든 콘텐츠에 일반화하지 않는다.
- 신규 support predicate는 정확히 세 fit track의 six-member row-span list를 구분해야 한다. 기존 one-fit/two-fit row-span과 one-/two-fit auto-placement의 admission을 대체하거나 넓히면 안 된다. 세 cap을 authored track index 및 track-sizing trace와 같은 owner/revision에 결합하고, 누락·중복·오래된 trace 또는 span count 불일치는 fail-closed한다.
- Automatic minimum은 모든 span text item에 같은 mode를 쓴 고정 profile에서만 연다. Item별 혼합, scrollable overflow, flexible track, nested/inline/replaced child, unsupported text/style, implicit column, seven-plus positive implicit row는 거부한다. 실패는 layout/render output을 부분 게시하지 않고 같은 RuntimeSession에서 valid revision 복구를 허용해야 한다.
- 세 fit track에서 1/64px remainder tie 분배가 one-/two-fit 규칙과 같다고 미리 단정하지 않는다. Chrome used row값과 실제 RuntimeSession trace를 대조하고, 일치하지 않으면 새 위치/track별 검증된 규칙만 제한 추가한다.

## 지원 및 거부 경계

- **허용:** six-member list에 정확히 three-fit + min-content + max-content + auto, finite non-negative direct px/percentage caps, 모든 fit-position triple과 나머지 tail order 여섯 개, 두 고정 six-row span topology, forward/reverse DOM, start/stretch, 전 text item의 `min-height:0` 또는 전 text item의 `min-height:auto`.
- **거부:** one/two/four fit list mutation은 이 새 predicate의 positive profile에 포함하지 않는다. 잘못된 list length/multiplicity/order, negative·non-finite·`calc()` cap, indefinite percentage basis, auto-placement에 three-fit list 사용, 7개 이상 implicit row, 다른 row/column placement, dense/column flow, mixed min-height item, scrollable overflow, flexible/extra Grid track, unsupported alignment/style/payload, stale/missing/duplicate trace·owner·revision은 거부한다.
- 이 제한 profile은 공개 API/FFI 또는 numeric contract version `0.1.0`을 추가하지 않는다. 일반 CSS 지원, 전체 WPT, 실기기/GPU pixel parity나 C11.4/C11 완료를 의미하지 않는다.

## 계획과 구현 검증 관문

1. 제품 코드를 바꾸기 전에 이 계획을 서로 다른 실패 관점 20개로 검토하고, 계획 PR의 실제 변경분도 별도로 30개 관점으로 검토한다.
2. Style projection은 typed three-fit values와 authored order/caps를 보존하며 정확한 three-fit row-span profile만 허용한다. 기존 single/two-fit row-span과 one-/two-fit auto-placement positive/negative tests를 그대로 유지한다.
3. Layout은 exact six positive implicit rows, supported owner/revision, item order/placement, same-session trace를 확인한다. Three fit cap은 각 해당 implicit row에 적용하고, intrinsic minimum floor·growth limit·1/64px remainder 기록을 대조한다.
4. 기본 parsed Root HTML RuntimeSession에서 paired `min-height:0|auto` 11,520 scene 전체를 Chrome 관측 및 독립 frame/track/order/alignment/eligibility 관계와 비교한다. Eligible payload floor, authored member order, six caps/position assignment 및 line count를 검사한다.
5. Negative matrix는 1/2/4 fit, 잘못된 multiplicity/tail/list length, invalid cap/basis, auto-placement, wrong span/column/flow, mixed minimum, unsupported child/style, extra rows, owner/revision/stale/missing/duplicate/out-of-order trace를 포함한다. Rejection atomicity와 같은 session recovery를 확인한다.
6. 기존 row-span one-fit positions 1–6 4,320 scenes, one-fit row-span automatic minimum 5,760 scenes, two-fit row-span 3,840 scenes, two-fit row-span automatic minimum regression, auto-placement one-/two-fit, repeated-text 810 scenes와 Grid track-sizing regression을 실행한다. 변경 crate의 focused Rust tests, `Clippy -D warnings`, `cargo fmt --all -- --check` 결과를 기록한다. Baseline 확인 전에는 broad-suite 실패를 제품 회귀로 단정하지 않는다.
7. 동일 source SHA의 기본 Root HTML route를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 light/dark로 실행한다. baseline/candidate start/stretch sentinel의 HTML/app hashes, `layout=ready`, draw/present, OS screenshot을 보존한다. 화면은 visual smoke이며 PixelCopy/Metal readback, compositor 이후 pixels 또는 실기기 증거가 아니다.
8. 공용 Android/iOS profile을 재사용한 뒤 shutdown한다. 새 task-owned profile은 실행 종료 뒤 제거한다. 앱 설치·HTML asset은 기존값을 백업 hash로 확인하고 복원하며, 재사용 profile과 사용자 data는 지우지 않는다. Agent Browser는 고유 session을 쓰고 캡처 직후 닫은 뒤 세션과 해당 작업 process가 사라졌는지 확인한다. Terminal Browser는 사용하지 않는다.
9. 계획 검토와 다른 20개 관점으로 구현/runtime을 검토하고, implementation PR diff는 별도 30개 관점으로 검토한다. PR 본문에 가능한 Android/iOS 동작 화면의 commit-pinned raw 링크를 넣고 Agent Browser로 load 확인한다.
10. Rebase and merge 뒤 internal contract 0059, `spec/STATUS.md`, public roadmap, 계획/evidence 및 지정 Tailnet preview를 실제 결과와 동기화한다. 이 bounded slice만으로 C11.4/C11 checkbox, 공식 `78/214` 또는 numeric contract `0.1.0`을 바꾸지 않는다.

## 완료 판정

Paired RuntimeSession matrix, CSS automatic-minimum eligibility와 track-stage trace, content floor, atomic rejection/recovery, 기존 profile 회귀, focused Rust/Clippy/rustfmt, Android/iOS default-route visual smoke, separate implementation/runtime 20-review와 implementation PR diff 30-review, merge 뒤 상태/spec/roadmap/Tailnet sync를 확인한 뒤에만 이 bounded slice를 구현 완료로 기록한다. 이는 three-fit row-span profile에 대한 제한 검증이며 general CSS Grid, WPT 또는 C11.4/C11 전체 완료가 아니다.
