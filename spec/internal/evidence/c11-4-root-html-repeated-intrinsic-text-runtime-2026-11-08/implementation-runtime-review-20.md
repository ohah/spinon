# C11.4 반복 intrinsic text implementation/runtime 20관점 검토

**범위:** C11.4 반복 text contribution 후보의 code path, 270-scene RuntimeSession matrix, iOS 실행과 Android AVD 실패 경계. 계획 검토와 별개의 구현 검토다. **결과:** 코드·Rust runtime 20/20 확인; Android platform runtime은 QEMU 실행환경 문제로 미검증.

| # | 독립 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | track-list 길이·member multiset이 넓게 허용되는가 | 6개 member, 첫 `fit-content()` 및 후행 `[min-content×2,max-content×2,auto]`만 받는다. 30개 고유 순서가 270-scene matrix에서 모두 확인됐다. |
| 2 | cap의 계산되지 않은 값이 통과하는가 | style projection의 typed 값과 Layout 모두 finite non-negative px/direct percentage만 받고, percentage는 definite 100px 또는 220px basis에서만 비교한다. |
| 3 | 다른 Grid topology가 profile로 오인되는가 | 40px 단일 column, 10px explicit row, row flow, 빈 line names, 무-gap/zero edges, visible overflow와 static positioning으로 제한한다. |
| 4 | unsupported alignment나 stretch 분배가 섞이는가 | `start|stretch`만 받는다. 270 scene은 두 alignment를 모두 비교하며 stretch에서 positive free space가 auto row에만 배분되는 expected rows를 확인한다. |
| 5 | item count·순서·owner가 잘못되어도 인식되는가 | 한 visible explicit item 뒤에 payload 8개인 9-item order를 검사한다. 추가/누락/잘못된 item은 style/layout admission에서 거부한다. |
| 6 | explicit item의 크기나 자식이 달라지는가 | 첫 item은 고정 10px이고 자식이 없어야 한다. 같은-session recovery와 focused Root HTML 회귀가 이 경계를 포함한다. |
| 7 | `min-height:0` 예외가 다른 profile로 새는가 | 첫 item 예외는 repeated-text profile에서만 허용한다. 비반복 4-member test에서 높이 변형의 오류 property 계약이 깨진 것을 발견해 진단을 기존 `grid-auto-rows`로 되돌렸고 회귀가 통과했다. |
| 8 | wrapper나 payload 자손 구조가 늘어나는가 | 각 implicit item에 visible payload div 하나, payload에 직접 text node 하나만 허용한다. 중첩 inline mutation은 원자 거부된다. |
| 9 | text source가 다른 문장·여러 source로 바뀌는가 | payload 여덟 문장을 node order대로 exact match하며 source 하나, inline ancestor/run/style 없음이 projection과 Layout 모두에서 검사된다. |
| 10 | font/line metrics 변형이 같은 profile로 받아들여지는가 | monospace 10px, weight 400, normal style, line-height 10px를 exact match하고 RuntimeSession에서 glyph 수·baseline line 수를 검사한다. |
| 11 | whitespace·방향·spacing·writing mode가 달라지는가 | wrapping/word-break/transform/spacing/indent/direction/writing-mode/text-orientation typed defaults를 제한한다. nowrap 및 font-family mutation은 실패한다. |
| 12 | text contribution 높이가 결과별 상수에 고정되는가 | 반복 text path는 `TextParagraphInput`의 실제 paragraph와 glyph layout을 사용한다. 270 scene에서 payload frames, glyph 수와 baseline 수를 Chrome expected와 비교한다. |
| 13 | cap 경계와 zero-size row가 누락되는가 | `20px`, `20%`, `0%`, `120%`를 모두 비교한다. zero-height item frame은 출력에 남고 payload는 paint snapshot에 유지된다. |
| 14 | visible overflow가 Grid/root frame에 clamp되는가 | 100px Grid의 30-scene cohort에서 모든 item/payload frames를 Chrome JSON과 비교한다. Grid 아래로 나간 payload frame도 보존된다. |
| 15 | `align-content:start`의 남는 공간이 row size에 섞이는가 | 220px cohort의 start scene에서 남는 높이가 row에 분배되지 않고 track/item/frame이 일치한다. |
| 16 | fractional frame 경계가 browser oracle과 어긋나는가 | repeated-text profile만 item 경계를 1/64 CSS px로 정규화하며 다음 sibling top을 사용해 누적 float drift를 피한다. 270 scene 각 component가 0.01px 이내다. |
| 17 | 일반 layout마다 정규화 비용이나 변경이 적용되는가 | track profile이 없으면 node map/hidden subtree 생성 전에 early return한다. 다른 Root HTML fit-content 23개 및 Layout 14개 regression이 통과한다. |
| 18 | style revision과 frame/render output이 서로 다른 generation인가 | 270 scene test에서 completed key와 cascade key가 같고 typed track order/serialization, item paint frames, text frames가 같은 결과를 낸다. |
| 19 | invalid profile이 partial layout/render output을 남기는가 | 잘못된 text, font, line-height, nowrap, inline descendant, fixed text height와 item min-height 변형이 node/property 진단과 함께 실패하고 `completed`가 비어 있다. |
| 20 | 거부 뒤 같은 RuntimeSession이 복구되지 않는가 | dark-mode style mutation에서 실패한 다음 light mode로 바꾸면 같은 session의 다음 revision이 Ready가 된다. |

Rust 실행 요약은 [실행 근거](./README.md)에 있다. iOS Simulator는 20 boxes의 layout-ready와 화면 smoke를 기록했다. Android build는 성공했지만 QEMU AVD 부팅이 실패해 Android runtime 결과는 이 검토의 통과로 세지 않았다.
