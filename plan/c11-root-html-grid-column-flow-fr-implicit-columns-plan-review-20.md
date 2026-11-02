# C11.1 `fr` implicit columns 계획 실패 관점 20개

계획 PR 작성 전 비교 fixture와 계획 범위를 다음 20개 독립 관점에서 공격적으로 점검했다. Chrome 관측은 규범 기대값과 분리했고, 발견한 계획 결손은 본문과 expected fixture에 반영했다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 계획 PR만으로 C11.1 또는 공식 완료 수가 올라가는가 | 통과 — 계획만 병합할 수 있고 체크박스와 `72/213`은 그대로 둔다. |
| 2 | definite width를 used width나 viewport width로 오인하는가 | 통과 — fraction/percentage basis는 같은 computed Grid owner의 direct finite non-negative px width로 고정하고 min/max, aspect ratio, math를 거부한다. |
| 3 | zero width를 indefinite width로 잘못 거부하는가 | 통과 — `width:0px`는 definite basis로 유지한다. |
| 4 | column gap을 percentage basis에서 빼거나 fr free space에서 두 번 차감하는가 | 통과 — percentage는 전체 content width, fr는 fixed/percentage tracks 및 각 column gap을 한 번 뺀 공간을 사용한다. |
| 5 | `grid-auto-columns` 목록이 첫 explicit column부터 시작하는가 | 통과 — 반복 pattern은 첫 implicit column부터 적용한다. |
| 6 | scalar `1fr`가 한 번만 적용되는가 | 통과 — 네 implicit column의 scalar repetition scene을 고정했다. |
| 7 | px와 fr 목록에서 fixed breadth가 flexible leftover를 잘못 바꾸는가 | 통과 — `10px 1fr` two-cycle control의 각 fr track은 62.5px다. |
| 8 | px/%/fr 혼합에서 percentage basis와 gap이 섞이는가 | 통과 — 300px basis의 `10px 10% 1fr` 반복을 10/30/57.5px 산술로 분리한다. |
| 9 | factor 합 1 미만을 모두 채우도록 정규화하는가 | 통과 — 두 `0.25fr` tracks는 각 20px이고 trailing 40px를 남긴다. |
| 10 | `0fr`를 무조건 0px track으로 기대하는가 | 보정 후 통과 — 고정 10px item의 automatic minimum base를 고려해 zero-factor track 10px, 나머지 70px로 고정했다. |
| 11 | zero-factor freeze/retry에서 다른 fr track이 남은 공간을 받지 못하는가 | 통과 — `0fr 1fr`은 10px base를 freeze한 뒤 남은 70px을 두 번째 track에 준다. |
| 12 | f32 factor sum overflow가 유효한 유한 fr을 잘못 거부하거나 0으로 만드는가 | 보정 후 통과 — auto-column expansion 경로에서 4×`1e38fr` `[30,30,30,30]`을 검사하고 기존 vendored Taffy f64 accumulator를 유지한다. |
| 13 | Chrome fractional serialization 오차가 규범 산술 오차로 오인되는가 | 보정 후 통과 — exact rational 기대와 1/64px DOMRect 위치, 네 자리 computed track 문자열을 별도 보존하고 0.02px 허용치를 명시했다. |
| 14 | negative, NaN, infinite fraction이 CSS fixture만으로 검사되는가 | 통과 — typed projection/layout rejection matrix에서 직접 변조한 값을 별도로 검사한다. |
| 15 | CSS syntax-invalid fallback과 valid-but-unsupported track을 혼동하는가 | 통과 — 두 경로를 별도 rejection/control로 확인한다. |
| 16 | implicit track cap 전에 큰 vector를 할당하는가 | 통과 — 10,000-track cap과 pattern expansion 예약 실패 atomicity를 유지한다. |
| 17 | Grid item width를 track width와 동일하다고 보는가 | 통과 — fixed 10px item frame과 used track size를 별도 비교한다. |
| 18 | ID/class rename 또는 DOM order 변경으로 semantic identity에 의존하는가 | 통과 — rename control과 source-order reversal을 별도 layout mapping으로 검사한다. |
| 19 | Chrome 관측이나 simulator screenshot을 native pixel parity·WPT 완료로 확대하는가 | 통과 — Agent Browser는 oracle, AVD/Simulator는 draw/present까지만 구분하고 pixel equality는 readback할 때만 주장한다. |
| 20 | bounded Root HTML slice가 general CSS 또는 C11 완료로 게시되는가 | 통과 — unsupported grammar, API/ABI, fixture FFI 및 C11 parent checkbox를 범위 밖으로 고정했다. |

계획 검토에서는 zero-factor automatic minimum, fractional observation precision, large-factor regression의 세 누락 위험을 확인해 계획에 보완했다. 구현 시작 전 계획 PR 변경분은 별도의 PR diff 검토를 거친다.
