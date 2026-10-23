# C11.4 양 축 auto-repeat + template areas 계획 적대 검토 20개

**검토 대상:** [집중 계획](./c11-4-two-axis-template-areas.md), [규범 독립 기대와 Chrome 관찰](../spec/internal/evidence/c11-4-two-axis-template-areas-precomparison-2026-10-23/README.md), [9-scene fixture](../tests/fixtures/css/c11/grid-auto-repeat-template-areas-two-axis-c114.html).

**판정:** 아래 독립 실패 경로를 계획에 반영했다. 구현/runtime 결과는 아직 없으며 이 검토는 구현 검토나 PR diff 검토를 대체하지 않는다.

| # | 실패 관점 | 계획의 방어선 | 판정 |
| --- | --- | --- | --- |
| 1 | 두 auto-repeat metadata 중 한 축이 사라져 다른 축의 count를 재사용한다. | Computed layout부터 GridContainerInput·Taffy style까지 두 axis를 독립 비교하고 scene별 used track을 기록한다. | 통과 |
| 2 | 같은 track list에 repeat 두 개까지 허용 범위가 넓어진다. | 축마다 하나만 허용하고 한 축의 중복 repeat은 기존 구체 오류로 유지한다. | 통과 |
| 3 | 기존 두 축 auto-repeat의 fill/fill 및 fit/fit 동작이 areas 경로 변경으로 회귀한다. | 기존 12-scene 양축 auto-repeat matrix를 별도 exact ID 회귀로 유지한다. | 통과 |
| 4 | 기존 한 축 auto-repeat + areas 지원이 차단 gate 삭제 중 손상된다. | 기존 13-scene area matrix를 별도 exact ID 회귀로 유지한다. | 통과 |
| 5 | template areas가 target profile 외 CSS profile에 유입된다. | 허용은 `RuntimeBlockFlexGridTrackSizingV1`만이며 나머지 profile은 negative control로 거부한다. | 통과 |
| 6 | area dimension이 expanded repeat보다 큰 축에서 track 수가 잘린다. | column-only·row-only·both-axis 추가 track scene을 두고 각 최종 count와 frame을 고정한다. | 통과 |
| 7 | area dimension을 반복 횟수에 합쳐 auto-fill count를 과대 산정한다. | definite root size로 repeat count를 독립 산출하고 확장 track은 뒤에 `grid-auto-*`로 분류한다. | 통과 |
| 8 | auto-repeat count 계산에서 gap을 빼먹어 40/76/100px boundary가 달라진다. | CSS Grid §7.2.3.2 공식에서 `n×20 + (n−1)×4`를 독립 계산하고 Chrome observation을 대조한다. | 통과 |
| 9 | area가 짧을 때 auto-fill 뒤쪽 반복 track이 implicit track으로 바뀌거나 사라진다. | `area-short`에서 4×3 repeat과 2×2 area를 함께 고정하고 full track arrays를 비교한다. | 통과 |
| 10 | auto-fit은 열만 접고 행은 유지하거나 그 반대로 축 상태를 공유한다. | `fill-fit`과 `fit-fill`의 expected arrays/frame을 서로 바꿔 검사한다. | 통과 |
| 11 | 이름 있는 area만으로 auto-fit track이 occupied 처리된다. | `fit-fit-empty`는 area를 선언하고 in-flow item을 제거하며 모든 반복 track의 0px collapse를 기대한다. | 통과 |
| 12 | 2×2 area item span이 교차 track 중 일부를 empty로 오판한다. | `fit-fit-span` item을 2×2 area에 stretch하고 첫 두 track만 두 축에서 유지되는지 확인한다. | 통과 |
| 13 | collapsed track 옆 gutter가 남아 suffix와 item frame이 밀린다. | `fill-fit`, `fit-fill`, `fit-fit-empty`, `fit-fit-span`에서 used sizes와 node origin을 비교한다. | 통과 |
| 14 | `hero-start`/`hero-end`를 한 축에서만 만들어 longhand placement가 비대칭으로 틀린다. | `area-longhand`는 두 축에서 같은 area alias를 각각 `grid-column`·`grid-row`로 사용한다. | 통과 |
| 15 | 추가 explicit area track이 fixed fixture 외의 자동 sizing grammar를 암묵 허용한다. | 이 scope는 고정 `16px`/`12px` `grid-auto-*`뿐이며 다른 value는 기존 제한을 유지한다. | 통과 |
| 16 | 잘못된/너무 큰 area metadata가 일부 축만 투영한 뒤 partial layout을 게시한다. | checked conversion, 기존 metadata cap, allocation failure 및 no-partial-publication control을 추가한다. | 통과 |
| 17 | track/frame 결과만 보고 `area-added-both`의 grid 바깥 item paint를 통과 처리한다. | Android PixelCopy/iOS Metal에서 root 안, 오른쪽·아래 overflow item, 외부 clear를 실제 좌표로 읽는다. | 통과 |
| 18 | screen sample이 border stroke·hardware·compositor 뒤 픽셀 증거로 확대 해석된다. | PixelCopy는 API 37 software AVD, Metal은 compositor 전 readback으로 쓰고 각 한계를 명시한다. | 통과 |
| 19 | fixture source가 기본 FFI product archive에 함께 링크된다. | default archive negative와 `c04-runtime-gpu-fixture-test` positive control을 각각 검사한다. | 통과 |
| 20 | 계획 검토를 구현 검토나 30관점 PR diff 검토에 재사용하거나 C11.4 완료로 승격한다. | 구현 runtime 20, 각 PR diff 30을 분리하고 공식 checkbox·`77/213`·계약 `0.1.0`을 유지한다. | 통과 |

## 반영 후 판정

Chrome 156 관찰 9/9가 W3C CSS Grid 2025 CRD에서 독립 계산한 track list와 root-relative item frame에 일치한다. `area-added-both`는 CSS surface에서 root 오른쪽 및 아래 경계 밖으로 넘치므로 Native Simulator gate에서 실제 PixelCopy/Metal 표본을 요구한다. 계획 범위에는 구현·platform 결과를 선반영하지 않았다.
