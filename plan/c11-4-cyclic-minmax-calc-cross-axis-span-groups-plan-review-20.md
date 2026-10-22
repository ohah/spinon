# C11.4 교차 축 span groups 계획 실패 관점 검토 20개

대상: 구현 계획과 계획 전 고정한 normative oracle, 재현 fixture, 플랫폼 판정 기준. 기존 양축 span text 및 overlapping-span slice와의 경계를 확인한다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | PR #506의 단일 text item 사례를 다른 이름으로 반복하는가? | 두 direct item이 서로 다른 column span으로 가운데 track을 공유하고, 그 planned-increase maximum을 함께 처리하도록 범위를 고정했다. |
| 2 | 두 item이 실제 같은 column track과 row tracks를 span하는가? | A는 columns `1/3`, B는 `2/4`; 둘 다 rows `1/3`으로 배치된다. |
| 3 | reverse scene에서 placement가 함께 바뀌어 순서 비교가 무효가 되는가? | reverse scene은 두 child의 DOM 순서만 바꾸고 class placement와 IDs를 보존한다. |
| 4 | 최초 `XX XXXXXXX` 초안이 실제로 cross-axis wrapping을 만드는가? | 만들지 않는다. definite case에서 automatic minimum이 70px로 유지되어 line count가 동일했음을 기록하고 최종 oracle에서 제외했다. |
| 5 | 최종 A text의 min/max-content 및 line threshold가 맞는가? | Ahem 10px에서 `AA A A`는 min/max 20/60px이고, 30px 경계를 넘어 intrinsic span은 2줄, definite span은 3줄이다. |
| 6 | B가 flex sizing 또는 text wrapping으로 A의 contribution을 오염시키는가? | B text `XXXX`는 40/40px이며 fixed preferred box 40×10px라 두 scene에서 한 줄·40px contribution이다. |
| 7 | min-content-constraint bases를 최종 used Columns로 잘못 보고하는가? | 기대값에서 `[9,19,19]` constraint bases와 percentage 재해석 뒤 최종 `[15.671875,15.671875,15.65625]`를 분리한다. |
| 8 | cyclic `calc()`에서 absolute `2px` 항을 잃는가? | intrinsic pass에서 percentage component를 cyclic zero로, absolute 2px를 initial base로 보존한다. |
| 9 | span gap을 누락하거나 두 번 빼는가? | 각 span contribution에서 gutter 2px를 정확히 한 번 빼고 전체 root width에는 두 gutter를 포함한다. |
| 10 | same-track planned increase를 합산해 DOM 순서 의존성을 만드는가? | A `[7,7,0]`, B `[0,17,17]` 제안의 per-track maximum `[7,17,17]`을 사용한다. |
| 11 | intrinsic min-content root size를 final column list의 단순 합으로 혼동하는가? | 51px content min-content size는 첫 sizing constraint `[9,19,19]`와 4px gutters에서 계산한다. |
| 12 | 최종 intrinsic pass에서 percentage resolution/free space를 생략하는가? | 51px definite size의 10%+2px minima와 max-content track space distribution을 별도 계산한다. |
| 13 | definite `40px`를 border-box width로 읽는가? | CSS width는 content box 40px이고 border 양쪽을 더한 root border box는 44px다. |
| 14 | row contribution을 columns보다 먼저 계산하는가? | Grid sizing의 columns-first 뒤 A line layout을 확정하고 rows contribution을 계산한다. |
| 15 | row heights가 line count 변화와 무관하게 고정되는가? | intrinsic A/B block contribution은 20/10px, definite는 30/10px이며 Rows는 `[9,9]`와 `[14,14]`로 달라진다. |
| 16 | root height 및 frame이 gutter/border를 빼먹는가? | intrinsic root는 55×24px, definite root는 44×34px로 content/gutter/border를 구분한다. |
| 17 | border 바깥 sample이 item을 실제로 가리키는가? | definite B는 root 오른쪽에서 12px 넘고 `(50,4)`는 B 내부, `(60,4)`는 두 item 외부다. |
| 18 | Chrome의 투명 페이지 배경을 native black clear와 같은 근거로 취급하는가? | 아니다. Chrome은 element probe만 기록하며 black surface clear는 모바일 runtime gate에만 둔다. |
| 19 | simulator evidence를 확보한 것으로 앞당겨 주장하는가? | 계획 상태는 runtime 미시작으로 고정하고 API 37 AVD/iOS Simulator는 구현 단계 gate로 둔다. |
| 20 | 네 scene을 C11.4 전체 완료나 78개 count 변경으로 확대하는가? | 공식 checkbox와 `77/213`을 유지하고 일반 CSS, WPT 및 parent C11.4는 남긴다. |

**결론:** 최초 초안의 wrapping 가정 오류를 runtime 진입 전에 제거하고, 독립 sizing 단계·Chrome 관측·모바일 표면 판정을 각각 분리했다. 이 표는 구현이나 PR diff 검토를 대신하지 않는다.
