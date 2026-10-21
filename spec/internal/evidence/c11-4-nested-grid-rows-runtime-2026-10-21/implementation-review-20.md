# C11.4 nested Grid 행 축 구현/runtime 실패 관점 검토 20개

계획 검토와 PR diff 검토에서 사용한 항목을 재사용하지 않고, runtime/layout 결과를 깨뜨릴 수 있는 방향을 각각 대조했다. 재검토 범위는 row-axis adapter, `Grid` indefinite-height validator, scene runner, Android/iOS harness와 세 고정 simulator 실행이다.

| # | 실패 가능성 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | Chrome 관측값을 규범 oracle로 잘못 승격 | CSS Grid 2025 CRD §6.6, §11.1, §11.5.1 계산을 별도로 유지하고 Chrome 156은 교차 관찰로만 기록했다. 통과. |
| 2 | spanning contribution에 outer row gap을 중복 포함 | span-two의 40px contribution에서 outer 4px gap을 빼 36px를 `[18,18]`로 분배한다. 두 simulator trace/track이 일치한다. 통과. |
| 3 | 한 track item의 contribution 뒤 root 높이에서 row gap 누락 | one-track 결과 `[40,0]`과 root height 44px를 frame·track 모두 검사했다. 통과. |
| 4 | definite-flex automatic minimum을 min-content로 잘못 사용 | control trace에서 nested item의 `Minimum` contribution 0px, rows `[28,28]`, root/nested 60px를 확인했다. 통과. |
| 5 | 높이가 indefinite인 그리드를 제한 없이 받아 기존 mode를 우회 | 허용 분기는 `TrackSizing` mode뿐이며 다른 mode는 기존 definite-height gate를 유지한다. 통과. |
| 6 | width도 indefinite인 track Grid를 허용 | 높이만 예외 처리하고 width의 definite 조건을 계속 강제한다. 통과. |
| 7 | 명시 `height`, `min-height`, `max-height` 제한을 넘겨 sizing | helper는 세 값 모두 `auto`인 경우에만 통과시킨다. 다른 style은 `IndefiniteGridBasis` 경로로 남는다. 통과. |
| 8 | indefinite-height percentage row를 임의 수치로 계산 | track validator가 `Percentage`를 거부하며 focused 회귀가 height basis 오류를 확인한다. 통과. |
| 9 | indefinite-height flex, fit-content 또는 flex `minmax()`를 허용 | 새 layout regression이 `Fraction`, `FitContent`와 flexible `minmax`를 각각 거부하는 3개 control을 확인한다. 통과. |
| 10 | `calc()` track이나 auto-repeat row가 예외를 우회 | `Calc`/`FitContent`/`Fraction`은 whitelist 밖이며 row auto-repeat은 helper의 명시 guard에 걸린다. validator의 기존 auto-repeat 검증도 유지된다. 통과. |
| 11 | `grid-auto-rows` implicit track 목록이 unsafe track을 추가 | auto rows는 `Auto`, px 또는 explicit row와 같은 안전 track subset만 허용하는지 확인했다. 통과. |
| 12 | columns sizing 결과 없이 inner row intrinsic size를 계산 | fixed 98px inline basis에서 text/image column과 inner rows `[20,16]`을 분리해 trace에 확인한다. 통과. |
| 13 | text run, Ahem font 또는 wrapping 변화가 contribution을 바꿈 | exact `XX XXXXXXX` leaf와 Ahem 10px/line-height 10px style을 검증하고 text frame 70×20을 양 platform에서 확인한다. glyph paint는 범위 밖으로 남긴다. 통과. |
| 14 | PNG source, decode 오류 또는 natural-size drift를 숨김 | direct image leaf/source, fixture PNG decode와 24×16 decoded dimensions를 확인하고 준비 실패를 error로 반환한다. 통과. |
| 15 | 추가/누락 child, 잘못된 owner 또는 다른 순서의 노드를 수용 | root→nested→text/image의 정확한 child count, parent, namespace, tag, ID 및 text/image leaf shape를 검사한다. 통과. |
| 16 | profile, dimensions, gaps, alignment 또는 track style의 변형을 부분 지원으로 처리 | computed style profile과 고정 outer/nested grid style, box sizing, row/column gap, min/max, auto-flow, alignment, line names, repeat를 비교하고 불일치 시 reject한다. 통과. |
| 17 | row/column line 또는 span 끝을 혼동 | nested Grid의 고정 row placement가 scene별 `1 / span 2` 또는 `1 / 2`, children 위치가 정확한 row/column line임을 검사한다. 통과. |
| 18 | contribution의 axis, item, kind 또는 span을 잘못 추적 | runtime test는 outer Rows trace에서 nested NodeId, contribution kind/value와 span을 확인하고 inner row tracks와 네 frame을 검사한다. 통과. |
| 19 | 앱 harness가 Rust fixture 기대값과 갈라짐 | Android/iOS에 동일 3 IDs, root frames, 4 boxes, node frame 배열을 등록했다. API 37 AVD와 iPhone 17 Pro에서 각각 3/3, trace 및 draw/present 통과. 통과. |
| 20 | 테스트 fixture가 기본 제품 archive에 섞임 | default static archive에서 세 새 scene ID가 없고 opt-in archive에서 IDs 및 PNG payload marker가 있는 positive control을 확인했다. 통과. |

추가 확인: 첫 Android launch가 신규 ID 미등록으로 거부된 것을 재현했고 양쪽 앱 allowlist/frame 검사 변경 후 재빌드했다. 이 보고서는 box layout·trace·draw/present 검토다. 화면은 glyph paint나 PNG texel·surface pixel을 증명하지 않으며 물리기기, WPT, Android hardware GPU, iOS compositor-after 픽셀도 검증하지 않았다. 상세 입력·hash·로그·화면은 [runtime README](README.md)에 있다.
