# C11.4 cyclic percentage `minmax(calc())` 계획 검토 20개

검토 대상: [고정 구현 계획](./c11-4-cyclic-minmax-calc.md), HTML fixture, 독립 계산 JSON, Chrome 156 Agent Browser observation, CSS Grid Layout Module Level 1 2025 CRD §7.2.1.

| # | 독립 실패 관점 | 계획의 차단 조건과 판정 |
| ---: | --- | --- |
| 1 | intrinsic sizing에서 calc percentage를 먼저 풀어 self-dependent size를 줄이는가? | intrinsic root와 final track pass를 분리하고 cyclic percentage를 먼저 auto처럼 취급한다. 통과. |
| 2 | `minmax()` maximum calc가 bare `calc()`나 `fit-content()`와 같은 함수로 오해되는가? | `GridTrackMaxSizingFunction::Calc` 및 `minmax(0px, …)`의 max bound를 직접 검사한다. 통과. |
| 3 | 0px minimum 대신 자동 최소를 사용해 expected max를 가리는가? | 네 scene 모두 track minimum은 literal `0px`; item min-size 0도 별도 고정한다. 통과. |
| 4 | calc absolute length가 intrinsic pass에서 사라지는가? | 최종 maximum 식에 `+4px` 또는 `+2px`를 보존하고 definite controls에서 각각 재계산한다. 통과. |
| 5 | column percentage basis에 block size 또는 row basis를 잘못 쓰는가? | column scene은 intrinsic/definite inline basis로, row scene은 intrinsic block basis 40px로 분리한다. 통과. |
| 6 | `aa aa` min-content와 max-content를 같은 값으로 계산하는가? | 고정 Ahem 10px에서 20px/50px를 독립 입력으로 명시하고 29px cap이 그 사이인지 확인한다. 통과. |
| 7 | row text가 네 줄이 되지 않아 40px basis가 바뀌는가? | 20px inline size와 `10px` line-height, `a a a a` 입력을 함께 고정하고 Chrome observation과 비교한다. 통과. |
| 8 | font load 실패 뒤 fallback font가 oracle을 통과시키는가? | observation은 `document.fonts.ready` 후 `check('10px Ahem')`가 true인 run만 인정한다. 통과. |
| 9 | definite control을 cyclic case처럼 평가하는가? | 40px width의 24px result를 cyclic column 29px와 분리해 기대값으로 둔다. 통과. |
| 10 | 10px small control의 min-size 때문에 9px track을 못 만드는가? | item `min-width:0`와 `min-height:0`을 고정하고 item width 9px을 track과 함께 비교한다. 통과. |
| 11 | intrinsic root 크기가 최종 max bound와 혼동되는가? | root 50/40px과 final tracks 29/22px을 독립 필드로 기록한다. 통과. |
| 12 | definite small percentage basis가 10px가 아니라 scroll/content extent를 참조하는가? | containing root width 10px을 basis로 사용하고 resulting scroll width 20px은 별도 관찰로만 기록한다. 통과. |
| 13 | row gap이나 padding/border가 track contribution에 중복되는가? | 모든 gap·padding·border를 0으로 명시한다. 통과. |
| 14 | computed track만 일치하고 item frame/stretch는 다를 수 있는가? | 각 scene의 used columns/rows와 root/item frame을 함께 acceptance한다. 통과. |
| 15 | Chrome observation을 규범 기준으로 잘못 취급하는가? | 독립 기여값과 CRD 단계로 기대를 계산하고 Chrome은 별도 관찰자로 보관한다. 통과. |
| 16 | overflow sample이 item/root 내부 좌표에 찍혀 외부 overflow를 증명하지 못하는가? | root-relative `(15,5)`는 10px root 및 9px item 밖이고 `(25,5)`는 text 밖인 control이다. 통과. |
| 17 | overflow 색 결과를 border stroke 검증이라고 과장하는가? | 계획·PR에서 해당 표본을 text paint overflow로 한정하고 border stroke를 명시적으로 제외한다. 통과. |
| 18 | fixture scene allowlist 밖에서 일반 CSS admission을 의도치 않게 넓히는가? | opt-in fixture feature의 정확한 ID/DOM/style만 허용하고 unsupported input은 fail-closed로 유지한다. 통과. |
| 19 | 기본 제품 artifact에 고정 scene이나 Ahem fixture payload가 들어가는가? | 기본 archive negative와 fixture-test positive control을 각각 검사한다. 통과. |
| 20 | 한 플랫폼 증거 또는 Chrome 결과만으로 전체 CSS Grid/C11.4 완료를 주장하는가? | Android AVD와 iOS Simulator를 따로 확인하며 WPT·실기기·compositor-after·일반 CSS 및 C11.4 집계를 범위 밖으로 둔다. 통과. |

판정: 네 scene은 cyclic intrinsic basis, final `minmax()` maximum, definite controls 및 root 바깥 text overflow를 분리한다. Chrome 관측은 독립 기대값과 일치하고 font readiness가 기록됐다. 구현을 시작할 수 있다. 이 계획 검토는 구현/runtime 20개와 PR diff 30개 검토에 재사용하지 않는다.
