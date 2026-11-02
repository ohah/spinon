# C11 parsed Root HTML Grid typed math gap 계획 적대 검토 20개

검토 대상은 [typed math gap 계획](./c11-root-html-grid-math-gap.md), [19 scene Chrome 156 사전 비교](../spec/internal/evidence/c11-root-html-grid-math-gap-2026-11-03/README.md), 입력 fixture와 원 관측 JSON이다. 각 항목은 서로 다른 axis, typed math, extent, admission, publication 또는 근거 경계를 확인한다. 구현 검토나 PR delta 검토에 재사용하지 않는다.

첫 검토 중 pure length 식이 indefinite axis에서 기존 `LayoutCssMathValue::resolve`에 전달되면 basis 검사로 거부될 수 있음을 발견했다. 계획에 gap 전용 basis 0 규칙을 추가했다. 또한 layout 오류 시의 원자성 검증을 style projection 오류와 혼동한 문장을 고쳐 실행 경계를 분리했다.

변경분 재검토 중 19-scene Chrome oracle에는 Grid가 여럿 있고 named-area route는 단일 visible Grid만 받는다는 상호작용을 확인했다. 전체 oracle을 앱 입력으로 쓰지 않고, 같은 Android/iOS run에 쓰는 별도 단일 Grid fixture/hash를 고정했다. cascade/layout 단위 검사는 각 scene을 isolated case로 다룬다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | `row-gap`이 width, `column-gap`이 height를 기준으로 퍼센트를 계산 | 통과 — 계획·fixture가 각 축과 `calc-mixed-row`/`calc-mixed-column`을 분리하고 item y/x를 각각 고정한다. |
| 2 | percentage basis로 viewport나 border-box를 사용 | 통과 — 계획은 대응 content-box 축을 규범 기준으로 고정하고 독립 계산 표가 200px/100px 축 값으로 계산한다. |
| 3 | definite 0px 축을 indefinite로 오인 | 통과 — `calc-zero-basis`는 `75% × 0 + 2px`, 두 zero-width item x=0/2를 고정한다. |
| 4 | percentage 없는 pure length math도 definite axis를 요구 | 통과 — 계획은 gap math에만 resolver basis 0을 전달해 이 경계를 처리하고 다른 property 의미를 바꾸지 않는다. |
| 5 | percentage를 포함한 mixed/min/max 식이 auto·indefinite 축에서 조용히 0으로 계산 | 통과 — definite finite nonnegative axis와 기존 min/max 제약이 없으면 reject하도록 요구한다. |
| 6 | `calc()` 중간 항이 음수지만 최종 gap이 유효한 입력을 중간값 검사로 거부 | 통과 — 최종 top-level expression range만 property 정책으로 clamp/censor하고 typed dimension은 별도로 검사한다. |
| 7 | CSS literal `gap:-10px`을 valid negative `calc()`와 같은 cascade 처리 | 통과 — 계획은 invalid literal cascade 보존 회귀와 음수 계산식 range checking을 분리한다. |
| 8 | wrong node/property, 누락·중복 ID 또는 잘못된 math 식이 부분 layout을 남김 | 통과 — layout binding/error 검증과 전체 frame publication atomicity를 별도 확인하도록 명시한다. |
| 9 | `gap: row column`의 순서를 바꾸거나 한 값 shorthand를 잘못 확장 | 통과 — 독립 2×2 frame과 axis별 산출값을 두 개의 longhand 결과와 대조한다. |
| 10 | `grid-gap` legacy alias가 math longhand 경로를 우회하거나 중복 gap 적용 | 통과 — 동일 geometry oracle과 단일 computed-longhand binding을 요구한다. |
| 11 | 120%를 100%로 제한하거나 nonnegative 범위라는 이유로 퍼센트를 0–100%로 자름 | 통과 — `calc-over-100`의 121px gap, x=131, scroll width 141을 분리 판정한다. |
| 12 | `min()`/`max()`를 basis resolve 전 비교하여 다른 항을 선택 | 통과 — 폭 200px에서 15%=30px인 관측으로 20/30px 결과를 각각 고정한다. |
| 13 | `clamp()`가 min > max인 경우 CSS의 minimum 우선 규칙을 잃음 | 통과 — `calc-clamp-min-wins`는 `clamp(30px,7px,10px)`의 30px gap과 x=40을 고정한다. |
| 14 | product/divide를 length-percentage dimension rules 없이 처리 | 통과 — `calc-product`, `calc-divide`를 독립 산술 결과와 computed serialization 모두로 고정한다. |
| 15 | NaN을 그대로 Taffy 또는 frame geometry에 전달 | 통과 — `calc-nan`은 Chrome `0px`/x=10 관측과 existing property censor 경계를 고정한다. |
| 16 | 양의 무한대를 f32 infinity로 전파하거나 좌표 상한 관측을 일반 parity로 과장 | 통과 — `calc-infinity`를 finite range stress로 분리하고 Chrome track/coordinate clip을 normal geometry claim에서 제외한다. |
| 17 | implicit row 확장에서 gap을 explicit row 수만큼만 더함 | 통과 — explicit+implicit 두 행의 used gap 7px와 다음 행 y=27을 함께 검사한다. |
| 18 | sparse column-flow의 implicit column 크기와 math gap basis가 달라 다음 column 원점이 어긋남 | 통과 — 200px column basis, 100px implicit column, 12px gap, x=52를 고정한다. |
| 19 | named-area spanning item이 내부 gutter를 누락 또는 두 번 포함 | 통과 — 128px header span과 lower-area x/y를 각각 정확한 geometry로 대조한다. |
| 20 | typed math admission이 Flex/다른 profile로 번지거나 multi-Grid oracle이 named-area route에서 reject | 통과 — Root HTML 범위 gate, isolated per-scene 단위 입력, Android/iOS 공통 단일 Grid fixture hash를 고정하고 화면을 native pixel equality로 해석하지 않는다. |

결론: 위 20개 계획 실패 경로에 차단 결함은 남지 않았다. 구현은 현재 typed AST와 owner binding만 재사용하며, 수치 경계의 무한대 track clip은 일반 frame parity로 승격하지 않는다. 계획 검토는 구현/runtime 20개 및 실제 PR delta 30개와 별도다.
