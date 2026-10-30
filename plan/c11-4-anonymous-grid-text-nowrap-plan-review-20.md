# C11.4 anonymous Grid text `white-space:nowrap` 계획 실패 관점 검토 20개

검토 대상은 [집중 계획](./c11-4-anonymous-grid-text-nowrap.md), [Agent Browser HeadlessChrome 156 사전 비교](../spec/internal/evidence/c11-4-anonymous-text-nowrap-precomparison-2026-10-30/README.md), CSS Grid Level 1 §6·§11.5.1과 CSS Text Level 3 §3이다. 계획 검토이며 구현 성공이나 제품 지원을 뜻하지 않는다.

| # | 실패 관점 | 판정 및 계획 조치 |
|---:|---|---|
| 1 | `white-space:nowrap`이 Grid item에 상속되지 않는가 | CSS Text §3은 inherited property로 정의한다. 계획은 Grid root에 적용된 computed `text-wrap-mode`와 fixture `ParagraphStyle` 일치를 확인한다. |
| 2 | 직접 text가 anonymous Grid item이 아니라 inline content로 남는가 | CSS Grid §6의 anonymous block item 규칙을 근거로 삼고, 기존 direct-text route를 유지한다. |
| 3 | 두 auto-repeat 문제가 valid feature로 다시 제안되는가 | §7.2.3.1은 auto-repeat을 track list에 최대 한 번 허용한다. 계획과 stage C를 이를 구현 gap으로 세지 않도록 수정했다. |
| 4 | 이번 slice가 `white-space:pre`나 `pre-line`까지 허용하는가 | `Normal`과 `NoWrap`만 allowlist에 두고 `PreLine` negative control을 유지한다. |
| 5 | `nowrap`이 space를 보존해 width 계산이 달라지는가 | CSS Text §3은 nowrap이 normal처럼 whitespace를 collapse한다고 정의한다. input은 한 내부 space를 사용한다. |
| 6 | text sample에 허용되지 않은 soft wrap 기회가 남는가 | `XX XX` Ahem sample의 only break opportunity는 한 space이며 nowrap은 이를 금지한다. |
| 7 | Ahem glyph advance 가정이 틀렸는가 | Chrome 관측은 `10px/10px` Ahem readiness를 true로 확인하고 5 cells × 10px = 50px를 관찰했다. asset SHA를 보존한다. |
| 8 | normal control의 min-content가 nowrap 값과 잘못 합쳐지는가 | 같은 문구의 normal control에서 20px track 및 두 20×10px fragment를 고정했다. |
| 9 | root의 available width가 min-content sizing 결과를 가리는가 | 두 scene 모두 root width 20px·min-width 0으로 고정하고 track `20px`/`50px`를 별도로 관찰한다. |
| 10 | nowrap overflow를 track width와 root width 혼동으로 판단하는가 | nowrap은 root width 20px, track 50px이다. 30px overflow를 각각의 값으로 분리한다. |
| 11 | row height가 text line count를 검증하지 않는가 | normal row 20px, nowrap row 10px로 관찰됐고 Range fragment 개수와 line-height도 기록한다. |
| 12 | browser page position이 app-local geometry로 오인되는가 | 관측 JSON에는 원 좌표를 남기고 acceptance는 grid-local dimensions·track·fragment relative offset만 사용한다. |
| 13 | Range rect가 anonymous item box 또는 glyph ink로 오인되는가 | 계획과 evidence에서 Range를 text fragment observation으로만 취급하고 item/frame 및 surface proof를 분리한다. |
| 14 | CSS shorthand와 구현 longhand가 다른 값을 만드는가 | Browser oracle의 shorthand와 runtime fixture의 `white-space-collapse`/`text-wrap-mode`를 별도 확인 항목으로 둔다. |
| 15 | fixture가 일반 CSS cascade 지원을 과장하는가 | `ParagraphStyle`은 현재 fixture architecture의 고정 입력이며 general style-to-text mapping을 범위 밖이라고 명시한다. |
| 16 | anonymous run grouping 또는 source provenance가 변하는가 | text grouping, anchor, source-node list, owner/order/revision 확인을 기존 경로와 함께 유지한다. |
| 17 | `order != 0`의 anonymous item 순서가 무검증으로 열리는가 | 계획은 nonzero order negative control 및 partial output 부재를 요구한다. |
| 18 | FFI fixture가 기본 제품 artifact로 새어 나가는가 | 새 scene id와 payload가 fixture-test opt-in artifact에만 있는지 default/positive control을 확인한다. |
| 19 | simulator frame 결과를 WPT나 하드웨어 렌더 proof로 넓히는가 | Android AVD/iOS Simulator의 제한된 runtime 범위와 하드웨어·WPT·iOS compositor 이후 미검증을 분리한다. |
| 20 | 작은 slice 때문에 상위 C11.4 완료 수가 바뀌는가 | 계획은 C11/C11.4 checkbox와 공식 `72/213`을 고정적으로 미완료 유지한다. |

**결론:** 구현 전 관문으로 차단 사항은 없다. 실제 구현 전에는 Chromium observation/hash, independent expected values, exact scene/style gate, negative controls 및 artifact boundary를 다시 대조한다.
