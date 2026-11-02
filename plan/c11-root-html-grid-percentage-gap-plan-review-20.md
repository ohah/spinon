# C11.1 Root HTML Grid percentage gap 계획 적대 검토 20개

검토 대상은 [`c11-root-html-grid-percentage-gap.md`](./c11-root-html-grid-percentage-gap.md)와 Chrome 156 사전 비교 [`README`](../spec/internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/README.md)다. 아래 관점은 서로 다른 계산·admission·경계 실패를 확인하며 같은 결과를 표현만 바꾸어 반복하지 않는다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | `row-gap`을 width로, `column-gap`을 height로 해석 | 통과 — 계획의 `basis`/`shorthand` 입력과 독립값이 물리 축 대응을 분리하고 CSS Gaps §2.3을 인용한다. |
| 2 | percentage 기준을 viewport 또는 border box로 잘못 사용 | 통과 — root content box를 기준으로 명시하고 fixture에서 padding/border를 제거해 basis를 식별한다. |
| 3 | 0% gap을 `normal`/px 입력으로 잘못 분류 | 통과 — 별도 `zero` 장면에서 cascade 값과 used 0px 동작을 분리한다. |
| 4 | definite zero-size basis에서 75%를 비정상 입력으로 거부하거나 NaN 생성 | 통과 — `zero-basis`는 0px width, 0px tracks 및 두 item x=0을 판정값으로 고정한다. |
| 5 | 100%를 넘는 gap을 100%로 clamp | 통과 — 100px basis의 120% gap과 x=130px, scroll width 140px를 독립값과 Chrome 관측으로 대조한다. |
| 6 | `gap` shorthand의 첫째 값을 column, 둘째 값을 row에 연결 | 통과 — `5% 10%`에서 row=5px/column=20px와 두 축 track/frame을 각각 고정한다. |
| 7 | `grid-gap` legacy alias를 새 지원에서 누락 | 통과 — §2.4 별칭 근거와 shorthand 동일 geometry 대조 장면을 계획에 포함했다. |
| 8 | invalid negative declaration을 runtime error로 오해하거나 앞선 cascade 값 제거 | 통과 — valid 5px 뒤 invalid negative를 넣고 computed 5px와 x=15px를 기대한다. |
| 9 | `calc()`와 bare percentage를 같은 admission 경로로 허용 | 통과 — calc는 Chrome 관측을 남기되 Spinon 계산 결과 없이 정확한 gap property로 reject한다. |
| 10 | cyclic percentage gap의 intrinsic contribution 규칙을 definite final layout에 일반화 | 통과 — §2.3의 cyclic/final-layout 차이를 기록하고 indefinite/cyclic 입력을 제외한다. |
| 11 | named-area gate가 기존 0-gap 제한으로 새 feature를 계속 막음 | 통과 — 고정 2×2 area map에 percent gap 장면을 추가하고 spanning header의 gutter 포함 폭을 검사한다. |
| 12 | named-area gap 변경이 이미 기대한 px 지원/회귀를 잃음 | 통과 — area route의 기존 zero-only gate를 의도적으로 확장하며 독립 px control과 geometry를 포함한다. |
| 13 | row-sparse implicit row가 explicit rows와 다른 gap basis를 사용 | 통과 — definite 100px height, 25% implicit row 및 10% row gap을 함께 대조한다. |
| 14 | column-sparse relative implicit column extents에서 gap을 빼먹음 | 통과 — 50% implicit column, 10% column gap과 다음 column x=60px를 하나의 scene으로 검증한다. |
| 15 | computed spacing은 percentage인데 Taffy projection에서 px로 조기 변환 | 통과 — 현재 `LayoutGap`/spacing projection이 percentage를 표현하며, 구현에서 Taffy에 typed percentage를 전달하는 것을 명시한다. |
| 16 | resolved percentage나 gutter 수 곱이 f32 범위를 넘은 뒤 partial frame 발행 | 통과 — per-gap resolved 값과 현재 adapter가 계산하는 aggregate extent의 finite f32 검증 및 output 부재를 요구한다. |
| 17 | NaN, infinity, negative computed value가 low-level projection을 우회 | 통과 — 유한·비음수 admission과 직접 computed-style 입력 negative tests를 완료 기준에 둔다. |
| 18 | unsupported percentage gap이 `StyleLayoutOutput` 일부를 남기고 반환 | 통과 — root/property 오류와 partial output 부재를 확인하도록 명시했다. |
| 19 | 공유 spacing 변경이 Flex나 unrelated Root HTML profiles로 새어 나감 | 통과 — Root HTML Grid 전용 gate를 명시하고 Flex/다른 profile 비변경을 회귀 항목으로 둔다. |
| 20 | 시각 overflow·simulator smoke를 표면 pixel equality나 전체 Grid 완료로 과장 | 통과 — screenshots는 시각 보조로 분류하고 same-hash Android/iOS, root 밖 overflow, C11.1/C11 미완료 및 WPT 제외를 명시한다. |

결론: 위 20개 관점에서 계획을 수정할 차단 결함은 남지 않았다. 구현 시 review #16의 aggregate extent 경계를 고정 Rust 입력과 no-partial-output assertion으로 직접 시험한다. 계획 검토는 이후 구현/runtime 20관점 또는 PR delta 30관점으로 재사용하지 않는다.
