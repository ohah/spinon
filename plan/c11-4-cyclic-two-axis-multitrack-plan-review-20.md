# C11.4 동시 cyclic percentage 다중 track 계획 검토 20개

검토 대상: [계획](./c11-4-cyclic-two-axis-multitrack.md), 고정 HTML fixture와 Chrome 156 관찰, CSS Grid 2025 CRD §7.2.1·§11·§11.1에서 독립 계산한 기대값.

| # | 독립 실패 관점 | 판정과 계획의 대응 |
| ---: | --- | --- |
| 1 | intrinsic root sizing 전에 cyclic percentages를 실제 percentage로 풀어 basis를 축소 | 계획은 intrinsic container sizing에서 cyclic percentage를 `auto`로 취급하고 final sizing에서 다시 푸는 두 단계를 따로 계산한다. |
| 2 | columns percentages를 block-size basis 또는 rows percentages를 inline-size basis에 적용 | Columns는 40px inner inline size, Rows는 20px inner block size를 각각 사용한다. |
| 3 | 두 cyclic 축을 한 번의 동시 계산으로 풀어 row sizing이 이전 column sizing을 놓침 | §11.1 순서대로 columns를 먼저 풀고 text item의 row contribution에는 그 column 결과를 사용한다. |
| 4 | `max-content` root의 definite/intrinsic 성격을 혼동 | both scene의 두 축은 track 의존 `max-content`; single-axis control에서는 나머지 축을 고정 길이로 분리한다. |
| 5 | 첫 cyclic track 외의 빈 두 번째 cyclic track을 auto basis에서 누락 | 각 축의 두 번째 percentage track을 명시하고 empty track의 intrinsic 기여가 0임을 fixture와 계산에서 고정한다. |
| 6 | fixed third track의 길이를 percentage basis에서 빠뜨림 | inline basis `20 + 0 + 20 = 40px`, block basis `10 + 0 + 10 = 20px`로 합산한다. |
| 7 | percentage track 길이 합이 root content box보다 큰 결과를 root size 증가로 오해 | final used track 합 `50×25px`과 intrinsic root `40×20px`를 별도 판정해 overflow 10×5px을 유지한다. |
| 8 | percentage 비율을 50% track 하나에만 적용하고 25% track에 누락 | 네 장면 모두 track count와 각 CSSOM used length를 각각 세 값으로 검사한다. |
| 9 | Ahem font 미로드 또는 fallback으로 20px inline basis가 변함 | 브라우저 capture는 Ahem font load/status를 필수 metadata로 남기며 미로드 capture는 invalid로 처리한다. |
| 10 | 줄바꿈/line-height 변경으로 10px block basis가 변함 | `aa`는 공백 없는 단일 line, font-size와 line-height는 모두 10px로 고정한다. |
| 11 | overflow child가 max-content basis를 의도치 않게 늘림 | child는 세 번째 fixed track과 동일한 20×10px로 두고, 고정 track이 intrinsic basis를 늘리지 않음을 독립 계산한다. |
| 12 | track 3 placement 오류로 암묵 track을 추가하거나 overflow origin 이동 | explicit 3×3 tracks와 `grid-column/row:3`을 allowlist하고 trace에서 각 축 track count가 3인지 확인한다. |
| 13 | 축별 cyclic control이 실제 해당 축 하나만 intrinsic인지 불분명 | columns-only는 height와 rows를 고정하고 rows-only는 width와 columns를 고정한다. |
| 14 | definite control에 cyclic 기대값을 재사용하거나 별도 분기 누락 | width 40px·height 20px definite control의 percentage values를 각 고정 basis로 직접 풀어 같은 track 배열을 별도 비교한다. |
| 15 | root, text, overflow item frame을 track list만으로 추정 | trace의 used tracks, 세 node frame, root-relative overflow origin을 함께 확인한다. |
| 16 | item이나 root의 padding/border/gap 때문에 percentage inner basis가 달라짐 | padding·border·gap 0을 입력과 runtime scene에 고정해 inner basis가 40×20px가 되도록 한다. |
| 17 | CSSOM/root 화면의 누적 page offset을 root-relative 위치 오차로 보고 | page absolute `y`는 acceptance에서 제외하고 item 위치는 각 root 원점 기준으로 비교한다. |
| 18 | overflow box의 track 밖 paint와 root 밖 paint를 같은 것으로 취급 | root-relative point `(45,22)`가 green item 안이면서 40×20 root 밖임을 검사하고 track/frame 결과도 별도로 대조한다. |
| 19 | CSS pixel 표본을 Simulator physical pixel로 잘못 읽거나 surface 단계 과장 | PixelCopy/Metal 표본은 display scale로 좌표 변환하고 Android AVD·iOS Simulator와 실기기, compositor 전/후를 분리 보고한다. |
| 20 | fixed fixture를 일반 CSS Grid 지원·WPT 통과·C11.4 완료로 확대 | 네 scene만 허용하며 일반 CSS admission, WPT, hardware GPU, 실기기와 C11.4 전체 상태는 미완료로 유지한다. |

**판정:** 독립 basis `40×20px`, final tracks와 root 밖 overflow item 위치가 규범 단계와 모순되지 않는다. 브라우저 capture는 이 계산과 일치하며 계획 단계의 비교값으로만 보존했다. 수정이 필요한 차단 항목은 발견하지 못했다. 이 20개 관점은 구현/runtime 검토나 PR 변경분 검토에 재사용하지 않는다.
