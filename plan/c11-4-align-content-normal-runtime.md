# C11.4 · `align-content: normal` 3행 auto-track stretch 앱 런타임 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 사전 비교를 고정했다. [계획 자체 20관점 검토](./c11-4-align-content-normal-runtime-plan-review-20.md)를 통과했으며 계획 PR diff 30관점 검토 뒤에만 병합한다. 계획이 병합되기 전에는 구현을 시작하지 않는다. 이 제한 fixture 후속은 새 CSS 지원 완료나 C11.4 승격을 뜻하지 않는다. C11.4 checkbox와 공식 완료 집계 `72/213`은 유지한다.

## 목적

병합된 Rust oracle은 CSS Grid §11.8의 `align-content: normal`에서 두 auto 행이 남은 definite free space를 균등하게 받고 fixed 세 번째 행이 유지되는 입력을 확인한다. 병합된 앱 runtime fixture는 `normal`과 auto-track stretch 경로를 확인하지만 대상 auto row가 하나인 다른 geometry다. 이 계획은 기존 C11.4 opt-in CSS→HostDocument/Stylo→typed projection→Taffy→FFI→Android/iOS host 경로에 oracle과 같은 row-axis 입력을 가진 3행 scene을 한 개 추가해 **균등한 성장·fixed row 유지가 앱 출력에서도 같은가**를 확인한다.

이는 새 CSS admission, Taffy 알고리즘 변경, public API 또는 일반 Grid 지원을 추가하지 않는다. 변경은 fixture-test profile, 고정 scene allowlist와 검증 harness에 한정한다.

## 구현 전 비교 모델

[C11.4 §11.8 사전 비교](../spec/internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27/README.md)는 CSS Grid Layout Module Level 1, 2025 CRD §11.8과 독립 산술을 기준으로 한다. Agent Browser `0.31.1` / Chrome `156.0.8078.48` (revision `d0c02bfad08ec92a6ffcf989e53cd995e6381a4c`) 관찰은 [fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-align-content-normal-runtime.html), [입력](../spec/internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27/input.json), [기대값](../spec/internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27/expected.json), [관찰 JSON](../spec/internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27/chrome156-observation.json), [Chrome 화면](../spec/internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27/chrome156-normal-auto-track-rows.png)에 고정했다. WPT harness나 Spinon 구현은 이 비교에서 실행하지 않았다.

고정 입력은 다음과 같다.

- Root border-box와 viewport 모두 `160×100px`, definite root block-size, border/padding 0이다. 기존 pure Rust oracle은 `RootSizingPolicy::Match`, 앱 Grid runtime profile은 viewport containing-block 정책(`ResolveWithin`)을 사용한다. root width/height를 모두 고정해 두 경로에서 같은 root frame과 definite row-axis size를 기대한다. 이 fixture는 root-sizing 정책 자체의 parity를 주장하지 않는다.
- Explicit columns [`160px`], rows [`auto, auto, 5px`], `row-gap:10px`, `column-gap:0`, `align-content:normal`, `justify-content:flex-start`, row auto-flow. Pure Rust oracle와 행 축 track·gap·height·alignment 값은 같고 root/column/child 폭은 oracle `20px`, 앱 fixture `160px`이다. 화면 캡처에서 입력 색상 영역을 식별하기 쉽도록 영향 없는 inline 축만 넓혔으며 root, column, child 폭을 같게 고정해 row-axis 결과에 free space가 생기지 않는다. 이 계획은 전체 x축 frame parity를 주장하지 않는다.
- DOM 순서의 direct block items 높이 [`10,20,5`]`px`; 각 item은 한 행에 하나씩 놓인다.
- §11.8 전 row bases [`10,20,5`]`px`; 두 내부 gutter는 각각 `10px`이다. 양의 definite free space는 `100 - (10 + 20 + 5) - (10 + 10) = 45px`이므로 auto max track 두 개는 각각 `22.5px` 증가하고 fixed `5px` track은 유지된다.
- 기대 rows [`32.5,42.5,5`]`px`, 남은 row space `0px`, root frame `(0,0,160,100)`, item frames `(0,0,160,10)`, `(0,42.5,160,20)`, `(0,95,160,5)`.
- 이 값은 Taffy 출력에서 역산하지 않는다. CSS track trace에서 gutter entries를 CSS row 집계와 분리한다. 반올림 전 CSS-px floats의 허용 오차는 절대 `1e-4 CSS px`다.

## Fixture와 구현 범위

- 새 고정 scene ID `align-content-normal-equal-auto-rows`를 기존 `c11_4_grid_track_sizing_runtime_source` opt-in fixture dispatch에 추가한다. 새 CSS grammar·FFI entry point·ABI는 만들지 않는다.
- Fixture source는 이미 승인된 C11.4 TrackSizing profile에서만 허용한다. root의 정확한 size/track/gap/alignment와 child 수·순서·높이를 검사하고 unknown scene ID 및 JavaScript suffix를 붙인 scene ID를 거부한다. 새 CSS parser capability를 일반 cascade에 열지 않는다.
- 새 scene의 generic runtime report는 computed-style snapshot을 별도로 내보내지 않는다. FFI source allowlist test는 고정 source의 explicit `align-content:normal` 선언을 확인하고, 새 runtime matrix는 두 auto row의 stage/track/frame geometry만 주장한다. computed normal→typed projection 자체는 이미 병합된 #664 fixture의 검증 경계이며 이 계획에서 다시 주장하지 않는다.
- Rust/FFI 검증은 source allowlist와 exact tree identity, 같은 root의 Rows pass 한 개, `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks`의 단일·정순 stage, 처음 세 stage row bases [`10,20,5`], 마지막 `StretchAutoTracks { applied: true }`와 [`32.5,42.5,5`], 내부 gutter [`10,10`], fixed row 불변, root와 child frames 및 revision 일치를 확인한다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 source commit과 fixture input hash로 이 한 scene을 실행한다. 각 host는 exact rows/stages/frames, expected root/node identity, layout revision, nonempty draw와 presentation completion을 확인한다. platform pixel sample은 저장해 화면 경계를 보조하되, 명시적 RGBA 비교 없이는 pixel equality를 주장하지 않는다. 결과는 simulator/AVD에 한정하며 실기기·Android hardware GPU·iOS compositor-after 픽셀로 일반화하지 않는다.
- 기본/product artifact에서 C11.4 fixture ID/source/symbol/payload가 빠지고 fixture-test positive control에 들어가는 경계를 다시 확인한다. static archive 검사는 최종 APK/IPA exclusion을 증명한다고 해석하지 않는다.
- public contract 0059, `spec/STATUS.md`, CSS 지원 문서는 구현이 새 공개 동작이나 계약 변경을 만들지 않는 한 완료 상태를 올리지 않는다. 공식 C11.4 checkbox와 `72/213`은 바꾸지 않는다.

## 작업 순서와 검토

1. 이 계획 문서, 고정 CSS fixture, Chrome 사전 비교의 서로 다른 실패 관점 20개 계획 검토를 완료한다.
2. 계획 변경 전체는 별도 30관점 PR diff 검토를 통과한 뒤 Rebase and merge한다. 계획 PR을 병합하기 전에 구현하지 않는다.
3. 새 구현 branch에서 기존 fixture-test feature와 C11.4 app host route를 확장하고 focused Rust/FFI tests, formatter/Clippy, fixture-artifact exclusion/positive control을 실행한다.
4. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 최신 source의 exact one-scene matrix와 raw logs/screenshots를 수집한다. Terminal Browser는 사용하지 않는다. Agent Browser를 별도로 열면 작업 후 그 session을 닫고 `session list`에서 활성 session이 없음을 확인한다.
5. 구현과 실제 runtime 결과는 계획 검토와 다른 새 실패 관점 20개로 검토한다. 구현 PR diff는 작업 및 구현 검토와 독립된 30관점으로 검토한다. 결함은 수정 후 영향 관점을 다시 확인한다.
6. PR은 Rebase and merge한다. PR description에는 검증된 simulator 화면을 full commit SHA를 사용하는 immutable raw image URL로 첨부한다. GitHub check rollup이 비면 CI 통과로 쓰지 않는다.

## 검증 경계

이 slice는 단일 row-axis auto-stretch 입력 하나를 기존 test-only app route에 추가한다. 행·열 전반의 alignment, flex tracks, implicit tracks, auto-repeat, text/image contribution, span·gutter 조합, 일반 CSS admission, Spinon WPT harness 및 전체 CSS Grid 적합성은 포함하지 않는다. 독립 Rust oracle의 결과는 [PR #656 실행 근거](../spec/internal/evidence/c11-4-align-content-normal-auto-track-stage-oracle-2026-10-26/README.md)에 있으며, 본 구현은 그 oracle과 Android/iOS fixture 경로의 geometry 연결만 새로 주장한다.
