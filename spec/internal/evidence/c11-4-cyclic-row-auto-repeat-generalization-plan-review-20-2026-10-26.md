# C11.4 행 축 cyclic auto-repeat 일반화 계획 검토 20개

**검토 대상:** [행 축 cyclic auto-repeat fixture profile 일반화 계획](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md)
**일자:** 2026-10-26
**검토 유형:** 계획 문서의 서로 다른 실패 관점 20개. 구현/runtime 검토와 PR diff 검토에는 재사용하지 않는다.

계획 검토 담당자는 범위, 규범 계산, 장면·trace 계약, mutation 거부 경계, 플랫폼별 증거를 독립적으로 확인했다. 재검토에서 원시 관측과 독립 기대값 사이의 산술·좌표 불일치는 찾지 못했다. 처음 발견한 잔여 공간 설명과 evidence 저장 경로의 누락을 반영한 최종 계획은 20개 항목 모두 통과했다. 별도 설계 검토도 shared gutter 위치, suffix 좌표 `23/36/23px`, browser-only probe가 intrinsic basis에 미치는 영향이 없음을 확인했다.

이 기록은 문서·계획 검토 결과다. 구현, 빌드, 테스트, simulator 실행의 통과 근거로 간주하지 않는다.

| # | 실패 관점 | 판정 및 확인 결과 |
|---:|---|---|
| 1 | Max-content 행 축 auto-repeat 수를 definite 크기 기준으로 잘못 계산할 위험 | **PASS** — 첫 반복 수를 1로 두고 max-content와 definite control을 구별한다. [계획 §목적](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L13-L24) |
| 2 | Intrinsic cyclic percentage 처리와 최종 definite basis 계산을 혼합할 위험 | **PASS** — intrinsic pass의 `auto` 처리, 44px basis, 최종 11px을 별도 단계로 설명한다. [계획 §목적](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L15-L17) |
| 3 | 겹친 두 item contribution을 합산해 40px로 잘못 산출할 위험 | **PASS** — span 1 두 항목의 기여 24px·16px을 기록하고 track에는 최대값 24px을 쓴다. [계획 §목적](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L26), [독립 기대값](./c11-4-cyclic-row-auto-repeat-generalization-precomparison-2026-10-26/spec-derived-expectations.json#L57-L62) |
| 4 | 합이 31px인 track/gutter가 root 44px을 채우지 못하는 잔여 13px의 위치를 누락할 위험 | **PASS** — 두 max-content 장면에서 `align-content:start`가 잔여 공간을 suffix 뒤에 둔다고 명시하고 root 44px, suffix `y=23px`, 잔여 13px 검사를 요구한다. [계획 §목적 및 검증](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L15), [검증 관문](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L56-L57) |
| 5 | percentage resolution 뒤 row를 재확장한다고 잘못 가정할 위험 | **PASS** — target repeat count는 `1 → 1`이며 재확장 가설은 별도 Chrome 입력에서 기각한 것으로 분리한다. [계획 §목적](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L13-L15), [거부 가설](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L28) |
| 6 | Definite repeat count에서 고정 이웃 행과 gutter를 빼먹을 위험 | **PASS** — `8 + 8 + 11n + 2(n+1) ≤ 44`로 최대 count 2를 정한다. [계획 §목적](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L17) |
| 7 | `auto-fill` control의 두 반복과 suffix 위치가 틀릴 위험 | **PASS** — rows `[8,11,11,8]`, suffix `y=36px`를 고정한다. [계획 §검증](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L57) |
| 8 | `auto-fit` collapsed run의 공유 gutter를 이중 계산할 위험 | **PASS** — 2px shared boundary와 collapsed row 뒤의 0px gutter를 trace 순서로 지정한다. [계획 §목적 및 검증](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L17), [독립 기대값](./c11-4-cyclic-row-auto-repeat-generalization-precomparison-2026-10-26/spec-derived-expectations.json#L510-L568) |
| 9 | 점유된 max-content `auto-fit` 반복 행을 비었다고 보고 collapse할 위험 | **PASS** — 첫 repeated row를 두 item이 점유하므로 target에서 collapse하지 않는다. [계획 §목적 및 장면](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L15-L24) |
| 10 | Track list index와 repeat range를 혼동할 위험 | **PASS** — 전체 row list 길이 3, half-open 범위 `start=1,end=2`를 고정한다. [계획 §허용 profile](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L32-L35) |
| 11 | CSSOM의 resolved `minHeight:0px`를 authored fixed zero로 오해할 위험 | **PASS** — 미선언 값은 typed `Auto`이며 typed `Fixed(0)` mutation을 거부한다. [계획 §허용 profile 및 mutation](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L34-L35), [검증 관문](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L76) |
| 12 | Child의 크기·box sizing·placement·순서가 실제 profile과 달라질 위험 | **PASS** — prefix, primary, secondary 각각의 style·배치와 세 child cardinality를 명시한다. [계획 §허용 profile](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L35) |
| 13 | Browser-only suffix probe를 runtime input으로 섞을 위험 | **PASS** — probe는 관측용이며 runtime node가 아니라고 명시하고, mutation으로 추가하는 경우 거부하도록 한다. [계획 §허용 profile 및 mutation](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L35-L37), [검증 관문](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L74) |
| 14 | Pass, contribution, gutter, collapse trace의 순서나 owner를 잃을 위험 | **PASS** — root grid node의 leaf contribution, pass별 count/basis, 모든 gutter와 collapse flag를 기록한다. [계획 §검증](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L56-L57) |
| 15 | 분수 CSS 좌표에 정수 exact 비교를 적용하거나 오차를 넓힐 위험 | **PASS** — 정수 frame은 exact, 분수 track/좌표는 1/64 CSS px 허용오차로 구별한다. [계획 §검증](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L56), [독립 기대값](./c11-4-cyclic-row-auto-repeat-generalization-precomparison-2026-10-26/spec-derived-expectations.json#L7-L8) |
| 16 | CSS 좌표와 device pixel, paint 결과의 범위를 혼동할 위험 | **PASS** — root origin과 실행 DPR을 적용한 정수 device 좌표·원시 RGBA를 요구하고 border, root 밖 overflow, hardware/compositor 주장을 제외한다. [계획 §pixel 표본](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L78-L88) |
| 17 | Android/iOS 결과를 같은 산출물 hash로 오기록할 위험 | **PASS** — 두 앱은 같은 source commit을 사용하되 플랫폼별 build/install hash를 각각 남긴다. [계획 §runtime matrix](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L77) |
| 18 | 지원하지 않는 입력이 sizing까지 흘러가거나 오류 owner가 흐려질 위험 | **PASS** — exact-scene predicate가 지원 여부를 결정하고 오류 자체는 row-axis validator가 구성함을 구분한다. Max-content row target에 definite column auto-repeat를 더하는 mutation은 `InvalidGridTrackList(grid-template-rows)`, 두 축을 모두 cyclic하게 만드는 mutation은 검증 순서상 `InvalidGridTrackList(grid-template-columns)`를 기대한다. Runtime style projection은 별도 property 오류를 낸다. [계획 §mutation 표](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L58-L78) |
| 19 | 기대값·브라우저 관측·fixture·screenshot의 출처와 hash를 혼합할 위험 | **PASS** — 독립 기대값 JSON과 Chrome 원본을 분리하고 fixture·관측·screenshot hash, Chrome·viewport·DPR을 기록한다. [사전 비교 README](./c11-4-cyclic-row-auto-repeat-generalization-precomparison-2026-10-26/README.md#L1-L18), [hash 목록](./c11-4-cyclic-row-auto-repeat-generalization-precomparison-2026-10-26/README.md#L37-L49) |
| 20 | 네 장면 결과를 일반 CSS Grid·WPT·C11.4 완료로 확대 주장할 위험 | **PASS** — profile은 opt-in 고정 fixture에 한정하며 공식 `72/213`, C11.4 checkbox, public support claim을 그대로 유지한다. [계획 §범위](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L9-L10), [허용 profile](../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md#L30-L38) |

## 검토 후 조정

- 첫 검토에서 두 max-content scene의 suffix 뒤 13px 잔여 공간 판정·assertion이 빠져 있어 **4번 PARTIAL**이었다. 계획에 `31px` track/gutter 합, `44px` root, `align-content:start`에 따른 suffix 뒤 `13px`, suffix `y=23px` 검사를 추가한 뒤 재검토에서 PASS로 바뀌었다.
- 별도 검토 evidence 경로는 계획에 연결했지만 파일은 본 기록을 작성할 때 생성했다. 따라서 **19번 PENDING**은 이 파일에 20개 결과를 보존함으로써 해소했다.
- Mutation별 오류 owner/property와 계획 질문의 한글화를 보완했다. 다른 19개 관점에서 새 수치·좌표·범위 문제는 발견되지 않았다.
- PR diff 검토에서 다른 축 auto-repeat mutation 누락과 즉시 오류 owner의 부정확한 설명을 발견했다. Max-content row target의 definite column repeat는 row validator의 `grid-template-rows` 오류로, 두 축 모두 indefinite cyclic repeat를 갖는 입력은 먼저 실행되는 column validator의 `grid-template-columns` 오류로 나누어 계획 및 18번 관점에 추가했다. 소스 추적 재검토에서 두 오류 경로를 확인했다.

**검증 경계:** 본 기록은 계획 문서 review다. 구현 PR의 별도 20개 runtime review와 PR diff 30개 review를 대체하지 않는다.
