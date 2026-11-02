# Root HTML `fr`·percentage + fixed implicit row 계획 PR 변경 diff 실패 관점 검토 30개

검토 대상은 `plan/c11-root-html-grid-fr-percent-implicit-rows.md`, C11 계획·상태 대장 링크, Chrome Agent Browser precomparison HTML/관측/기대값/screenshot, 별도 계획 검토 20개다. 기준 base는 `17fd9d3d034d8c45c0c16075fe96e988f3dbf1ca`다. 계획 변경과 구현 변경을 구분해 검토했다.

| # | 독립 실패 관점 | diff에서 확인한 방어 | 판정 |
|---:|---|---|---|
| 1 | 계획이 C11 전체 또는 public CSS support를 완료로 보이게 하는가 | 계획만 추가하며 C11/C11.1/C11.3.3/C11.4와 `72/213`을 그대로 둔다고 명시한다. | PASS |
| 2 | 이미 허용된 feature의 정확한 결합 결손을 말하는가 | #1015/#1019 track slices와 #1024 fixed implicit-row 제한의 현재 경계를 대조한다. | PASS |
| 3 | 구현 대상이 opt-in scene registry로 새는가 | 기본 `new_runtime_gpu_with_root_html()` 경로를 명시하고 fixture profile을 금지한다. | PASS |
| 4 | API/profile 버전이 계획만으로 바뀌는가 | 새 public API·fixture route·interface version 변경을 추가하지 않는다. | PASS |
| 5 | projection admission만 넓히고 layout contract가 닫혀 있는가 | projection과 `RootHtmlImplicitRowsV1`의 양쪽 변경/검증을 요구한다. | PASS |
| 6 | layout track validator의 다른 profile이 같이 넓어지는가 | 기존 TrackSizing/template admission 규칙의 비변경을 관문에 넣는다. | PASS |
| 7 | percentage가 indefinite inline/block basis에서 우연히 계산되는가 | 해당 root content axis definite 조건과 negative case를 분리한다. | PASS |
| 8 | row flexible factor가 fixed implicit track을 제외하지 않는가 | 60px block에서 15px implicit row를 뺀 45px를 1:2로 분배한다. | PASS |
| 9 | gap이 fractional free-space calculation에서 누락되는가 | gap 5px percentage-column scene과 zero-gap factor controls를 분리한다. | PASS |
| 10 | percentage track과 gap의 valid overflow를 잘못 clamp하는가 | used tracks `[30,90]`, gap 5px와 root/item의 15px overflow를 고정한다. | PASS |
| 11 | overflow item의 frame과 painted surface가 혼동되는가 | root-relative frame, PixelCopy/Metal readback, screenshot 근거를 구분한다. | PASS |
| 12 | row percent basis가 implicit row 또는 leftover space로 계산되는가 | definite 80px root의 explicit rows `[20,20]`, implicit row 15px, free space 25px를 각각 기록한다. | PASS |
| 13 | root width/height 또는 box-sizing으로 expected basis가 달라지는가 | definite CSS-px root와 zero edges, fixed dimensions를 scene input에 둔다. | PASS |
| 14 | `grid-auto-rows`가 list/percentage/fr/intrinsic size까지 함께 허용되는가 | 한 개의 non-negative px 값만 남기고 나머지는 reject 목록에 둔다. | PASS |
| 15 | `grid-auto-columns`도 우연히 확대되는가 | auto 값만 허용하고 implicit columns를 계속 거부한다. | PASS |
| 16 | direct breadth whitelist가 `minmax()`·`fit-content()`·`auto`까지 확장되는가 | px/fr/percentage direct variants만 허용하며 해당 문법은 제외한다. | PASS |
| 17 | named line/area 또는 auto-repeat과 조합되어 새로운 resolver contract가 필요한가 | line names, areas, auto-repeat과 authored placement를 범위 밖으로 둔다. | PASS |
| 18 | sparse placement에 없는 implicit column이 생성되는가 | 기본 row flow, explicit columns, auto implicit columns만을 요구한다. | PASS |
| 19 | empty fixed child 조건이 intrinsic item sizing을 통과시키는가 | text/image/nested/intrinsic item을 계속 reject하고 fixed block만 둔다. | PASS |
| 20 | source reorder와 ID rename 결과를 같은 의미로 비교하는가 | rename은 geometry 보존, reorder는 CSS auto-placement frame ownership 변경으로 구분한다. | PASS |
| 21 | hidden child가 occupied cell이나 implicit row 수에 들어가는가 | `display:none` exclusion positive control을 계획했다. | PASS |
| 22 | negative breadth가 f32/Taffy로 전달되는가 | negative/NaN/infinite typed values를 직접 거부하는 matrix를 요구한다. | PASS |
| 23 | invalid CSS parse fallback과 unsupported computed value diagnostic이 뭉개지는가 | cascade drop controls와 projection rejection을 별도로 비교한다. | PASS |
| 24 | cap/stale/layout failure 후 partial 또는 cached output이 남는가 | 10,000 track ceiling, same-revision gates와 partial frame 부재를 acceptance에 둔다. | PASS |
| 25 | precomparison 기대값이 Chrome 출력에서만 복사되는가 | 고정 root/tracks에서 계산한 expected JSON과 Chrome observation JSON을 분리한다. | PASS |
| 26 | expected JSON이 실제 Agent Browser source/screenshot과 다른가 | HTML 및 PNG SHA-256을 저장하고 machine check로 재확인했다. | PASS |
| 27 | Chrome observation이 generic browser version으로 모호한가 | `HeadlessChrome/156` user agent와 1280×633 viewport를 보존한다. | PASS |
| 28 | local docs links가 잘못되어 plan/status가 열리지 않는가 | plan→evidence, STATUS→plan, master plan→plan 상대 경로를 대조했다. | PASS |
| 29 | CI, device, pixel readback을 계획 검토나 screenshot만으로 주장하는가 | 미실행 gate는 금지하고 Android renderer·Metal 시점 및 no-check 상태 기록을 요구한다. | PASS |
| 30 | PR 변경 diff의 실질적인 실패 관점이 20개 요청과 project rule을 충족하는가 | 구현 전 plan review 20개와 이 PR의 별도 30개 검토를 각각 기록한다. | PASS |

## 결과

30개의 서로 다른 계획·범위·oracle·실행·문서 실패 경로를 검토했다. 수정할 blocker는 발견하지 못했다. 이 검토는 구현/runtime 20관점 검토를 대신하지 않는다.
