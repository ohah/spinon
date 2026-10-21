# C11.4 bordered Grid overflow 계획 PR 변경 실패 관점 검토 20개

이 검토는 plan PR에 실제로 포함된 roadmap 문구, Chrome fixture와 evidence 파일을 교차 확인한다. CSS 규범 자체의 독립 계획 검토와 구현 검토를 대체하지 않는다.

| # | 변경에서 확인한 실패 경로 | 결과 |
| --- | --- | --- |
| 1 | plan의 fixture 링크가 저장소 내 실제 경로를 가리키지 않음 | 경로가 존재하고 plan과 evidence README에서 같은 파일을 연결한다. 통과. |
| 2 | PR 본문/plan의 scope가 기존 C11.4 fixture와 다른 기능처럼 기술됨 | #428에서 빠졌던 border+overflow 조건을 두 text run/두 image로 제한했다. 통과. |
| 3 | status가 plan-only 후보를 구현 완료처럼 표시함 | plan 상태는 구현 미실행을 명시하고 STATUS는 unchecked/72 of 213으로 남긴다. 통과. |
| 4 | fixture title과 ID가 anonymous run 또는 border overflow를 오해시킴 | title과 두 scene ID 모두 anonymous text + image + border overflow를 지칭한다. 통과. |
| 5 | root CSS `box-sizing`과 width가 border/content 계산과 다름 | `border-box`, width 40px, border 2px씩으로 content width 36px임을 observation과 대조했다. 통과. |
| 6 | root height와 fixed rows가 맞지 않아 implicit row가 생김 | height 184px과 border 4px 합산 content height 180px, 3×60px rows가 일치한다. 통과. |
| 7 | Ahem 또는 PNG 상대 URL이 fixture 위치에서 resolve되지 않음 | asset 경로 두 개가 `tests/fixtures/resources`의 실제 font/PNG를 resolve하며 decode 완료를 기록한다. 통과. |
| 8 | scene child order가 계획 표와 HTML markup 간 달라짐 | forward/reversed 두 DOM sequence와 image IDs를 observation JSON에서 확인했다. 통과. |
| 9 | image heights 또는 span placement가 scene별로 바뀜 | 두 PNG는 16px/32px height와 같은 고정 column/row span을 쓴다. 통과. |
| 10 | Chrome observation JSON에 규범 기대값을 섞어 actual 값으로 위장함 | JSON은 UA, computed styles, frames, Range fragments만 저장하고 독립 계산은 README/plan에 분리한다. 통과. |
| 11 | text `Range` fragment가 anonymous item frame으로 불림 | JSON key는 text fragment를 명시하고 plan은 item frame 대체가 아니라고 경고한다. 통과. |
| 12 | overflow `22px`가 content edge를 기준으로 계산됨 | `imageOverflowRight`는 image border-box 끝과 Grid border-box 끝의 차이다. 두 scene 값은 22다. 통과. |
| 13 | reverse scene의 second image가 JSON 배열에서 index 1이라고 가정함 | README 결과는 scene ID와 image ID를 보고 geometry를 해석하며 child index에 의존하지 않는다. 통과. |
| 14 | Chrome full version 또는 viewport/DPR provenance가 빠짐 | Chrome `155.0.8059.85`, Agent Browser, macOS ARM64, 1280×700, DPR 1이 JSON/README에 있다. 통과. |
| 15 | screenshot이 CSS border 또는 실제 border 밖 image 영역을 보이지 않음 | 1280×700 PNG에서 gold border와 patterned image가 오른쪽으로 이어지는 두 scene을 확인했다. 통과. |
| 16 | screenshot의 border/image pixel sample 값이 예상과 다름 | DPR 1 pixel readback은 border `#f0c674`, border 밖 PNG quadrant `#00ff40`, image 뒤 바탕 white다. 통과. |
| 17 | sample point가 image border 안에 있거나 source texel 위치가 불명확함 | Grid-local `(51,71)`은 border-box edge x=40 밖이며 image B의 scaled source pixel `(18,4)` 중심이다. 통과. |
| 18 | README SHA-256이 현재 fixture/JSON/PNG/font/asset과 다름 | 다섯 값이 `shasum -a 256` 및 SHA256SUMS와 일치한다. 통과. |
| 19 | roadmap 설명이 검증 전 결과를 확정하거나 완료 수를 올림 | STATUS는 next-plan 후보로만 기록하고 count를 72/213으로 고정한다. 통과. |
| 20 | plan PR을 Android/iOS 앱 또는 실기기 테스트 통과로 보고함 | 계획 PR은 browser precomparison만 포함하며 simulator app 검증은 후속 구현 gate로 남긴다. 통과. |

최종 검토에서 plan PR diff를 막아야 할 문서·fixture·근거 불일치는 발견하지 못했다. 구현 PR은 코드와 Android/iOS Simulator 결과를 대상으로 별도 20개 관점 검토가 필요하다.
