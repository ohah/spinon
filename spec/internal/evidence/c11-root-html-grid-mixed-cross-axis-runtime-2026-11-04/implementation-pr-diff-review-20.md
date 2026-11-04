# C11.1 mixed cross-axis 구현 diff 실패 관점 검토 20개

대상: mixed cross-axis 구현을 위해 실제 변경한 Rust·Android·iOS 소스. 범위 의미·런타임 실행 결과 검토인 [implementation/runtime review](./implementation-runtime-review-20.md)와 분리해 코드 diff의 연결·호환성·경계만 검토했다.

| # | diff 검토 관점 | 판정 |
|---:|---|---|
| 1 | 새 mode가 별도 enum 값으로 추가되어 기존 one-axis/mixed-explicit 값의 의미를 바꾸지 않는가 | **PASS.** 새 `RootHtmlMixedCrossAxisAutoPlacementV1` variant만 추가했다. |
| 2 | mode가 grid validator의 허용 목록에서 빠져 일부 container gate만 통과하는가 | **PASS.** 변경된 supported-mode matches에 새 variant를 등록했다. |
| 3 | 새 mode의 item width/height에서 `Auto`가 빠지거나 기존 허용 범위가 넓어지는가 | **PASS.** 새 mode만 Auto/Fixed를 받고 기존 mode 분기는 유지된다. |
| 4 | normalizer admission은 추가됐지만 item validation은 이전 one-axis 의미로만 작동하는가 | **PASS.** validator에 cross-axis flag를 분리 전달한다. |
| 5 | placement mode가 배치 전에 검증되지 않아 malformed input이 Taffy로 유입되는가 | **PASS.** 기존 sparse normalization 전 `validate_items` 경로에서 owner와 items를 확인한다. |
| 6 | mixed cross-axis 경로가 공개 FFI/API로 새어 나오는가 | **PASS.** 이 diff는 layout enum과 기존 style projection 안에 제한 mode를 추가하며 새 public FFI entry point를 만들지 않는다. |
| 7 | default root HTML profile 이외의 snapshot profile이 새 selector를 실행하는가 | **PASS.** projection route 선택은 `RuntimeRootHtmlTextPaintV1`에 한정된다. |
| 8 | selector가 다른 owner의 item을 현재 Grid 집합에 포함하는가 | **PASS.** `grid_item_owners`를 owner별 map으로 분리한다. |
| 9 | dense owner가 새 route와 dense route 양쪽에 동시에 들어가는가 | **PASS.** dense route node는 새 selector 시작에서 제외한다. |
| 10 | areas, auto-repeat, repeat group 또는 line-name metadata가 numeric path를 우회하는가 | **PASS.** selector와 layout validation의 rejection guard가 추가/유지됐다. |
| 11 | fully-auto placement가 기존 one-axis mode에 새로 허용되는가 | **PASS.** fully-auto 분기는 cross-axis mode에서만 수행하고 이전 one-axis mode에는 영향이 없다. |
| 12 | fully-definite item을 mixed-explicit admission 변수와 혼동하는가 | **PASS.** cross-axis validator는 fully-definite를 공통 분류하되 row/column partial counts를 별도로 요구한다. |
| 13 | row/column partial count가 overflow할 수 있는가 | **PASS.** layout validator의 `checked_add` 경로를 사용한다. selector count는 유한 snapshot item 목록 길이만큼 증가한다. |
| 14 | 완전한 mixed cross-axis owner가 아닌 경우도 새 mode가 선택되는가 | **PASS.** projection selector와 layout validator 모두 row 및 column lock이 각각 최소 1개인지 확인한다. |
| 15 | numeric span이 0/음수이거나 named span이 포함된 상태로 layout에 도달하는가 | **PASS.** mode-specific endpoint validation은 positive numeric span과 Auto/Line만 받는다. |
| 16 | row와 column axis 혼합을 허용하면서 row/column endpoint 조합 자체를 느슨하게 받는가 | **PASS.** 한 축의 두 endpoint가 auto 또는 numeric definite이고, 다른 축은 반대 형태인 조합만 통과한다. |
| 17 | 새 mode가 별도 placement 구현을 복사해 기존 sparse 알고리즘과 달라지는가 | **PASS.** `normalize_sparse_grid_container` shared path를 재사용한다. |
| 18 | auto row/column track 입력이 implicit track sizing 지원으로 확대되는가 | **PASS.** 기존 auto-track validator는 implicit extent가 필요한 track list를 거부한다. |
| 19 | Android PixelCopy adapter가 일반 app source set이나 기본 runtime에 포함되는가 | **PASS.** 소스는 opt-in `c11_grid_metal_surface_pixels_fixture_test` source set에 있다. |
| 20 | iOS marker/argument parser가 기본 build에 노출되거나 다른 run을 소비하는가 | **PASS after correction.** 첫 flag-off executable scan에서 scene ID helper의 prefix 하나가 guard 밖에 남은 것을 찾아 `#if SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST_1` 안으로 옮겼다. 최종 default executable scan은 marker 부재, opt-in scan/runtime은 run ID와 26/26 summary를 확인한다. |

## 결과

20개의 독립 source-diff 관점을 검토했다. iOS default artifact scan에서 발견한 guard 누출을 수정하고 default/opt-in artifact 경계를 재검증했다. Android flag-off APK DEX와 iOS flag-off Simulator executable에는 mixed-cross-axis test marker가 없고, opt-in DEX/executable 및 simulator runtime에는 대응 marker가 있다. PR 최종 diff의 별도 30-point review는 PR 생성 후 해당 PR 번호에 맞춰 기록한다.
