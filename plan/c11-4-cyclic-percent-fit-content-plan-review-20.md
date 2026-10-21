# C11.4 cyclic percentage `fit-content()` 계획 검토 20개

검토 대상: [cyclic percentage `fit-content()` 계획](./c11-4-cyclic-percent-fit-content.md), Chrome 156 Ahem 사전 비교 fixture/JSON, CSS Grid 2025 CRD sizing rule에서 독립 계산한 expected tracks와 frames.

| # | 독립 실패 관점 | 차단 조건과 계획의 대응 |
| ---: | --- | --- |
| 1 | definite percentage case를 cyclic sizing case로 잘못 분류 | column·row 장면은 `max-content` 축, control만 definite width로 구분한다. |
| 2 | container intrinsic size 계산 때 `50%`를 먼저 풀어 self-dependent basis를 축소 | intrinsic container pass와 final track sizing pass를 분리해 각각의 percentage 의미를 적었다. |
| 3 | `fit-content()`를 fixed max track처럼 취급해 auto minimum을 잘라냄 | limit뿐 아니라 auto minimum floor와 max-content ceiling을 함께 계산한다. |
| 4 | cyclic percentage 처리로 `fit-content()` 전체를 `auto`로 바꿈 | intrinsic pass의 cyclic percentage operand만 auto 취급하고 final fit-content sizing에서 percentage limit을 다시 푼다. |
| 5 | column의 min-content contribution을 max-content 크기로 착각 | Ahem `aa aa`의 min-content `20px`, max-content `50px`와 final limit `25px`을 구분한다. |
| 6 | row의 inline wrapping이 불명확해 40px intrinsic height가 달라짐 | width `20px`, 10px Ahem advance/line-height, 네 단어의 line formation을 입력으로 고정한다. |
| 7 | `min-width:auto`/`min-height:auto` floor가 fit-content limit을 초과해 cap을 가림 | item에 두 축 `min-size:0`을 지정하고 해당 값이 expected minimum에 필요한 이유를 설명한다. |
| 8 | Ahem font load 실패 뒤 fallback sans geometry로 oracle 결과가 바뀜 | observation에 font loaded/status를 기록하고, 미로드면 oracle run을 무효로 한다. |
| 9 | CSS fixture와 runtime text harness의 font/style subset이 달라짐 | runtime acceptance는 명시 Ahem text style·content를 고정하고 projection 전에 snapshot 값을 검사한다. |
| 10 | `line-height`가 normal로 계산돼 root/item height가 달라짐 | Ahem `font-size`와 `line-height`를 모두 `10px`로 고정한다. |
| 11 | UA margin·body margin이 root size나 pixel 위치에 섞임 | HTML/body와 모든 fixture box의 margin·padding·border를 0으로 명시한다. |
| 12 | box sizing/padding/border가 50% basis를 content box와 다르게 만듦 | padding·border를 0으로 고정하고 명목 content width와 containing box를 일치시킨다. |
| 13 | 두 번째 implicit track 또는 자동 placement가 한 track 결과에 섞임 | root당 item 하나와 explicit track 하나만 허용하고 trace의 track 수를 확인한다. |
| 14 | definite control 결과를 cyclic 기대값으로 재사용 | control은 width `40px`의 definite basis와 `20px` track을 독립 항목으로 둔다. |
| 15 | computed track list만 맞고 root/item frame은 잘못됨 | 각 장면에서 used tracks와 root/item frame을 함께 비교한다. |
| 16 | root들의 누적 page y 위치를 intrinsic size 비교로 혼동 | page absolute y는 제외하고 root 기준 크기와 item-relative origin을 검사한다. |
| 17 | row에서 item box와 넘친 text line/glyph를 같은 paint 증거로 봄 | row item frame `20px`과 root `40px`을 분리하고 glyph paint equality를 acceptance에서 제외한다. |
| 18 | scene 색이나 표본 좌표가 track 안/밖 경계를 잘못 설명 | item/background의 box 내부·root 표본만 명시하고 CSS border stroke는 주장하지 않는다. |
| 19 | 문서 뒤쪽 변경 이력의 UA 조정 설명을 현재 §7.2.1 규범 재량으로 잘못 인용 | 현재 §7.2.1과 §11.1의 sizing 규칙을 기준으로 기대값을 계산하고 변경 이력 문구는 근거에서 제외한다. |
| 20 | 세 고정 scene을 일반 `fit-content()` 또는 C11.4 완료로 홍보 | 계획·PR·roadmap에 opt-in fixture 경계와 C11.4 `72/213` 미승격을 유지한다. |

판정: Chrome 156 Ahem oracle은 font load를 확인했고, intrinsic basis가 final 50% limit보다 커서 `fit-content()`의 clamp를 실제로 드러낸다. 이전 fixed 24px item 입력은 limit을 구분하지 못해 acceptance에서 제외했다. 계획은 구현 PR 전의 3-scene opt-in 범위, 현재 규범과 변경 이력의 경계, definite control, failure closure를 분리했으며 구현을 시작할 수 있다. 이 계획 표는 향후 구현/runtime 20개 검토나 PR diff 30개 검토로 재사용하지 않는다.
