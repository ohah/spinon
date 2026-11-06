# C11.4 parsed Root HTML percentage `row-gap` 계획 실패 관점 검토 20개

검토 대상: [percentage row-gap 계획](./c11-root-html-auto-row-percentage-gap.md), [70-scene Chrome 사전 비교와 독립 기대값](../spec/internal/evidence/c11-root-html-auto-row-percentage-gap-precomparison-2026-11-07/README.md), 현재 `RootHtmlAutoRowStretchV1` projection/layout guards. 기능 코드 변경 전 계획을 검토했다.

| # | 서로 다른 실패 관점 | 계획 검토 결과 |
|---:|---|---|
| 1 | percentage가 row track 크기 대신 Grid의 inline 크기를 기준으로 계산되는가 | **통과** — 세로축 40px content-box height를 basis로 고정하고, 동일 비율을 40px 기준의 독립 oracle과 Taffy에 전달한다. |
| 2 | 40px 선언이 실제 content box와 달라질 수 있는 padding/border/box-sizing/zoom 환경인가 | **통과** — 기존 exact Root HTML shape 검증을 유지하고 fixture의 computed width/height·box sizing·zero edge conditions를 확인한다. |
| 3 | parsed typed percentage를 잃고 authored 문자열 또는 px로 재계산하는가 | **통과** — computed `Percentage(fraction)`만 받으며 projection부터 `LayoutLengthPercentage::Percentage`까지 비율을 보존한다. |
| 4 | Taffy가 percentage basis를 resolve하는데 adapter에서 다시 px 변환해 double-resolve하는가 | **통과** — manual gap 보정 없이 typed percentage를 Taffy에 전달하고 final used gap을 별도 검사한다. |
| 5 | 첫/마지막 track 바깥 gutter까지 더해 free-space를 잘못 계산하는가 | **통과** — 세 row 사이의 gutter 두 개만 합산하며 independent formula를 모든 scene에 대조한다. |
| 6 | zero, positive-free-space, equality, negative-free-space 구간을 놓치는가 | **통과** — 0%·6.25%·12.5%·18.75%·20%·100%·120%가 각각 free space 10·5·0·-5·-6·-70·-86px 구간을 덮는다. |
| 7 | 100% 초과 gap을 clamp하거나 음수 free-space alignment를 양수 공간처럼 분배하는가 | **통과** — 120%를 유지하고 모든 alignment frame과 기존 Grid fallback을 검증한다. |
| 8 | `space-between`·`space-around`·`space-evenly` 음수 공간 fallback 또는 start/end alias가 빠지는가 | **통과** — 10개 computed alignment를 각각 유지하고 Chrome observation과 독립 계산을 분리한다. |
| 9 | 40px 기준과 세 base row의 합이 `f32` layout 범위를 넘는 거대한 유한 ratio를 수용하는가 | **통과** — 두 gutters와 30px base tracks를 합산한 finite-f32 representability guard와 경계 단위 검사를 요구한다. |
| 10 | 음수, NaN, ±infinity 또는 수학식 값이 projection gate를 우회하는가 | **보완** — 계획 전제가 아닌 실제 코드를 확인했다. 현재 Spinon parser는 authored `row-gap:-5%`를 fatal cascade diagnostic으로 만든다. 이 parser recovery 차이를 이 bounded layout slice의 범위 밖으로 명시하고, direct typed negative/non-finite values와 valid-but-unsupported `calc()` 거부를 각각 검증한다. |
| 11 | 0%·`normal`이 fixed 0px과 다르게 처리되거나 기존 px 0..7.5px 동작이 바뀌는가 | **통과** — 기존 px allowlist 회귀, percentage zero scene, Grid에서 used zero인 `normal` 초기값 회귀를 둔다. |
| 12 | 같은 helper를 공유하는 다른 Root HTML profile까지 새 percentage gap을 받는가 | **통과** — `RootHtmlAutoRowStretchV1`의 전용 gate만 확장하고 다른 placement mode/profile 회귀를 명시한다. |
| 13 | column-gap percentage 또는 non-zero column-gap이 row-only 확장에 섞이는가 | **통과** — `column-gap:0` exact 조건을 cascade와 Layout validator에서 각각 유지한다. |
| 14 | auto rows 개수, explicit track, direct child 수, item size가 함께 넓어져 원인을 숨기는가 | **통과** — 40px column, 10px explicit row, 두 10px implicit auto rows, direct child 3개를 고정한다. |
| 15 | item sizing·visibility·position·writing mode 등 기존 실패 경계가 느슨해지는가 | **통과** — 기존 static position, track/item shape, visibility 및 alignment allowlist 제약을 유지하고 unsupported control을 거부한다. |
| 16 | stale revision, 잘못된 owner 또는 unsupported declaration 뒤 partially published layout이 남는가 | **통과** — owner/revision 검사와 실패 원자성·거부 후 복구를 RuntimeSession acceptance criteria에 포함한다. |
| 17 | 70개 expected value가 Chrome 출력에서 직접 복사되어 oracle 순환 검증이 되는가 | **통과** — raw Chrome JSON은 관측 자료로만 두고 수식 생성기와 independent expected JSON을 구현 전 고정한다. |
| 18 | fractional CSS coordinate quantization을 exact equality로 판단하거나 비교 허용차를 사후 확대하는가 | **통과** — precomparison에서 1/32px 허용차를 고정했고 최대 관측 오차·원본값을 보존한다. |
| 19 | 앱 검증용 scene/adapter/payload가 기본 Android/iOS artifact에 들어가는가 | **통과** — 계획을 보완해 기존 opt-in `*-fixture-test` 경로만 사용하고 default와 fixture artifact 경계를 따로 확인한다. |
| 20 | 화면의 overflow가 app surface에 실제로 나타나는지와 screenshot/readback 범위를 혼동하는가 | **통과** — 120%/center의 grid 밖 pixel을 확인하고 Android PixelCopy·iOS drawable readback·screenshot 관찰을 구분한다. Screenshot-only 결과는 surface readback으로 보고하지 않는다. |

## 결론

계획 검토에서 CSS-invalid 음수 declaration의 fatal cascade 진단을 실제 코드에서 확인해 parser compatibility limitation으로 기록하고, direct typed-value 거부와 구분했다. 앱 fixture 경계와 표면 관측 분류도 구체화했다. 계획 기준은 구현 전에 고정했으며, 구현/runtime review 20개와 PR 실제 diff review는 별도 산출물로 새로 수행한다. C11.4와 C11은 미완료로 유지한다.
