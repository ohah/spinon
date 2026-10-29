# C11.4 auto-repeat fixed maximum 구현/runtime 실패 관점 20개

2026-10-29 최종 tracked source diff와 Android/iOS 49-scene 실행을 대상으로 확인했다. 계획 실패 검토는 재사용하지 않았다. 검토 중 수정한 matrix count와 iOS pixel feature 설정은 마지막 캡처 전에 다시 확인했다.

| # | 별도 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | Chrome oracle와 runtime scene ID가 하나라도 빠지거나 추가되는가? | FFI catalog test는 두 ID set이 각각 49개이며 exact equality임을 확인한다. |
| 2 | 세 inflexible minimum 중 하나가 cascade에서 누락되는가? | `auto`, `min-content`, `max-content` × 세 maximum breadth의 36 조합을 Stylo track-sizing test가 검사한다. |
| 3 | fixed-maximum 분기가 잘못된 maximum을 허용하는가? | intrinsic-min branch는 `LengthPx`, `Percentage`, 제한 `Calc`만 통과시키며 `Fraction`, `Auto`, intrinsic maxima를 거부한다. |
| 4 | negative/NaN/infinite px 또는 percentage가 repeat admission을 통과하는가? | Stylo와 layout validator에서 non-finite 및 음수 값을 거부하는 경계를 확인했다. |
| 5 | percentage maximum이 indefinite axis basis에서 계산되는가? | layout preflight는 definite positive content basis와 유한 결과가 없으면 percentage maximum을 거부한다. |
| 6 | Columns/Rows percentage가 서로의 basis를 참조하는가? | 49 oracle의 column/row 입력과 양 platform root-frame matcher에서 axis별 basis가 일치한다. |
| 7 | `calc()` ID가 다른 node/property에 속한 기록을 빌려 쓰는가? | 기존 `auto_repeat_maximum_calc_is_supported`의 node, grid property, typed expression registry 검증을 fixed maximum 경로가 사용한다. |
| 8 | 임의 calc AST나 `calc()` 최소값까지 새로 허용하는가? | 추가한 branch는 calc maximum만 노출한다. parser/projection test와 기존 exact `length + percentage` owner 검사가 계속 범위를 제한한다. |
| 9 | 0px fixed maximum repeat-count fallback이 맞는가? | low-level validator는 유한한 0도 허용하지만 49-scene oracle은 양수 maximum만 포함한다. 따라서 zero-maximum behavior는 이 검증에서 보장하지 않고 후속 경계로 남겼다. |
| 10 | 최대값보다 큰 intrinsic minimum/item이 fixed max에 잘못 잘리는가? | axis당 overflow 대조군에서 20px item과 10px max의 used track/item overflow가 Chrome expectation에 일치한다. |
| 11 | `auto-fill`의 빈 반복 track이 남는가? | 36 matrix scene의 exact tracks/collapse 비교에서 `auto-fill`은 지정 반복 수를 유지한다. |
| 12 | `auto-fit`이 빈 track/gutter를 접지 않는가? | 36 matrix scene에서 사용 track의 collapse flag와 최종 track list를 oracle과 비교한다. |
| 13 | 129px/130px 경계에서 반복 횟수가 한 칸 빨리 바뀌는가? | Columns/Rows의 40px 최대값과 5px gap 경계 4 runtime scene이 각각 예상 2/3회 반복을 확인한다. |
| 14 | 두 track 반복 조각에서 내부 gap을 count 산식에서 빼먹는가? | 양축 × 두 repeat mode의 4 scene은 전체 pattern breadth와 internal gap을 포함한 135px 기준을 확인한다. |
| 15 | Taffy `MaximizeTracks`/intrinsic contribution trace가 실제 layout 결과와 어긋나는가? | Rust 49-case layout test와 두 앱의 per-scene track trace 검사가 Chrome 고정 expectation을 통과했다. |
| 16 | Stylo computed minimum이 style-to-layout projection 중 `Auto`/intrinsic keyword로 보존되지 않는가? | projection focused test에서 각 최소 종류와 px/%/calc 최대값 typed 조합을 검사한다. |
| 17 | fixture adapter가 exact scene만 구성하고 입력 CSS/tree를 섞지 않는가? | fixture source는 JSON의 CSS/box/items를 읽고 등록된 49개 ID만 반환하며 unknown ID는 거부한다. |
| 18 | 기본 제품 FFI archive에 새 fixture symbol/payload가 포함되는가? | default archive에서 신규 marker 부재, fixture-test archive positive control에서 세 marker 존재를 확인했다. |
| 19 | Android/iOS harness allowlist 또는 sample matcher가 서로 다른 scene을 인정하는가? | JSON, Android ID 배열, iOS ID set을 대조해 모두 같은 49개 집합이며 각 simulator가 전 scene을 실행했다. |
| 20 | pixel 좌표·scene 수 변경 때문에 부분 검증이 전부 통과한 것처럼 보이는가? | 일반 43×2와 overflow 6×3의 104 표본을 각 플랫폼에서 exact 확인했다. 오래된 `545` 기대값을 `594`로 고친 뒤 boundary test 21/21과 최종 49-scene capture를 실행했다. iOS pixel feature define 및 Android 설치 APK SHA도 최종 결과에 맞춰 확인했다. |

**결과:** 이 20개 관점에서 미해결 차단 결함을 남기지 않았다. 최종 runtime 근거는 [README](./README.md), Android/iOS per-scene 로그와 metadata는 그 문서에 연결한 run directory에 있다. AVD software renderer, iOS compositor 이전 readback, 전체 C11.4 및 일반 CSS 지원은 이 검토 범위 밖이다.
