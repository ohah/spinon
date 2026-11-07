# C11.4 Root HTML fit-content auto-row 목록 · PR diff 실패 관점 검토 30개

**검토 기준:** `feat/c11-4-root-html-fit-content-auto-row-list-20261108`의 기준 `e788203c304b72473f5b43d3195646dc12373e4e` 대비 최종 소스·fixture·계획·증거 변경. 검토에는 테스트를 책임별 하위 모듈로 나눈 최종 diff와 빈 목록/지원 밖 숫자 negative assertion을 포함했다. 계획 검토 20개와 구현/runtime 검토 20개를 재사용하지 않고 별도 경로를 확인했다.

| # | 공격 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | 빈 `grid-auto-rows` 목록이 유효 profile로 선택돼 track 없는 Taffy 입력으로 전달된다. | Projection과 layout의 `supports_track_list`가 빈 목록을 먼저 거부한다. 두 계층의 focused negative assertion이 추가됐다. **차단 없음.** |
| 2 | fixed-px/percentage만 있는 목록이 fit-content 후보로 잘못 라우팅된다. | 목록에 `FitContent`가 하나 이상 있어야 profile을 선택한다. Projection candidate assertion과 layout rejection을 확인했다. 일반 route의 처리까지 이 후보가 확장하지 않는다. **차단 없음.** |
| 3 | `GRID_MAX_TRACKS + 1` 목록을 부분 truncate하거나 큰 할당 전에 넘긴다. | Projection·layout이 같은 상한을 확인하고 상한 초과 전체를 거부한다. 각각 상한+1 negative assertion이 통과했다. **차단 없음.** |
| 4 | `auto`, `fr`, `min-content`, `max-content`가 섞여도 나머지 목록만 계산된다. | Projection의 typed allowlist가 항목 전체를 검사하고, layout은 `supports_track_list`에서 같은 종류를 거부한다. **차단 없음.** |
| 5 | `minmax()`나 percentage-bearing `calc()`가 우회해 supported size로 취급된다. | `minmax` typed value는 projection negative matrix에서 거부되고, `Calc` cap은 기존 singleton invalid-limit 회귀에서 거부된다. **차단 없음.** |
| 6 | 음수, NaN 또는 infinity가 fit-content/direct track에 들어간다. | Projection과 layout 모두 finite·non-negative 값을 요구한다. fit-content cap, fixed-px, direct-percentage invalid case가 focused tests에 있다. **차단 없음.** |
| 7 | Projection이 mixed list의 종류나 순서를 바꾼다. | `FitContent(20%)`, `LengthPx(12)`, `Percentage(10%)`의 typed 결과와 RuntimeSession computed serialization `fit-content(20%) 12px 10%`를 각각 비교한다. **차단 없음.** |
| 8 | explicit 10px row가 목록 첫 항목을 소비해 첫 implicit row가 두 번째 항목으로 시작한다. | Layout trace와 각 item frame은 explicit row 다음 y=10px에서 첫 목록 entry를 적용한다. 세 값 pattern과 runtime scene이 이를 고정한다. **차단 없음.** |
| 9 | 두 항목 pattern이 두 번째 implicit row 뒤 반복되지 않는다. | Layout expected rows `[20, 34, 20, 6]px`와 runtime repeated item frame을 확인한다. **차단 없음.** |
| 10 | 세 항목 pattern이 세 번째 항목 뒤 잘못 순환한다. | Layout expected rows `[10, 20, 50, 10]px`가 전체 순환 주기를 확인한다. **차단 없음.** |
| 11 | fit-content와 fixed-px를 섞을 때 fixed entry를 intrinsic sizing으로 계산한다. | `[fit-content(20%), 12px, fit-content(80%)]`에서 row trace `[20, 12, 50, 6]px`와 item/payload frames를 대조한다. **차단 없음.** |
| 12 | direct percentage를 fit-content cap으로 바꾸거나 payload 높이만큼 축소한다. | `[fit-content(20%), 12px, 10%]`는 definite 100px block basis로 `[20, 12, 10, 20]px`를 만들며 8px payload와 10px row를 따로 확인한다. **차단 없음.** |
| 13 | 0% cap을 최소 양수로 올리거나 item을 제거한다. | Layout과 runtime 모두 첫 implicit item의 0px frame을 유지하고 34px payload overflow를 별도로 보존한다. **차단 없음.** |
| 14 | 120%를 100%로 clamp한다. | `fit-content(120%)` row는 payload의 34px max-content에서 끝난다. 값이 100%로 재작성되지 않는 frame assertion을 확인했다. **차단 없음.** |
| 15 | minimum contribution이 첫 fit-content cap보다 커질 때 cap이 minimum을 잘라낸다. | 허용된 3-item list shape의 `min-height:28px`에서 20% cap보다 큰 28px row와 34px nested payload를 Layout/RuntimeSession에서 대조한다. **차단 없음.** |
| 16 | minimum floor를 다른 list topology나 implicit item에 무제한 허용한다. | 28px floor는 3-child shape의 첫 implicit item에만 허용하고, 5-child pattern 및 다른 shape에서는 거부한다. layout negative assertion을 확인했다. **차단 없음.** |
| 17 | explicit track/item이 바뀌어도 동일 profile로 처리한다. | Projection과 Layout은 40px column, 10px explicit row와 첫 10px item을 요구한다. 잘못된 explicit track 및 item은 기존 rejection gate에 남는다. **차단 없음.** |
| 18 | 임의 개수의 direct child가 이 fixture profile을 통과한다. | 보이는 direct item 수는 기존 3개 또는 5개만 허용하고 owner count와 대조한다. 4개·6개 및 그 밖의 수는 실패한다. **차단 없음.** |
| 19 | hidden subtree나 anonymous ownership으로 보이는 item 수가 계층마다 달라진다. | Style projection과 Layout이 hidden child를 제외한 visible shape를 각각 세고 Grid item owner 개수도 비교한다. nested Grid owner는 별도 후보에서 거부한다. **차단 없음.** |
| 20 | text node, non-`div` element 또는 지원 밖 DOM namespace가 direct item으로 섞인다. | Root HTML validator는 공백 text만 건너뛰고 HTML namespace `div`만 item으로 받는다. Layout의 non-collapsible text gate도 유지된다. **차단 없음.** |
| 21 | auto-sized item이 실제로는 positioned 또는 explicit-placement item이다. | 각 implicit item에 auto grid placement와 static positioning을 요구한다. position/line placement는 projection 및 layout shape 검사에서 거부된다. **차단 없음.** |
| 22 | intrinsic item payload가 없거나 둘 이상이라 다른 contribution이 합쳐진다. | 각 auto-sized item은 직접 child 하나만 가져야 하며 그 child는 fixed-px payload여야 한다. child 수 0 또는 2는 fail-closed한다. **차단 없음.** |
| 23 | 임의 높이·중첩 자손·nested Grid가 payload allowlist를 우회한다. | payload 높이는 고정 fixture allowlist만 허용하고, nested Grid와 payload descendants는 거부한다. **차단 없음; 임의 HTML 지원을 주장하지 않음.** |
| 24 | item의 min/max, margin, padding, border, display 또는 self-alignment가 contribution을 바꾼다. | Style projection과 Layout은 Block/auto-size/zero-edge/auto self-alignment의 제한 shape를 확인한다. 미지원 constraint는 전체 layout 실패로 처리한다. **차단 없음.** |
| 25 | 두 Grid owner가 있거나 Grid owner 아래에 예상 밖 container가 들어간다. | Style validator는 visible candidate와 전체 Grid owner가 각각 하나인지 확인한다. item/payload에 추가 Grid container가 있으면 거부한다. **차단 없음.** |
| 26 | percentage basis가 viewport/surface 높이 또는 padding/border가 포함된 크기로 바뀐다. | Owner는 definite 40×100px border-box, zero padding/border/gap을 요구하고 percentage test는 100px content-box basis를 사용한다. **차단 없음.** |
| 27 | column flow, repeat, named lines, areas, implicit columns 또는 nonzero gap이 이 row profile에 섞인다. | Track topology, auto-flow, auto-columns, line names, repeat/area 부재와 zero row/column gap을 style projection 및 Layout에서 각각 검사한다. **차단 없음.** |
| 28 | unsupported media/style revision 뒤 partial frame/render가 남거나 같은 session이 복구하지 못한다. | RuntimeSession test는 실패 revision에서 completed layout/render 부재를 확인하고 같은 session의 다음 light revision에서 기대 frame 복구를 확인한다. **차단 없음.** |
| 29 | 기존 singleton px/% 동작이나 public API/ABI/internal contract version이 바뀐다. | 기존 singleton cap·6px probe·length calc normalization tests가 유지된다. 공개 API/ABI 변경은 없고 내부 계약 `0.1.0` 및 C11.4/C11 체크는 유지한다. **차단 없음.** |
| 30 | focused 결과·Simulator 화면을 전체 crate green, 표면 pixel parity, 실기기 또는 C11 완료로 확대한다. | 현재 focused 결과는 Layout 8/8, projection 3/3, RuntimeSession 6/6, changed-crate Clippy·rustfmt 통과다. 저장된 넓은 `root_html_grid` 탐색은 C11.5 fixed-track auto-size/stretch 오류 5개로 green이 아니며 별도 한계로 남긴다. Android API 37 AVD/iPhone 17 Pro Simulator 화면은 visual smoke이고 surface readback·실기기 결과가 아니다. **범위 제한 명시.** |

## 판정

이 diff에서 C11.4 후보를 막는 추가 결함은 찾지 못했다. Empty/no-fit/oversized/invalid-numeric 목록, ordering/cycle, floor/overflow, DOM/style boundary, 실패 원자성 및 기존 singleton 회귀를 코드와 focused evidence로 대조했다. 넓은 exploratory test의 다섯 C11.5 실패는 숨기거나 통과로 계산하지 않는다. PR/merge와 merge 후 roadmap 동기화는 이 검토 문서만으로 완료된 것으로 처리하지 않는다.
