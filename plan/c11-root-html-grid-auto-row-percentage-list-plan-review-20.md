# C11.1 Root HTML `grid-auto-rows` percentage list 계획 적대 검토

**대상:** [`c11-root-html-grid-auto-row-percentage-list.md`](./c11-root-html-grid-auto-row-percentage-list.md)와 Chrome 156 사전 비교 자료
**시점:** 계획 PR 전. 구현은 시작하지 않았다.

| # | 독립 실패 관점 | 검토 결과 및 요구 조치 |
|---:|---|---|
| 1 | 다른 CSS profile 또는 fixture 전용 route에 Root HTML 지원을 섞음 | 기본 parsed Root HTML profile 하나로 한정하고 opt-in fixture profile은 변경하지 않는다. |
| 2 | percentage basis를 viewport나 grid의 외곽 border-box로 오인 | CSS Grid의 definite inner block size를 사용한다. 현 Root HTML profile의 0 border/padding 조건을 확인한다. |
| 3 | explicit rows를 auto-row 목록과 함께 순환 | explicit rows는 그대로 두고 첫 implicit row에서 목록 index 0으로 시작하는 기대값을 고정한다. |
| 4 | 혼합 px/% 목록의 선언 순서를 바꾸거나 한 단위만 사용 | `15px 25%`를 포함하고 expanded rows 및 frames를 각 순서대로 기록한다. |
| 5 | row-gap을 percentage basis에서 빼거나 percentage 뒤에 이중 차감 | `25%`의 120px basis가 30px이고 gap은 별도로 적용되는 관측·계산을 둔다. |
| 6 | percentage가 100%를 넘으면 clamp 또는 reject | definite axis에서는 `120%`를 120px로 계산하고 visible overflow frame을 보존한다. |
| 7 | 0% implicit track을 auto/min-content 크기로 늘림 | 0px used row와 0px item height를 별도 경계로 확인한다. |
| 8 | 0px definite height를 indefinite size로 분류 | Chrome 사전 비교의 definite-zero scene에서 50% implicit row가 0px로 resolve되는 것을 확인하고, 구현 수락 matrix에서 indefinite height rejection과 분리한다. |
| 9 | indefinite/cyclic percentage를 지금의 0px 혹은 min-content로 가장 | 현재 slice에서는 definite height만 admission하고 indefinite 축은 fail closed로 둔다. |
| 10 | percentage를 `minmax()`, `calc()` 또는 intrinsic track 안에 간접 허용 | 직접 px/% typed values만 추가하고 조합 문법과 intrinsic track은 거부한다. |
| 11 | fixed scalar px 또는 기존 px 목록이 회귀 | scalar 및 목록 regression을 기존 oracle과 동일한 test group에 유지한다. |
| 12 | percent auto row를 무한 횟수/빈 목록과 결합 | 비어 있지 않은 목록, expanded track 및 10,000 track cap을 먼저 검증한다. |
| 13 | `grid-auto-flow:column|dense` 또는 implicit columns가 딸려 들어옴 | 기존 row-only placement와 auto columns 계약을 유지한다. |
| 14 | named area + implicit percentage row 조합의 제한이 풀림 | area와 implicit row sizing의 조합은 계속 거부한다. |
| 15 | negative, non-finite, f32 범위 초과 값이 Taffy에 전달 | percentage×basis의 개별 track 및 확장된 track/gutter 합을 확인하고 overflow 전 layout을 거부한다. |
| 16 | percentage declaration 오류와 유효하지만 제한 밖 computed value를 혼동 | syntax-invalid fallback과 valid-but-indefinite/profile rejection을 서로 다른 assertion으로 둔다. |
| 17 | 화면 밖 item frame을 root 안으로 clamp하거나 clip | `120%` 경계의 root-relative frame과 visible-overflow 전제를 비교한다. |
| 18 | unsupported input 또는 할당 실패 뒤 partial frame이 게시 | unsupported, cap, failpoint에서 root/item frame 미게시와 후속 요청 복구를 확인한다. |
| 19 | ID/class 선택자나 source revision이 track 계산을 결정 | runtime에 rename/reordered controls, revision identity 및 같은 입력 hash를 둔다. |
| 20 | Chrome 관찰이나 simulator 화면을 WPT, surface pixel, 실기기 증거로 확대 | evidence 종류를 분리하고 readback 전 screenshot을 visual evidence로만 표기한다. |

## 반영

관점 2·5·6·7·9·15·18·20을 계획의 기대값·입력 제한·오류 원자성·근거 경계에 직접 반영했다. 이 표는 계획 검토이며 구현/runtime 결과가 아니다.
