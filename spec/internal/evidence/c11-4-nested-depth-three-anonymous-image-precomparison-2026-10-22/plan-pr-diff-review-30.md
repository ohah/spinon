# C11.4 depth-three anonymous text + image 계획 PR diff 적대 검토 30개

이 검토는 계획·precomparison diff만 독립적으로 대상으로 삼는다. 구현/runtime 실패 관점 20개와 계획 문서 자체의 20개 review에는 포함하지 않는다.

| # | 검토한 실패 경로·불변 조건 | 결과 |
|---:|---|---|
| 1 | 계획의 상위 C11.4, depth-three 및 내부 0059 링크가 현재 문서 ID를 가리키는가 | 통과: 저장소의 기존 파일과 anchor를 확인했다. |
| 2 | 공식 status에 계획 단계를 구현 완료처럼 표시하는가 | 통과: 구현 미시작이라고 명시하고 C11.4와 `72/213`을 유지했다. |
| 3 | 부모 계획의 새 항목과 집중 계획의 상태가 모순되는가 | 통과: 둘 다 2026-10-22 계획/Chrome 비교 완료, 구현 미시작으로 맞췄다. |
| 4 | 내부 index가 이 계획을 PR #453의 병합 결과처럼 잘못 붙이는가 | 통과: `planned` heading으로 분리했다. |
| 5 | scene이 두 축에서 같은 intrinsic keyword를 사용한다고 명확한가 | 통과: width/height와 columns/rows 모두 mode별로 고정했다. |
| 6 | chain 깊이가 depth-three와 네 Grid 노드 중 무엇인지 모호한가 | 통과: `root → middle → branch → inner`, 세 경계를 네 노드로 적었다. |
| 7 | inner item 개수 또는 anonymous source가 불분명한가 | 통과: direct text sequence 하나와 HTML image 하나로 제한했다. |
| 8 | text run의 min-content 계산에서 공백·긴 단어를 잘못 합산하는가 | 통과: 가장 긴 `XXXXXXX`는 70px, whole run은 100px로 분리했다. |
| 9 | 10px font와 10px line-height 혼동으로 20px min-content height를 잘못 정하는가 | 통과: min width에서 두 line fragment와 line-height를 별도로 기록했다. |
| 10 | image natural dimension이 encoded asset 또는 CSS author size와 다를 수 있는가 | 통과: 고정 asset·decode·24×16 natural-size 확인을 요구한다. |
| 11 | image를 CSS blockification 전에 inline width로 계산하는가 | 통과: §6.1 근거와 element/item frame 확인을 포함한다. |
| 12 | auto placement 순서를 기대값에 반대로 적용하는가 | 통과: source order별 `[text,image]` / `[image,text]` track 및 x 위치를 명시했다. |
| 13 | source-order reversal이 parent aggregate size도 바꾼다고 오인하는가 | 통과: aggregate frame은 같고 inner track/order만 바뀐다고 고정했다. |
| 14 | two columns 대신 implicit track을 하나 더 만들 수 있는가 | 통과: computed track count 검사와 정확한 두 direct item을 계획에 포함했다. |
| 15 | min-content row height가 text 20px과 image 16px 중 작은 값으로 결정되는가 | 통과: track sizing의 item max를 적용해 20px로 유도했다. |
| 16 | max-content row에서 text 10px보다 image 16px가 높아야 함을 놓치는가 | 통과: row 16px와 image frame 16px를 독립적으로 적었다. |
| 17 | grid total width에 gap/box edge가 누락됐는가 | 통과: 모든 margin/padding/border/gap을 0px로 고정했다. |
| 18 | 상위 Grid가 inner contribution을 그대로 받지 않고 별도 width가 될 수 있는가 | 통과: 각 parent의 one-child contribution, track, frame을 모두 검사한다. |
| 19 | screenshot/page y offset이 oracle에 들어가는가 | 통과: frame은 scene root-relative이고 page placement는 판정하지 않는다. |
| 20 | Chrome observation이 독립 기대값을 덮어쓰는가 | 통과: expected JSON과 observation을 별도 산출물로 유지한다. |
| 21 | Chrome/Ahem 준비 상태 미확인으로 값이 fallback font에서 나오는가 | 통과: UA/version, `document.fonts.check`, resource hash를 보관했다. |
| 22 | PNG load 실패를 min-content 값으로 오인할 수 있는가 | 통과: decode와 exact natural size 모두 선행 조건이다. |
| 23 | CSS Grid source order 조항을 잘못 인용했는가 | 통과: 공식 2025 CRD §8.5 auto-placement 링크를 직접 Agent Browser에서 확인했다. |
| 24 | Grid anonymous item 조항과 blockification을 같은 규칙으로 혼합하는가 | 통과: §6 item formation과 §6.1 display를 분리 인용했다. |
| 25 | intrinsic track sizing을 spanning-item 알고리즘 하나로 과도하게 단순화하는가 | 통과: 무-span 입력과 §11.5 content-based track procedure를 명시했다. |
| 26 | 허용 오차가 CSS px 기준인지 기기 pixel 기준인지 혼동되는가 | 통과: 1/64 CSS px로 지정하고 device scale factor는 별도 기록했다. |
| 27 | 계획 단계에서 일반 지원 범위를 열어 놓는가 | 통과: 네 IDs 외 depth/topology 입력은 거부 조건으로 남겼다. |
| 28 | fixture-only 코드가 product archive에 들어갈 위험을 계획이 놓치는가 | 통과: default/fixture artifact positive·negative control을 요구했다. |
| 29 | Simulator frame/draw 증거를 paint equality 또는 실기기 결과로 확대하는가 | 통과: 실기기·glyph/image pixel·iOS post-compositor·WPT 한계를 각각 분리했다. |
| 30 | PR 본문 screenshot 또는 Agent Browser 종료가 누락되는가 | 통과: 화면 링크와 browser session/process 정리를 계획 단계별로 명시했다. |

판정: 계획 diff에 구현 승인으로 오해될 상태, 계산 누락 또는 검증 과장이 발견되지 않았다. 구현은 이 계획 PR 병합 뒤 새 검토 20개와 새 PR diff 검토 30개를 별도로 수행한다.
