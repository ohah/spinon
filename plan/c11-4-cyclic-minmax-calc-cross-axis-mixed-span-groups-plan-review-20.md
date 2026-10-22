# C11.4 mixed cross-axis span groups 계획 적대 검토 20개

대상은 세 item mixed-span 계획, Chrome 156 사전 관측과 CSS Grid 2025 CRD 독립 기대값이다. 구현 PR diff 검토는 포함하지 않는다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 기존 two-item cross-axis 장면을 이름만 바꿔 반복하는가? | 세 번째 span-3 item C와 2/2/3 span topology를 추가해 대상 차이를 고정했다. 통과. |
| 2 | span group 순서의 차이가 CSS 규칙에서 도출됐는가? | CSS Grid 2025 CRD §11.5는 점점 큰 span을 처리하고 §11.5.1은 per-track planned increase를 둔다. 통과. |
| 3 | item order reversal이 placement도 바꾸는가? | reverse scene은 A/B/C의 child 순서만 뒤집고 각 node의 placement/style은 유지한다. 통과. |
| 4 | span-2 A/B가 실제 가운데 column을 공유하는가? | A `1/3`, B `2/4`로 track 2를 공유한다. 통과. |
| 5 | C가 정확히 세 columns를 span하는가? | C `1/4`로 동일 3-track explicit grid의 전 columns를 span한다. 통과. |
| 6 | span-3 C의 min-content가 이미 계산된 bases를 다시 늘리는가? | C minimum 20px는 `[9,19,19]`와 4px gutters의 51px 합보다 작아 추가 growth가 없다. 통과. |
| 7 | A/B planned increases를 합산하는가? | A `[7,7,0]`, B `[0,17,17]`에서 per-track maximum `[7,17,17]`을 사용한다. 통과. |
| 8 | intrinsic bases와 최종 percentage-resolved tracks를 혼동하는가? | constraint bases `[9,19,19]`와 final `[15.671875,15.671875,15.65625]`를 별도 단계로 기록한다. 통과. |
| 9 | cyclic `calc()`에서 absolute 2px을 잃는가? | intrinsic 초기 track base에 2px 절대항을 유지하고 final percentage resolution을 별도로 계산한다. 통과. |
| 10 | gutter를 누락하거나 contribution에서 중복 차감하는가? | span-2 contribution에 2px, span-3 contribution에 4px gutter를 한 번씩 포함한다. 통과. |
| 11 | A/C의 min/max text widths가 고정된 font metrics와 맞는가? | 저장소 Ahem SHA-256, 400 weight, `10px/10px` line metrics를 지정하고 A `20/60px`, C `20/110px`을 Chrome 156과 대조한다. 통과. |
| 12 | 자동 minimum이 definite wrap 변화를 막는가? | A min-content 20px < 26px span, C min-content 20px < 40px span이다. Chrome에서 A 3줄/C 4줄을 직접 관측했다. 통과. |
| 13 | intrinsic C line threshold가 실제 51px span 안에 맞는가? | `AA AA AA AA`는 50px 두 덩어리로 두 줄이며 각 line rect가 50px이다. 통과. |
| 14 | row sizing 전에 Columns와 item wrap이 확정되는가? | Chrome output과 normative order 모두 Columns→item layout→Rows 순서다. 통과. |
| 15 | C의 row contribution이 definite Rows에 영향을 주는가? | C 40px contribution이 A 30px보다 커 Rows `[19,19]`를 결정한다. 통과. |
| 16 | text contribution과 stretched item frame을 혼동하는가? | definite C contribution 40px, frame 40px; A contribution 30px과 stretched frame 40px을 구별한다. 통과. |
| 17 | root box가 border/gutter를 포함해 계산되는가? | intrinsic 51px content + 4px border =55px; definite 40px + 4px=44px이며 heights도 독립 계산한다. 통과. |
| 18 | definite overflow 표본이 실제 B item을 가리키는가? | B x=16, width40으로 root right border 44px을 12px 넘는다. C는 x=2, width40이라 `(50,4)`를 덮지 않는다. 통과. |
| 19 | Chrome 관측을 규범 oracle로 삼거나 simulator/platform 결과로 확대하는가? | 독립 계산을 기준으로 두고 Chrome을 진단 관측으로만 둔다. runtime/플랫폼 gate는 구현 단계에 남긴다. 통과. |
| 20 | 네 fixed scene을 일반 CSS 지원 또는 C11.4 완료로 과장하는가? | Exact opt-in allowlist와 fail-closed 경계를 둔다. C11.4 checkbox·`77/213`·WPT 상태는 유지한다. 통과. |

**결론:** 처음 입력, track 단계, text wrapping, contribution과 frame 사이의 차이를 고정했다. 이 검토는 구현 성공이나 PR diff 검토를 대신하지 않는다.
