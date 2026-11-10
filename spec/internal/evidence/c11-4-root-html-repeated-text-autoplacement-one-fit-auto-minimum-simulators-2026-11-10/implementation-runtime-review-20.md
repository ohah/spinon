# C11.4 one-fit auto-placement 자동 최소 크기 구현/runtime 검토 · 20개 관점

범위는 Style projection과 Layout의 one-fit repeated-text `min-height:auto` admission, RuntimeSession oracle 행렬·대조군, native simulator smoke다. 계획 검토 및 PR diff 검토와 별도로 수행했다.

| # | 적대적 실패 관점 | 근거와 판정 |
|---:|---|---|
| 1 | 일반 Grid가 자식 text를 포함했다는 이유만으로 `min-height:auto`를 허용하는가? | **통과.** 제한된 여섯-member auto-placement list, 정확한 9-child topology, repeated-text profile도 일치해야 한다. 나머지 mutation은 거부된다. |
| 2 | Style projection은 허용하지만 Layout이 더 넓은 규칙으로 처리하는가? | **통과.** 두 gate 모두 repeated-text automatic-minimum profile을 요구한다. RuntimeSession 행렬 1,440개가 cascade부터 layout까지 검사한다. |
| 3 | explicit-row control이 automatic-minimum item으로 바뀌는가? | **통과.** 첫 item은 computed `min-height:0`, 높이 10px, frame `[0,0,40,10]`을 유지한다. control을 `auto`로 바꾸면 거부된다. |
| 4 | 일부 item이 auto, 일부가 zero인 혼합 minimum도 profile로 통과하는가? | **통과.** 여덟 text item 모두 `auto`여야 한다. 혼합 `auto`/`0`과 고정 nonzero minimum은 거부 사례다. |
| 5 | fit-content track 개수나 tail multiplicity를 느슨하게 인정하는가? | **통과.** one-fit list는 fit 1개, `min-content` 2개, `max-content` 2개, `auto` 1개만 받는다. fit 0/3개와 잘못된 multiplicity가 거부된다. 기존 two-fit profile은 별도로 검사했다. |
| 6 | explicit row 뒤 cycle index가 잘못 적용되어 일부 fit 위치만 우연히 통과하는가? | **통과.** 후보 key는 authored 위치 여섯 개와 반복 row cycle을 포함한다. 각 fit-row item과 text floor를 검사한다. |
| 7 | `0%`, `20%`, `120%`, `20px` cap이 동일한 type이나 basis로 정규화되는가? | **통과.** Typed track-list 비교가 length와 percentage 및 authored order를 검사한다. Grid block axis는 확정된 220px이고, 미등록 7px 및 13% 대조군도 실제 content floor로 검사한다. |
| 8 | 작은 cap이 fit row를 text min-content보다 줄이는가? | **통과.** zero-cap trace는 automatic·content-based minimum을 기록한다. 1,440 scene에서 baseline 중 floor보다 작았던 600개 row frame이 후보에서 회복된다. |
| 9 | text payload나 glyph wrapping 대신 frame만 검사하는가? | **통과.** 8 payload frame, glyph 수, 줄 수를 Chrome 관측과 비교하고 9 item frame도 검사한다. |
| 10 | `align-content:stretch`가 fit-content 또는 intrinsic row를 늘리는가? | **통과.** start/stretch 쌍의 총 row 합은 각각 180px/220px다. Authored `auto` row만 stretch를 받는다. |
| 11 | content floor 수정으로 definite Grid 밖에 visible overflow가 생기는가? | **통과.** 후보 scene마다 text payload의 bottom을 검사하며 1,440 scene에서 overflow는 0개다. |
| 12 | scrollable 또는 hidden item overflow를 content-based automatic minimum으로 잘못 취급하는가? | **통과.** 새 automatic-minimum 경로는 visible overflow를 요구한다. item의 `auto`, `scroll`, `hidden` mutation과 Grid scrollable overflow가 거부된다. |
| 13 | percentage cap을 definite block-size basis 없이 resolve하는가? | **통과.** 검사 Grid의 height는 220px다. 이를 `auto`로 바꾸면 layout publish 전에 거부된다. |
| 14 | 비슷해 보이지만 track geometry가 다른 container도 admission되는가? | **통과.** 40px column, 10px explicit row, 40×220px owner, 0 gap/edge, static position, row flow, LTR로 제한한다. width, flow, direction, alignment mutation은 거부된다. |
| 15 | 명시적 item placement가 auto-placement row-cycle 가정을 우회하는가? | **통과.** 여덟 content item 모두 자동 배치여야 한다. explicit row mutation은 거부된다. mixed row-span/auto-placement 거부는 별도로 실행한 two-fit suite에 남아 있다. |
| 16 | child 누락·추가 또는 nested payload로 auto-placement item owner가 달라지는가? | **통과.** 아홉 번째 Grid child 누락과 nested payload descendant가 거부된다. 허용 입력은 각 text item의 payload div 하나를 유지한다. |
| 17 | Chrome floor 비교 중 font metrics가 바뀌는가? | **통과.** 고정 monospace/10px/10px text profile을 검증하고 font-size mutation은 거부한다. |
| 18 | trace 누락 또는 stale trace여도 그럴듯한 final layout을 publish하는가? | **통과.** Feature-gated trace는 automatic minimum을 동일 owner와 최종 row sizing 결과에 연결한다. focused trace tests가 통과하며 행렬은 typed style과 publish frame revision도 검사한다. |
| 19 | 거부된 dark 입력이 stale frame을 게시하거나 다음 revision을 오염시키는가? | **통과.** 같은 세션의 dark three-fit 입력은 completed output 없이 실패한다. 유효 light profile로 돌아오면 새 revision에서 모든 frame을 복원한다. |
| 20 | simulator screenshot으로 모든 track 위치 또는 플랫폼 pixel parity를 주장하는가? | **통과.** iOS·Android light/dark route 로그는 `layout=ready`와 draw/present를 보여준다. Screenshot은 position 4 대표 장면이며 위치 1–6은 RuntimeSession이 다룬다. Android는 software llvmpipe이고 compositor 이후 pixel parity를 주장하지 않는다. Capture 뒤 두 simulator와 앱 process를 정지했다. |

**결과:** 서로 다른 구현/runtime 실패 관점 20개를 검토했다. 근거는 고정 profile에 한정되며 C11.4/C11 완료나 공식 `78/214` 증가를 뜻하지 않는다.
