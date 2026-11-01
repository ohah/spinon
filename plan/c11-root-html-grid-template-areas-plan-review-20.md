# C11.1 Root HTML named-area 계획 · 실패 관점 20개

**대상:** [계획](./c11-root-html-grid-template-areas.md), Chrome 156 사전 비교 fixture와 독립 expected JSON. 기능 구현이나 Root HTML runtime 동작을 검증하는 문서가 아니다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | C11.3.3 fixture profile을 기본 Root HTML 경로로 잘못 간주할 수 있음 | **통과.** 목표는 `new_runtime_gpu_with_root_html()`의 기존 기본 route이며 fixture-test constructor에 종속되지 않는다. |
| 2 | computed area string을 직접 다시 파싱할 수 있음 | **통과.** 계획은 같은 revision의 typed `template_areas`와 typed item placement만 사용한다. |
| 3 | area가 CSSOM serialization만으로 결정되었다고 오인할 수 있음 | **통과.** 독립 expected geometry는 고정 cells/track sums에서 계산하고 Chrome은 교차 관측으로 분리했다. |
| 4 | `grid-area` shorthand가 네 placement longhand와 다른 의미로 취급될 수 있음 | **통과.** Browser observation은 shorthand 및 네 computed longhand 값을 보존하고 implementation은 typed placement를 요구한다. |
| 5 | repeated area를 rectangle이 아닌 임의 모양으로 지원한다고 해석될 수 있음 | **통과.** 고정 2×2 직사각형만 명시하고 arbitrary area grammar를 제외했다. |
| 6 | area order와 DOM order를 혼동할 수 있음 | **통과.** Content→header→side 원본과 DOM reverse control을 고정하고 semantic frames 비교를 요구한다. |
| 7 | semantic item을 NodeId, class, DOM index로 고를 수 있음 | **통과.** ID/selector 변경 control을 포함하고 computed area value를 semantic owner로 삼는다. |
| 8 | repeated cells가 track 수를 늘리거나 row/column 수가 뒤바뀔 수 있음 | **통과.** 행렬 2행×2열, columns `[40,80]`, rows `[20,50]`을 분리해 기록했다. |
| 9 | header가 첫 셀만 차지할 수 있음 | **통과.** 두 column cell을 점유하며 독립 frame은 폭 `40+80=120px`이다. |
| 10 | side와 content가 좌우로 바뀔 수 있음 | **통과.** Area matrix 및 각 item geometry를 별도 기대값으로 적었다. |
| 11 | gap을 track 크기에 더하거나 이중 반영할 수 있음 | **통과.** 이 fixture의 gap은 0이며 nonzero gap은 #1002의 기존 route 계약으로 분리된다. |
| 12 | auto-placement가 named placement보다 먼저 적용될 수 있음 | **통과.** 세 항목은 각각 computed `grid-area`로 배치하며 auto-placement 동작을 성공 기대하지 않는다. |
| 13 | unassigned/extra/duplicate item이 조용히 implicit cell에 배치될 수 있음 | **통과.** 정확히 세 area와 세 item의 bijection을 요구하고 extra/duplicate/missing cases를 거부한다. |
| 14 | unknown area name이 named implicit line을 만들어 허용 범위 밖 배치가 될 수 있음 | **통과.** unknown area 및 implicit tracks는 publish 전 거부하도록 계획했다. |
| 15 | template dimension mismatch가 partial layout으로 노출될 수 있음 | **통과.** computed template/track dimension 일치와 malformed/invalid controls를 요구한다. |
| 16 | source-order reverse가 paint order 보존을 깨뜨릴 수 있음 | **통과.** semantic frame 검사와 별개로 DOM/source paint order를 보존하도록 계약했다. |
| 17 | CSS invalid declaration drop과 valid-but-unsupported 오류가 섞일 수 있음 | **통과.** syntax-invalid cascade controls와 computed profile rejection을 분리한다. |
| 18 | stale document/style/layout revision의 frame이 성공처럼 보일 수 있음 | **통과.** 모든 typed values와 layout snapshot이 같은 revision인지 확인하고 stale revision control을 둔다. |
| 19 | root overflow나 surface capture를 geometry로 오인할 수 있음 | **통과.** layout frame/draw-present와 optional PixelCopy/Metal evidence를 분리하고 compositor 경계를 명시한다. |
| 20 | 제한 fixture를 public CSS, WPT 또는 C11 완료로 확대 보고할 수 있음 | **통과.** 지원/제외 범위와 공식 checkbox·`72/213` 불변 조건을 계획과 precomparison에 적었다. |

**판정:** 범위·독립 기대값·typed input source·실패 경계와 검증 게이트를 대조했다. 계획은 별도 구현 전 PR 및 구현/runtime 검토 대상으로 진행할 수 있다. 이 기록은 구현이나 mobile simulator를 통과 처리하지 않는다.
