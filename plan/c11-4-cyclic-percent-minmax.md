# C11.4 · cyclic percentage `minmax()` track sizing 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획의 세 고정 장면을 opt-in C11.4 runtime fixture에 연결했고 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 3/3 실행했다. 이는 제한 fixture 검증이지 일반 CSS 지원 선언이나 C11.4 완료가 아니다. C11.4 checkbox와 공식 완료 수 `72/213`은 유지한다. [계획 검토 20개](./c11-4-cyclic-percent-minmax-plan-review-20.md) · [사전 비교](../spec/internal/evidence/c11-4-cyclic-percent-minmax-precomparison-2026-10-22/README.md) · [구현·simulator 결과와 검토](../spec/internal/evidence/c11-4-cyclic-percent-minmax-runtime-2026-10-22/README.md)

## 목적과 범위

직전 [cyclic percentage runtime slice](../spec/internal/evidence/c11-4-cyclic-percentage-track-runtime-2026-10-22/README.md)는 `grid-template-columns:50%`와 `grid-template-rows:50%`를 고정 scene으로 확인했다. 이번 계획은 같은 입력에서 percentage maximum을 `minmax(0px, 50%)`로 감싼 세 장면을 opt-in C11.4 fixture 경로에 연결한다. 기존 generic layout 경로가 규범 결과와 다르면 원인을 좁혀 최소 경계만 고치고, 이미 맞으면 fixed fixture와 검증 근거만 추가한다.

대상은 HTML namespace `div` 두 개(root와 item), 단일 explicit track, 한 축 cyclic percentage, `max-content` sizing과 definite column control이다. CSS author support를 확장하지 않고 내부 fixture-test feature만 사용한다.

## 입력과 독립 기대값

Chrome 비교 입력은 [fixture](../tests/fixtures/css/c11/grid-cyclic-percent-minmax-c114-precomparison.html)다. 모든 root와 item은 margin, padding, border, gap 0이고 item은 `24×16px`로 고정한다. color는 root `#101820`, item `#e5484d`다.

| Scene | Root sizing | Explicit tracks | 독립 used tracks | Root frame | Item frame |
| --- | --- | --- | --- | --- | --- |
| `cyclic-minmax-column` | width `max-content` | Columns `minmax(0px, 50%)`; Rows `auto` | Columns `[12px]`; Rows `[16px]` | `24×16px` | `24×16px` |
| `cyclic-minmax-row` | width `24px`; height `max-content` | Columns `auto`; Rows `minmax(0px, 50%)` | Columns `[24px]`; Rows `[8px]` | `24×16px` | `24×16px` |
| `definite-minmax-column` | width `40px` | Columns `minmax(0px, 50%)`; Rows `auto` | Columns `[20px]`; Rows `[16px]` | `40×16px` | `24×16px` |

### 규범 계산

[CSS Grid Layout Module Level 1, 2025 CRD §11 Grid Layout Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)은 먼저 Grid container size를 구하고, 이 단계의 cyclic percentage in track sizes는 `auto`처럼 취급한다고 적는다. 이어 resulting container size로 Grid Sizing Algorithm을 실행할 때 track percentages를 해당 container size에 대해 푼다. 이 계획은 그 두 단계를 분리한다.

- 열 scene의 intrinsic width는 item의 `24px` contribution이다. 최종 width `24px`에서 percentage maximum은 `12px`로 해석된다. item은 `24px`이므로 12px track을 넘어가지만 root frame 밖으로는 나가지 않는다.
- 행 scene도 item의 `16px` block contribution에서 root height `16px`을 얻은 뒤, `50%` row를 `8px`로 해석한다. 16px item은 row track 아래로 넘친다.
- definite control은 content width `40px`의 절반인 `20px` track을 사용한다. cyclic intrinsic sizing 단계가 없어 percentage를 바로 definite basis로 푼다.

위 수치는 브라우저 결과를 역으로 복사하지 않고 규범 단계와 고정 item contribution으로 계산했다. Chrome 156은 별도 관찰 자료이며 Chrome 155 고정 baseline을 대체하지 않는다.

## 사전 oracle과 재현 조건

- 브라우저: Agent Browser `0.31.1`, macOS arm64의 `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1.
- 관찰 항목: browser user agent, `getComputedStyle().gridTemplateColumns/Rows`, root/item `getBoundingClientRect()`.
- Chrome 결과는 [observation JSON](../spec/internal/evidence/c11-4-cyclic-percent-minmax-precomparison-2026-10-22/chrome156-observation.json)과 [화면](../spec/internal/evidence/c11-4-cyclic-percent-minmax-precomparison-2026-10-22/chrome156-minmax-cyclic-tracks.png)에 보존한다. 이 관찰은 위 독립 계산과 일치하지만 단일 Chrome 버전의 결과다.
- track list와 정수 pixel geometry는 exact 비교한다. runtime f32/화면 경계 표본의 허용치는 실행 근거에서 raw coordinate와 display scale에 맞게 고정한다.

## 구현과 판정 절차

1. 계획 PR 전에 별도의 실패 관점 20개로 계획 입력·규범 단계·예상값·미지원 경계를 검토한다. [계획 검토](./c11-4-cyclic-percent-minmax-plan-review-20.md)는 구현 검토에 재사용하지 않는다.
2. Rust projection/layout focused tests에서 `MinMax { min: 0px, max: 50% }`, used track, root/item frames, stage trace와 non-partial error behavior를 비교한다. 인접 track, gap, `calc()`, 다른 min/max 함수, 일반 CSS admission은 추가하지 않는다.
3. fixture FFI는 위 3개 scene ID만 allowlist한다. 노드 수·순서·style·revision이 다르거나 미등록 scene이면 부분 layout 없이 거부한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 3 scene의 root/item frame, 두 축 track trace, draw/present를 확인한다. item color 표본은 column의 `x=18px`과 row의 `y=12px`처럼 track 바깥·root 안 지점에서 읽고, definite control은 20px track 경계를 넘는 item을 확인한다. 앱에서 실제 border stroke를 그리지 않는 현재 C11.4 fixture renderer로 border paint 완료를 주장하지 않는다. PixelCopy는 Android SurfaceView 표본이며 iOS Metal drawable은 compositor 제출 전 표면이다.
5. 기본 product archive에서 신규 fixture symbol/ID/payload가 빠지고 fixture-test positive control에는 포함되는지 각각 검사한다. focused Rust tests, formatting, relevant Clippy/build와 두 simulator matrix의 실행 여부를 정확히 기록한다.
6. 구현/runtime 변경에는 계획 검토와 다른 실패 관점 20개를 남기고, 최종 PR 변경분은 별도 30개 관점으로 검토한다. 차이를 찾으면 수정 후 영향받는 항목을 다시 확인한다.
7. PR 본문은 C11.4를 전체 완료로 표시하지 않고 세 고정 장면, 화면, 실행근거와 미지원 범위를 적는다. 병합 전에 현재 required checks/merge state를 확인하고 Rebase and merge를 사용한다. Simulator 화면 링크는 Agent Browser에서 로드되는지 확인한 뒤 Agent Browser를 종료한다.

## 범위 밖

`minmax()`의 percentage minimum, `calc()`가 포함된 percentage maximum, `fit-content(%)`, 여러 percentage track, 같은 Grid에서 columns와 rows가 동시에 cyclic인 경우, nested/anonymous/replaced item, auto-repeat 결합, writing mode 변경, WPT, Android 실기기, hardware GPU 및 iOS compositor 이후 pixel은 검증 범위가 아니다. 구현 결과는 opt-in fixed fixture의 근거일 뿐 일반 CSS admission이나 C11.4 완료를 뜻하지 않는다.

## 구현 결과 · 2026-10-22

- `cyclic-minmax-column`은 Columns `[12px]` / Rows `[16px]`, `cyclic-minmax-row`는 Columns `[24px]` / Rows `[8px]`, `definite-minmax-column`은 Columns `[20px]` / Rows `[16px]`였다. Root/item frame은 계획 표와 exact 일치했다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 3/3 scene의 frame, 양 축 track trace, draw/present를 통과했다. PixelCopy는 6/6 표본, iOS Metal drawable은 compositor 제출 전 6/6 표본이 exact였다.
- 첫 Android 실행은 새 sample ID의 기대 RGB가 Android 검증 allowlist에 없어 surface summary가 mismatch를 냈다. RGB allowlist를 수정하고 APK를 다시 빌드해 세 장면 전체를 재실행했다. 실패 로그도 evidence에 보존했다.
- Rust focused test, fixture module 경계 검사, formatter, focused Clippy, Android/iOS Simulator fixture build를 통과했다. 기본 `spinon-ffi` release rlib에서는 새 module marker와 세 ID가 없고 `c04-runtime-gpu-fixture-test` positive control에는 모두 있었다. 이는 static archive 경계 결과이며 최종 release bundle 검증은 아니다.
- [구현 실패 관점 20개](../spec/internal/evidence/c11-4-cyclic-percent-minmax-runtime-2026-10-22/implementation-review-20.md)와 [PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-cyclic-percent-minmax-runtime-2026-10-22/pr-diff-review-30.md)는 계획 검토와 분리했다.
