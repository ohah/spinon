# C13.1 runtime hit-test 회귀 delta 실패 관점 20개 검토 · 2026-10-14

검토 대상은 agent-browser Chrome 155 `document.elementFromPoint()` 표본과 Rust `scene.hit_test_css_point()`를 잇는 추가 회귀다. 원본 browser 좌표는 [Chrome observation JSON](c13-1-overflow-clip-android-avd-2026-10-14/chrome155-runtime-observation.json)의 `hitTestSamples`이며, CSS node·parent가 공유 JS fixture와 맞는지도 unit test에서 대조한다. 이 검토는 DOM event 전달, 전체 hit-test 적합성 또는 C13.1 완료를 뜻하지 않는다.

| # | 실패 관점 | 적대적 확인과 결과 |
| --- | --- | --- |
| 1 | HTML과 JS fixture가 바뀌어 테스트 oracle이 stale | HTML/JS 고정 SHA와 Chrome observation을 보존한다. 현재 JS source SHA가 기록값과 같다. |
| 2 | Rust test가 다른 viewport 크기로 계산 | helper에 Chrome CSS viewport `320×120`을 전달한다. agent-browser 활성 fixture도 `320×120`이다. |
| 3 | root node 순서가 바뀌어 NodeId index가 틀림 | 13개 fixture ID를 순서대로 만들고 각 sample의 기대 index를 비교한다. |
| 4 | parent 관계가 browser fixture와 다르게 하드코딩 | Rust parent index를 JS 변수명으로 바꾸어 각 fixture call의 parent와 대조한다. |
| 5 | declaration이 바뀌었는데 Rust 복사본이 통과 | 각 ID·parent·전체 inline declaration tuple이 공유 JS source에 들어 있는지 확인한다. |
| 6 | body/document 아래 root wrapping 차이가 좌표를 이동 | root는 fixture와 같이 host root에 직접 배치하며 11 query의 기대 NodeId를 검사한다. |
| 7 | root 자체의 크기가 줄어 fallback hit가 달라짐 | source guard가 `300×100` border-box root 선언을 고정하고 (295,76) fallback을 확인한다. |
| 8 | visible overflow가 paint clip으로 잘림 | (102,20)은 visible child overflow의 leaf `visible-target`을 반환한다. |
| 9 | clip 안쪽의 양성 hit가 모두 거부 | (180,20)은 `clip-target`을 반환한다. |
| 10 | clip 바깥 descendant가 계속 hit됨 | (198,20)은 clipped child 대신 root를 반환한다. |
| 11 | padding-box clip margin 확장이 hit 경계에 반영되지 않음 | 확장 영역 (291,20)은 `margin-target`을 반환한다. |
| 12 | clip margin 밖까지 target hit가 번짐 | (297,20)은 margin target 대신 root를 반환한다. |
| 13 | nested x clip 안의 descendant를 누락 | (84,76)은 nested leaf를 반환한다. |
| 14 | nested x 경계 밖에서 outer box fallback이 없음 | (95,76)은 nested leaf가 아니라 `nested-outer`를 반환한다. |
| 15 | `overflow-y:visible` 축을 x clip과 함께 자름 | (45,92)는 y 방향으로 확장된 nested leaf를 반환한다. |
| 16 | nested outer y clip 바깥에서도 leaf를 반환 | (45,97)은 root로 fallback한다. |
| 17 | static clip이 positioned containing block처럼 absolute child를 자름 | (230,76)은 outer clip 바깥 positioned `escape-target`을 반환한다. |
| 18 | outer clip 바깥으로 나온 absolute child가 잘못 노출 | (295,76)은 escape child가 아니라 root를 반환해 ancestor paint/hit boundary를 보존한다. |
| 19 | Chrome에서 저장한 좌표나 target ID가 현재 관찰과 드리프트 | agent-browser로 동일 fixture를 다시 열어 320×120 viewport에서 11개 `elementFromPoint()` 결과를 재확인했고 JSON과 일치했다. |
| 20 | box query 통과를 DOM event 전달 성공으로 과장 | 테스트는 runtime query 반환 NodeId만 비교한다. DOM dispatch, listener 호출, 앱 입력 경로는 검증 범위 밖이라고 문서에 분리했다. |

검증 실행: `cargo test --locked -p spinon-style-to-render runtime_hit_test_matches_agent_browser_chrome_155_element_from_point_samples` 통과(1/1), `cargo fmt --all -- --check` 통과. `document.elementFromPoint()` 관찰은 agent-browser로 수행했고 terminal-browser와 실기기는 사용하지 않았다. Android API 37 ARM64 AVD는 기존 paint/layout 실행 근거에만 포함되며 이 unit test는 host Rust에서 실행했다.
