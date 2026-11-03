# C11.5 Root HTML Grid auto-size/stretch 계획 실패 관점 검토 20개

검토 대상은 [auto-size/stretch 계획](./c11-root-html-grid-item-auto-stretch.md), Chrome 156 사전 비교, 독립 CSS 규범 기대값이다. C11.5 fixed-size 계획 검토나 구현/PR diff 검토를 대신하지 않는다.

| # | 실패 관점 | 결과 및 방어 |
|---:|---|---|
| 1 | 제한 scene 구현을 C11.5 또는 C11 전체 완료로 표시 | **통과.** 계획은 C11.5/C11 미완료와 공식 `72/213` 유지 조건을 명시한다. |
| 2 | `normal`을 replaced/aspect-ratio item에도 stretch로 적용 | **통과.** 입력은 빈 non-replaced `div`, preferred aspect ratio 없음으로 한정한다. |
| 3 | `align-self:auto`가 자식에서 부모 `align-items` 대신 초기값으로 해석 | **통과.** parent `normal`과 `start` 각 대조군, 정확한 parent/child computed values 및 used frame 검증을 요구한다. |
| 4 | `justify-self:auto`의 상속 대상을 `align-items`로 혼동 | **통과.** inline/block axis 별로 대응하는 `justify-items`/`align-items` 값 확인을 계획에 넣었다. |
| 5 | `normal`의 Grid 규칙을 다른 formatting context에서 가져옴 | **통과.** Grid item 전용 normative rule과 기존 Root HTML Grid 경로를 범위로 둔다. |
| 6 | `stretch`가 fixed width/height까지 덮어씀 | **통과.** 양 축 fixed 대조군에서 기존 16×10 크기 유지 조건이 있다. |
| 7 | 한 축 stretch가 다른 축까지 확장 | **통과.** align-only와 justify-only scene 및 width-only/height-only fixed scene을 별도로 둔다. |
| 8 | positional `start`에서 `auto` size가 무조건 0으로 단정 | **통과.** 기대값은 content 없는 빈 div에만 적용하고 content-bearing child는 거부한다. |
| 9 | `normal`/`stretch`에 parent 기본값 차이가 가려짐 | **통과.** self explicit와 self auto+parent normal/start 장면을 각각 포함한다. |
| 10 | auto margin이 stretch 동작을 바꿈 | **통과.** margin은 0으로 고정하고 auto/nonzero margin은 negative control이다. |
| 11 | padding/border/box-sizing 때문에 frame이 cell보다 커짐 | **통과.** zero edge와 `border-box`를 고정하고 edge 변경은 거부한다. |
| 12 | min/max clamp가 stretch 결과를 바꾸지만 기대값에서 누락 | **통과.** 기존 `min:auto`, `max:none`을 유지하며 변경 입력은 fail-closed control이다. |
| 13 | preferred size의 px 외 단위가 제한 profile로 새어 들어옴 | **통과.** 축마다 auto 또는 finite non-negative px만 허용한다. |
| 14 | 음수/NaN/무한 길이로 invalid geometry 생성 | **통과.** finite non-negative px만 입력으로 허용하고 실패 시 장면 발행을 금지한다. |
| 15 | item의 intrinsic content가 빈 div baseline을 오염 | **통과.** text와 descendants를 제외하고 content-bearing 입력은 별도 거부한다. |
| 16 | implicit/auto track이나 span이 cell extent를 바꿈 | **통과.** definite container, 단일 explicit fixed track/cell, auto placement를 유지하고 implicit/auto/span은 제외한다. |
| 17 | computed value 대신 raw authored string을 적용 | **통과.** computed parent/self snapshot을 값의 원본으로 요구하고 shorthand는 이 계획 범위 밖이다. |
| 18 | scene ID나 fixture 순서가 style 선택을 대신함 | **통과.** plan은 정확한 node/style owner와 snapshot에서 값·revision을 확인하도록 한다. |
| 19 | unsupported node 일부만 배치되어 partial scene이 게시 | **통과.** negative control이 diagnostic과 pre-publication atomic failure를 확인한다. |
| 20 | Chrome oracle 또는 simulator 제한을 과장하거나 item이 cell 경계 밖으로 나가는 회귀를 누락 | **통과.** Chrome 관측과 독립 계산을 분리하고 모든 item frame의 cell containment를 확인한다. Android software renderer 및 iOS drawable/compositor 한계도 별도로 기록한다. |

## 결론

이 계획은 빈 비대체 fixed-cell fixture에서 자동 크기 축의 `normal`/`stretch` 동작을 다음 구현 slice로 고정한다. 일반 CSS 지원, 전체 C11.5/C11 완료, WPT conformance를 암시하지 않는다.
