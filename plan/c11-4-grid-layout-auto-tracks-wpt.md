# C11.4 · WPT `grid-layout-auto-tracks.html` 제한 runtime slice

**상태:** 고정 WPT 장면의 제한 runtime 구현 후보를 추가하고 Android/iOS Simulator, archive 경계, 구현/runtime review를 확인했다. PR 병합 전 후보이며 CSS Grid 일반 지원이나 C11.4 완료를 뜻하지 않는다. [실행 근거](../spec/internal/evidence/c11-4-grid-layout-auto-tracks-runtime-2026-10-27/README.md).

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**원본 WPT·Chrome·규범 기대값:** [사전 비교 README](../spec/internal/evidence/c11-4-grid-layout-auto-tracks-precomparison-2026-10-27/README.md) · [expected.json](../spec/internal/evidence/c11-4-grid-layout-auto-tracks-precomparison-2026-10-27/expected.json) · [source fixture](../tests/fixtures/css/c11/grid-layout-auto-tracks-c114-precomparison.html)

## 목적과 범위

고정 WPT `css/css-grid/grid-definition/grid-layout-auto-tracks.html`의 한 시각 비교 장면을 `RuntimeBlockFlexGridTrackSizingV1` opt-in 앱 fixture로 실행한다. 한 Grid에서 명시 `auto` 행, 양의 암시 `auto` 열과 행, 같은 행에 걸친 두 item의 intrinsic contribution, root 폭 밖의 암시 열을 함께 검사한다. 기존 자동 배치, 단일 양의 암시 행/열 및 intrinsic text scene이 각각 다루는 조건의 교차다.

WPT 원본의 `<meta rel=match>`는 testharness 검사가 아니다. 원본과 reference를 Chrome에서 캡처한 결과는 baseline만 제공한다. App/runtime 입력은 브라우저 기본 font의 19px line box를 쓰지 않고 저장소 Ahem의 `10px/10px`를 지정한다. 이는 플랫폼별 기본 font metric 차이를 제거하기 위한 계획상의 normalization이며 upstream WPT 입력과 완전히 같은 CSS라고 주장하지 않는다.

대상 입력은 고정 fixture 하나다.

- Grid root: `width:100px`, `height:auto`, `grid-template-columns:100px`, `grid-template-rows:auto`, `grid-auto-columns:auto`, `grid-auto-rows:auto`, row auto-flow, gap 0.
- Ahem 10px/10px `&nbsp;` item `a`: row 1 / column 1.
- 50px fixed-width child와 Ahem `&nbsp;`를 가진 item `b`: row 1 / positive implicit column 2.
- direct Ahem `&nbsp;` item `c`: positive implicit row 2 / column 1.
- `align-items:start`, LTR, horizontal writing mode, margin/padding/border 0.

## 독립 oracle

기준 알고리즘은 [CSS Grid Layout Module Level 1, 2025 CRD §§6.6, 11.5–11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)이다.

Chrome 156 Agent Browser 관찰은 [expected.json](../spec/internal/evidence/c11-4-grid-layout-auto-tracks-precomparison-2026-10-27/expected.json)에 원본과 normalized fixture를 분리해 보관한다. WPT 원본에서 observed columns는 `[100,50]px`, rows `[19,19]px`, root `100×38px`다. 그 19px는 Chrome 기본 font 관찰값이다. 같은 WPT source와 reference의 캡처 hash가 일치했다.

Ahem fixture의 normative oracle은 CSS Grid Layout Module Level 1 2025 CRD §11.5–§11.6의 intrinsic track sizing과 §6.6 automatic minimum에서 독립적으로 계산했다.

- 두 auto row의 단일-span minimum contribution은 각 10px이다. 첫 row의 `a`와 `b` contribution은 같은 track에 합산하지 않고 큰 값 10px을 적용한다.
- `c`가 만드는 positive implicit row도 10px이다. root의 auto block size는 row 합계 20px이다.
- `b`의 50px child는 positive implicit auto column의 minimum contribution이다. 명시 100px column 뒤에 50px implicit column이 오며 root의 fixed width는 100px로 유지돼 item이 오른쪽으로 50px overflow한다.
- 예상 CSS tracks: columns `[100,50]px`, rows `[10,10]px`.
- 예상 border-box frames: root `(0,0,100,20)`; `a (0,0,100,10)`; `b (100,0,50,10)`; `c (0,10,100,10)`; `b` content `(100,0,50,10)`.
- surface 확인은 root 밖 `b`의 content 내부 `(125,5)`와 `(149,5)`, 그리고 item 끝 `(150,5)` 대조점을 사용한다.

Chrome 관찰값은 위 독립 기대값을 대체하지 않는다. Stage trace에서 columns/rows와 contributions를 따로 읽고 root/item frames를 각각 비교한다.

## 구현·검증 게이트

1. 계획의 20개 실패 관점 검토를 구현 전에 완료한다.
2. `spinon-layout`에는 explicit auto row, positive implicit column/row, row-1 competing contribution을 한 입력에서 검사하는 순수 Rust regression을 추가한다. 각 axis track trace, contribution source 및 root/item frame을 고정 oracle과 대조한다.
3. runtime fixture adapter는 허용한 root/item/child tag, 정해진 DOM child 순서·텍스트·CSS track values만 수용한다. 고정 scene ID 외 입력은 계산 전 거부한다. 기존 `c04-runtime-gpu-fixture-test` opt-in 경계 밖으로 adapter, FFI symbol 또는 payload를 노출하지 않는다.
4. fixture-test FFI route와 Android/iOS scene allowlist를 갱신하고 Rust runtime matcher에서 정확한 revision, track trace, contribution, 전체 node frame, draw/present를 확인한다. 누락·추가 node, 다른 font/line-height, child width, track pattern, placement, writing mode, gap, root identity는 거부한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최종 PR source를 각각 실행한다. 이 scene은 `b`가 Grid root 폭 밖으로 나가므로 PixelCopy와 Metal drawable에서 지정된 바깥 좌표 및 item 끝 대조 표본을 비교하고 screenshot·raw result·build hash를 보존한다. iOS drawable 표본은 compositor 제출 전 결과로만 보고한다.
6. 기본/제품 `spinon-ffi` artifact와 fixture-test positive control에서 새 fixture marker·FFI symbol·scene payload 포함 여부를 대조한다. 이 archive 결과를 APK/IPA 전체 배제 근거로 확대하지 않는다.
7. 새 실패 관점의 구현/runtime 20개와 별도 PR diff 30개를 통과한 뒤 `Rebase and merge`한다. PR의 merge/check 상태와 screenshot URL은 게시 직전 실제 로드까지 확인한다. Agent Browser를 사용한 뒤 browser process를 종료한다.

## 완료하지 않는 항목

이 계획은 WPT 전체 파일/Spinon WPT runner, 다른 auto-track 조합, 일반 author CSS admission, text font/script 전반, named line/area, 다른 placement mode, hardware GPU, Android 실기기, iOS compositor 이후 표면, 전체 C11.4/C11 지원을 닫지 않는다. 공식 C11.4 checkbox와 현재 `72/213` 집계는 구현 slice만으로 바꾸지 않는다.
