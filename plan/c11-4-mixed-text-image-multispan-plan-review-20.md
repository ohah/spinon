# C11.4 mixed text/image multi-span 계획 적대 검토 20개

| # | 실패 관점 | 공격 검토와 판정 |
| ---: | --- | --- |
| 1 | 오라클 사전성 | 예상 열 값을 Chrome 관찰에서 복사하면 규범 비교가 순환한다. README에서 §11.5.1 planned increase로 독립 계산을 먼저 고정해 수용했다. |
| 2 | font 준비 경합 | Ahem load 전 fallback font로 측정하면 50/180px 기대가 무효다. fixture가 `document.fonts.ready`와 `document.fonts.check`를 gate한다. |
| 3 | text min-content 정의 | `XX XXXXX`의 max-content 80px을 min-content로 오인할 수 있다. `width:min-content` probe에서 50px을 별도 관찰해 수용했다. |
| 4 | unbreakable text | 마지막 span-3 입력에 공백이 생기면 180px 기여가 줄어든다. 고정 18개 X와 `width:min-content` probe를 해시된 fixture에 고정했다. |
| 5 | image decode 경쟁 | decode 전 `naturalWidth=0`을 읽으면 transferred size가 무효다. 모든 image decode 완료와 natural 24×16 확인을 observation에 저장한다. |
| 6 | aspect-ratio transfer | 고정 `width:auto;height:48px`가 72px inline size를 만들지 못하면 비교 대상이 다르다. Chrome frame 72×48을 고정 기준으로 기록했다. |
| 7 | item-type grouping | text와 replaced input을 서로 다른 span pass로 분리하면 CSS Grid의 공통 span group을 검사하지 못한다. 계획은 단일 grid/revision 목록에서 종류별 source를 함께 추적한다. |
| 8 | node/item identity | reverse scene에서 DOM 순서만 바꾸고 item ID mapping을 재사용하면 실제 node association을 놓친다. scene별 `nodeIds`, 유형, source-order mapping을 validator에서 exact 검사한다. |
| 9 | span line off-by-one | `1 / span 2`를 세 열로 계산하거나 `2 / span 2`를 잘못 매핑할 수 있다. 독립 배치가 각각 [1,2], [2,3]을 고정하고 validator는 numeric span과 node를 함께 대조한다. |
| 10 | span group order | DOM order대로 span-3을 먼저 적용하면 최종 열이 달라질 수 있다. span-3-first DOM도 span-2 group 이후 같은 결과를 요구한다. |
| 11 | planned increase merge | 같은 span group의 50px와 72px contribution을 합산하면 과대 증가한다. track별 최대 planned increase를 적용하는 규범 유도를 명시했다. |
| 12 | rational arithmetic | `83/3`을 조기 반올림하면 f32 track 비교가 불안정하다. 독립 기대값은 rational로 보존하고 runtime/Chrome 각 허용치 1/64px를 따로 적용한다. |
| 13 | unused fourth track | span 범위 밖 fourth column이 contribution에 포함되면 결과가 틀리다. 네 번째 min-content track은 item이 걸치지 않으므로 0px이어야 한다. |
| 14 | flex-track 예외 | 이 scene을 flexible track의 automatic-minimum span 규칙으로 해석하면 잘못된 conclusion이 된다. 모든 track을 `min-content`로 고정하고 flex 조합은 명시적으로 범위 밖에 둔다. |
| 15 | row sizing 교란 | image의 48px height가 row contribution이나 다른 item과 세로 겹침을 만들면 column 결과가 흐려진다. 세 개 fixed 60px row에 item을 분리해 row sizing을 고정하고 image가 row 안에 들어오게 했다. |
| 16 | definite container clamp | 220px root 폭이 min-content track sum을 강제로 줄인다고 가정하면 180px span 결과가 오염된다. root/frame, used tracks와 intrinsic contribution을 따로 기록한다. |
| 17 | CSSOM serialization | fractional output string의 4자리 반올림을 exact f32 oracle로 취급할 수 있다. observation JSON double과 rational spec 기대를 별도 보존한다. |
| 18 | unsupported input partial result | 알 수 없는 scene/source를 일부 layout으로 받아들이면 fixture 결과가 과장된다. plan은 exact allowlist와 전체 오류 경계를 완료 조건으로 둔다. |
| 19 | screenshot 과장 | Chrome 화면을 Spinon/device/pixel equality로 오인할 수 있다. precomparison README는 브라우저 관찰만 증명한다고 한정한다. |
| 20 | artifact/API 누출 | 테스트 scene이 제품 ABI 또는 공개 CSS 지원으로 유출될 수 있다. 기본 artifact 부정 대조와 opt-in positive control을 계획의 구현 관문으로 고정했다. |

## 판정

계획 범위, oracle 독립성, layout geometry, cross-axis 조건, input fail-closed, artifact/API 경계를 서로 다른 실패 관점으로 확인했다. 15번 관점에서 48px image와 20px fixed row의 cross-axis overlap 위험을 찾아 row를 60px로 바꾸고 oracle을 재캡처했다. 최종 observation은 세 fixed row와 72×48 image frame을 보존한다. 누락을 찾아 계획/fixture를 수정한 뒤 계획 PR을 검토한다. 구현 성공이나 C11.4 완료 판정은 이 검토에서 주장하지 않는다.
