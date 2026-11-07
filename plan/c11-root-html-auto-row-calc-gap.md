# C11.4 parsed Root HTML auto-row alignment `calc()` `row-gap`

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [선행 percentage `row-gap`](./c11-root-html-auto-row-percentage-gap.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 제한 구현 PR [#1246](https://github.com/ohah/spinon/pull/1246)은 `2026-11-07T01:48:53Z` Rebase and merge됐고 merge SHA는 `ea86bd1ceea03df21ed1a5ba89b8b2aaad965948`다. Agent Browser Chrome 157 사전 비교 120/120과 독립 기대값 1,500/1,500 좌표 대조, Layout 12/12, projection 4/4, RuntimeSession row-gap regression 6/6, Android API 37 ARM64 AVD/iPhone 17 Pro iOS 26.2 Simulator positive-space·overflow 실행을 마쳤다. OS PNG pixel bounds에서 calc overflow의 마지막 item이 Grid 아래에 있는 것을 확인했다. 별도 [계획 실패 관점 20개](./c11-root-html-auto-row-calc-gap-plan-review-20.md), [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-root-html-auto-row-calc-gap-runtime-2026-11-07/implementation-runtime-review-20.md), [최종 PR diff 실패 관점 30개](../spec/internal/evidence/c11-root-html-auto-row-calc-gap-runtime-2026-11-07/pr-diff-review-30.md)를 통과했다. 이 작업은 C11.4/C11 완료나 일반 CSS 지원 선언이 아니다.

## 목적과 규범

선행 PR [#1243](https://github.com/ohah/spinon/pull/1243)은 기존 `RootHtmlAutoRowStretchV1`에서 definite 40px block-size 기준 direct percentage `row-gap`을 연결했으며, `calc()`는 실패 원자성을 위해 거부했다. 이 후속은 동일 Grid shape에서 계산 결과가 유한한 `<length-percentage [0,∞]>`를 만드는 제한 two-term CSS math만 추가한다.

- [CSS Gaps Level 1 §2.3](https://www.w3.org/TR/css-gaps-1/#percentages-in-gap-properties): definite Grid contents layout에서 percentage gap은 해당 축의 content-box 크기를 기준으로 한다. [`row-gap` 정의](https://www.w3.org/TR/css-gaps-1/#propdef-row-gap)는 비음수 `<length-percentage>`다.
- [CSS Grid Level 1 §10.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters): 행 사이에는 `row-gap` gutter가 놓인다.
- [CSS Values and Units Level 4 §5.1, §10.12](https://www.w3.org/TR/css-values-4/#calc-range): 지정 범위가 있는 수학 계산은 해당 범위에 맞게 clamp된다. 이 작업은 음수 계산 결과가 zero used gap으로 처리되는 Chrome 동작도 기대값과 대조한다.

## 고정 입력과 관측

[사전 비교 evidence](../spec/internal/evidence/c11-root-html-auto-row-calc-gap-precomparison-2026-11-07/README.md)는 40×40px content box, 40px column, 10px explicit row, auto implicit rows, direct child 2개 또는 3개, `column-gap:0`, `align-items:normal`, 기존 `align-content` 10개 값을 고정한다. Agent Browser HeadlessChrome 157.0.0.0 / DPR 1에서 6 expressions × 2 child-count variants × 10 alignments = 120 scene을 관찰하고 독립 식과 1,500 coordinate를 비교했다. 최대 차이는 `0.0208333334` CSS px로 `1/32` CSS px 허용치 이내다.

기존 projection profile에서는 positional/distributed `align-content`에 세 child(명시 행 item 하나와 implicit auto-row item 둘)가 필요하다. 따라서 두 child의 열 가지 alignment와 세 child의 열 가지 alignment는 Layout oracle에서 모두 검사하고, RuntimeSession representative matrix는 기존 admission을 지키는 세 child 입력으로 고정한다.

| 계산식 | 40px basis의 used gap |
|---|---:|
| `calc(1px - 10%)` | 0px (음수 결과 clamp) |
| `calc(6.25% + 2px)` | 4.5px |
| `calc(2px + 6.25%)` | 4.5px |
| `calc(25% - 2px)` | 8px |
| `calc(10% - 1px)` | 3px |
| `calc(120% - 2px)` | 46px |

독립 계산은 percentage 항 × 40px에 signed px 항을 더한 뒤 비음수 row-gap 범위로 clamp한다. 그 결과에서 기본 row 크기와 gutter를 빼 free space를 구하고, `normal`/`stretch`의 auto-row 증가, 위치 정렬과 positive-space 분산 정렬을 따로 계산한다. expected 생성은 Chrome 관측값을 참조하지 않는다. 브라우저 관측·fixture hash·전체 실행 결과는 위 evidence에 기록했다.

## 제한 구현 범위

1. `RootHtmlAutoRowStretchV1`의 기존 shape와 allowlist를 유지한다: fixed 40×40px content box, 40px column, 10px explicit row, auto implicit rows, row flow, visible auto-placed direct item 2–3개, `column-gap:0`, `align-items:normal`, 기존 `align-content` 열 값 10개.
2. `row-gap`은 기존 bounded px/direct percentage에 더해 `LayoutLengthPercentage::Calc(id)`를 받는다. 계산식은 finite `<length>` 한 항과 finite `<percentage>` 한 항의 2-term sum이며, 항 순서는 무관하고 한 항의 단항 음수 표기를 허용한다. 계산 record는 해당 Grid node 소유이며 property가 `RowGap`이어야 한다.
3. Percentage basis는 검증된 40px content-box 높이다. 두 종류의 typed value가 기존 Taffy sizing/alignment 경로에서 한 번만 gutter로 계산되도록 한다. 별도 수동 gutter 보정은 넣지 않는다. Math result에는 기존 CSS math resolver의 layout range (`33,554,428` CSS px 최대)를 적용한 뒤 non-negative row-gap 범위를 적용하고, 결과와 총 Grid extent를 검사한다. 유한한 큰 항은 이 기존 CSS range로 제한되므로 그 자체를 오류로 보지 않는다.
4. 잘못된/누락/중복 math ID, 다른 node/property 바인딩, 지원하지 않는 AST, NaN/infinity, basis 불일치, 다른 item/Grid shape는 기존 node/property 진단으로 실패시킨다. 총 Grid extent 검사도 유지한다. 실패한 입력은 부분 layout을 발행하지 않고, 이전 유효 장면과 후속 복구 입력을 보존한다.
5. 기존 px 및 direct percentage regression을 유지한다. 새 앱 fixture가 필요하면 `*-fixture-test` opt-in 경로만 사용하고, 기본 artifact에 fixture scene ID/payload가 없는지 positive control과 함께 확인한다.

## 검증 관문

- Projection: authored calc tree가 typed cascade에서 projection까지 보존되는지와 node/property 출처를 검사한다. CSS variable을 새로 허용해 범위를 넓히지 않는다.
- Layout: 위 120-scene oracle·독립 matrix와 모든 geometry를 비교한다. missing/misbound ID, 비지원 expression tree, 음수 최종 계산값, representability 한계와 오류 귀속을 주입한다. fixed px, direct percentage, overflow 회귀도 유지한다.
- RuntimeSession: 동일 calc expression을 기본 Root HTML 경로에서 실행한다. `6.25% + 2px` positive space, `1px - 10%`의 0px clamp, `120% - 2px` overflow를 independent expected와 대조한다. 지원하지 않는 math를 media override로 주입해 원자적 실패와 valid style 복구를 확인한다.
- Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 `calc(6.25% + 2px)`/`normal` positive-space와 `calc(120% - 2px)`/`center` overflow를 각각 같은 source hash로 실행한다. Overflow 화면의 PNG pixel bounds를 검증한다. Screenshot은 visual smoke 및 post-compositor OS screenshot pixel 관측으로만 기록한다. PixelCopy, Metal drawable readback, Android 실기기, hardware GPU 또는 compositor 이후 GPU surface parity는 주장하지 않는다.
- 변경 관련 layout/projection/runtime focused gates, 변경 crate Clippy, `cargo fmt --all -- --check`, `git diff --check`를 실행한다. 무관한 baseline 실패는 별도로 기록한다.
- 구현 후 별도 implementation/runtime 실패 관점 20개를 검토한다. 최종 PR diff는 서로 다른 실패 관점 30개 이상으로 검토하고 저장소 PR 양식, 가능한 simulator 증거, 최종 head/status 확인을 거친다.

## 제외 범위

임의 `calc()/min()/max()/clamp()` AST, 같은 dimension의 여러 항, 비유한 math operand, indefinite/cyclic basis, `column-gap` math, nested/area/repeat tracks, 3개 초과 child, 일부만 배치한 item, 다른 Root HTML/C11 profile, 일반 CSS admission, WPT 적합성 및 전체 C11은 제외한다. authored negative direct `row-gap`의 parser recovery는 별도 계약으로 남긴다.
