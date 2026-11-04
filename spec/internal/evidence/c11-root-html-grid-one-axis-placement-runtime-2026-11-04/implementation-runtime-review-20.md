# C11.1 one-axis Grid 구현·runtime 적대 검토 20개

대상은 현재 checkout의 bounded Root HTML one-axis placement 구현과 2026-11-04 Android/iOS simulator 실행이다. 계획 검토 20개를 재사용하지 않았으며, PR 변경 검토는 아직 별도 수행 전이다.

| # | 실패 관점 | 적대 확인과 판정 |
| --- | --- | --- |
| 1 | 제한 profile 밖의 public CSS 경로가 함께 바뀌는가 | `RuntimeRootHtmlTextPaintV1`에서만 owner selector와 전용 `GridPlacementMode`를 연결한다. generic `SparseRow`와 기존 explicit/dense/template mode의 selector는 유지 — 통과. |
| 2 | authored 선언 문자열이 아니라 현재 computed snapshot이 쓰이는가 | candidate는 `ComputedStyleSnapshot`의 typed Grid/item placement를 읽고 projection은 기존 document/root/revision 검증을 거친다. property 문자열 재파싱 경로 없음 — 통과. |
| 3 | sibling Grid owner의 visible item이 서로 섞이는가 | 후보 item을 owner별로 묶고 sibling row/column Grid 통합 fixture에서 각각의 좌표를 별도로 확인 — 통과. |
| 4 | `display:none` 자식이 axis 판정이나 occupancy를 바꾸는가 | runtime Grid node collection과 layout hidden-subtree map에서 제외하며, display-none child를 둔 Root HTML fixture가 나머지 세 item frame과 일치 — 통과. |
| 5 | `visibility:hidden` 자식이 occupancy에서 빠지는가 | hidden visibility는 in-flow item으로 유지한다. direct PixelCopy/Metal sample과 layout fixture가 hidden slot을 흰색으로 유지하고 뒤 item을 다음 row로 보냄 — 통과. |
| 6 | all-auto 기존 배치가 새 profile로 강제되는가 | selector는 definite numeric axis가 없는 owner를 선택하지 않는다. 별도 all-auto Root HTML control의 기존 1행 배치가 유지 — 통과. |
| 7 | 같은 Grid에서 row-locked와 column-locked item이 섞이는가 | owner별 axis agreement를 candidate admission에서 확인하고 layout validator도 재검증한다. mixed-axis projection/runtime 사례는 전체 scene rejection — 통과. |
| 8 | fully definite item 또는 fully definite/partial 혼합이 새 경로에 들어오는가 | selector의 axis-shape gate와 layout `locked_axis`가 양쪽 definite를 거부한다. 전용 layout negative fixture에서 fully explicit item rejection — 통과. |
| 9 | span만 있는 definite axis, 0/음수 span, named line/span이 배치되는가 | definite axis에는 numeric line이 적어도 하나 필요하고 모든 span은 양수여야 한다. named endpoints는 fail-closed — 통과. |
| 10 | 음수 line 또는 line/span 계산이 explicit grid 경계를 벗어나는가 | 기존 `resolve_axis`/checked track arithmetic을 재사용하고 negative explicit line scene을 검증한다. 범위 초과는 Grid 오류로 귀속 — 통과. |
| 11 | row-locked auto placement가 implicit row를 조용히 생성하는가 | explicit row 범위와 normalizer의 `expanded_rows` 결과를 확인한다. implicit extent가 생기면 결과를 반환하기 전에 거부 — 통과. |
| 12 | column-locked placement 또는 큰 span이 implicit column을 만드는가 | column span은 explicit column 개수 이하이어야 하고 explicit row item도 남는 column이 없으면 거부한다. 4 item/3 column negative case 통과 — 통과. |
| 13 | `grid-auto-flow` dense/column, template areas, named lines, repeat이 이 경로를 우회하는가 | candidate selector와 layout validator 양쪽에서 flow, areas, line metadata, repeat을 검사한다. 다른 profile에 fallback되어 부분 성공하지 않음 — 통과. |
| 14 | auto track sentinel이나 잘못된 track 값이 implicit sizing을 허용하는가 | template tracks는 finite non-negative px 목록으로 제한한다. auto rows/columns는 auto 또는 finite non-negative 단일 px만 받으며 implicit extent 자체는 항상 거부 — 통과. |
| 15 | text, replaced, nested, positioned, 비어 있지 않은 item의 intrinsic sizing이 섞이는가 | layout validator는 static empty non-replaced direct block, auto/fixed size만 허용하고 child Grid/text content/position/unsupported size를 거부한다 — 통과. |
| 16 | 컨테이너의 indefinite/비정상 크기, gap, edge, 정렬 값이 지원 범위를 넓히는가 | definite finite width/height, zero gap·margin·padding·border, 제한된 기본 alignment를 검사한다. 지원 밖 컨테이너 style은 property가 있는 오류로 거부 — 통과. |
| 17 | `order` 또는 positioned item이 source-order cursor·DOM paint 순서를 바꾸는가 | item order 0과 static positioning을 validator가 요구한다. normalizer는 기존 DOM child order를 사용하고 layout tree/paint order를 재배열하지 않는다 — 통과. |
| 18 | sibling 일부 성공 뒤 오류가 나면 부분 frame이나 stale snapshot이 publish되는가 | normalizer는 오류에서 `Result`를 반환하고 RuntimeSession negative fixture는 diagnostic, `completed=None`을 확인한다. candidate/baseline revision을 섞지 않음 — 통과. |
| 19 | Android PixelCopy가 다른 surface generation, viewport, coordinate, 기존 run output을 잘못 수용하는가 | opt-in adapter는 draw/surface size와 density를 확인하고 unique run ID·sample count·좌표·RGBA를 검증한다. stale generation은 capture callback supplier에서 거부하며 AVD readback 6/6 exact — 통과. |
| 20 | fixture 전용 진단·class·payload·FFI가 기본 앱에 새어 나오거나 iOS 표본을 compositor 이후 결과로 과장하는가 | Android 기본 APK에서 one-axis marker/key/class/service/payload와 dedicated FFI symbol이 없고 opt-in provider는 APK에만 존재한다. iOS 기본 app에서도 marker/payload/symbol이 없으며 Metal 6/6은 compositor 제출 전 drawable로 구분 — 통과. |

### 발견 및 수정

초기 기본 APK 검사에서 C11 전용 class-name과 진단 문자열이 `C0410RuntimeGpuDemo`/`MainActivity` 기본 DEX에 남는 것을 발견했다. 이를 기본 runtime의 generic `ServiceLoader` 확장 지점과 opt-in source/resource set으로 분리했다. 기본 APK 재빌드 후 C11 전용 문자열·provider·adapter class·fixture payload가 빠졌고, opt-in APK service registration을 통해 PixelCopy 6/6을 재실행해 확인했다.

### 실행 한계

Android는 API 37 ARM64 AVD의 ANGLE/Vulkan/SwiftShader software renderer이며 실기기·hardware GPU 검증이 아니다. iOS는 iPhone 17 Pro/iOS 26.2 Simulator이며 compositor 제출 후 화면은 검증하지 않았다. 이 검토는 별도로 수행할 PR diff 검토 20개를 대신하지 않는다.
