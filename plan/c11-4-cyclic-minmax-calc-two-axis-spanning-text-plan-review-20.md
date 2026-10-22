# C11.4 cyclic `minmax(calc())` 양축 span 계획 실패 관점 검토

| # | 공격 관점 | 판정과 계획 반영 |
|---:|---|---|
| 1 | `min-width:0`가 min-content contribution을 없애 Grid를 glyph보다 좁히는가? | 탐색 실행에서 root content width 2px와 10px text line overflow가 나왔다. 고정 input은 `min-width:auto`로 바꿨다. |
| 2 | `min-height:0`가 automatic minimum block contribution을 바꾸는가? | `min-height:auto`를 네 scene에서 고정하고 높이 제한은 `max-content`로 유지한다. |
| 3 | Ahem font가 준비되지 않아 text metric이 fallback 되는가? | Font bytes hash와 `document.fonts.check('10px Ahem')` 결과를 evidence에 둔다. |
| 4 | `word-break:break-all`의 min-content 값이 한 glyph 폭과 같은가? | Ahem 10px glyph 단위와 Chrome Range rect를 교차 확인하고 min-content width 10px로 독립 계산했다. |
| 5 | max-content text가 우연히 wrap되는가? | 60px inline size와 6×10px glyph advance에서 line count 1을 기대하고 관측했다. |
| 6 | column gap이 span contribution에서 빠지지 않거나 두 번 빠지는가? | 한 2px spanned gutter만 contribution에서 빼며 `[4,4]`/`[29,29]`가 gap 포함 root width를 재구성한다. |
| 7 | row gap이 block contribution에서 누락되는가? | Rows contribution도 2px gutter를 한 번 빼 `29+2+29=60`, `4+2+4=10`이 되게 한다. |
| 8 | column percentage minimum이 intrinsic basis에 순환 포함되는가? | §7.2.1대로 intrinsic calculation에서 auto처럼 처리한 뒤 3px/8px minimum으로 resolve한다. |
| 9 | row percentage minimum이 이미 계산된 root height를 잘못 참조하는가? | width10은 height60, width60은 height10인 per-axis basis를 JSON에 따로 고정한다. |
| 10 | row sizing이 column sizing 전에 실행되어 line count가 엇갈리는가? | `Columns → item inline available size → block contribution → Rows` trace 순서를 필수 판정에 넣었다. |
| 11 | definite width control이 intrinsic width oracle를 복제하는 데 그치는가? | 10px·60px definite controls는 cyclic minimum이 definite basis로 계산되는 독립 경로를 확인한다. |
| 12 | height가 definite여서 row track free-space 분배가 섞이는가? | 네 scene 모두 `height:max-content`이고 available block size를 definite로 만들지 않는다. |
| 13 | item이 두 축에서 실제 두 track씩 span하는가? | 정확한 `grid-column:1/3` 및 `grid-row:1/3`과 2×2 explicit tracks를 adapter에서 검증한다. |
| 14 | root border가 percentage basis와 reported frame을 혼동시키는가? | `%` basis는 10/60 content-box이고 root border-box는 각 방향 4px 추가임을 분리한다. |
| 15 | alignment나 auto placement가 기대 track size에 끼어드는가? | start alignment, explicit item placement, 단일 direct item을 고정한다. |
| 16 | item box 크기를 text line count로 오판하는가? | item frame, Range line rect, scroll size와 layout trace를 별도 필드로 검사한다. |
| 17 | Chrome screenshot을 Spinon text paint 증거로 확대하는가? | README와 PR 본문에 box screenshot만 표시하고 glyph pixel claim은 금지한다. |
| 18 | Simulator pixel 좌표가 DPR/native origin 차이를 무시하는가? | capture마다 DPR·viewport origin·scale transform을 기록하고 exact color sample만 판정한다. |
| 19 | iOS Metal drawable sample을 compositor 이후 결과라 주장하는가? | 계획은 readback을 compositor 제출 전으로 명시하고 최종 보고에도 한계를 남긴다. |
| 20 | fixture entry가 기본 product artifact에 포함되는가? | 기본 FFI artifact negative scan과 fixture-test positive control을 함께 요구한다. |

## 판정

서로 독립적인 입력·font·cycle·축 순서·trace·surface·artifact 실패 경로를 분리했다. 탐색 중 `min-width:0` 오류를 고쳐 최종 oracle input을 확정했다. Runtime 구현은 이 검토의 통과만으로 완료 처리하지 않는다.
