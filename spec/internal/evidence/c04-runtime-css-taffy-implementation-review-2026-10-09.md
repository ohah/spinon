# C04.9 · Runtime CSS→Taffy 구현 후 적대 검토

**검토일:** 2026-10-09 · **대상:** C04.9 Rust·FFI·Android/iOS probe 구현 및 내부 계약 · **비교 모델:** 고정 Chromium fixture · **결과:** 발견 항목 수정 뒤 재검토 완료

아래 관점은 구현의 서로 다른 실패 경계다. 계획 검토표를 재사용하지 않았다. 수정 후 영향을 받은 관점은 테스트와 플랫폼 probe를 다시 실행했다.

| # | 공격 관점 | 확인 결과와 근거 |
|---:|---|---|
| 1 | 기존 C04.8 JSON 소비자에게 layout 속성이 새로 노출되는가 | 기존 7개 UA property serializer가 superset snapshot에서 whitelist만 꺼낸다. 기존 C ABI tests와 Android/iOS probe의 ua_values PASS를 확인했다. |
| 2 | layout 입력을 위해 DOM cascade를 두 번 돌리는가 | 한 번의 RuntimeFlexLayout cascade에서 UA snapshot과 computed style을 만들고, 같은 style snapshot을 Taffy projection에 전달한다. compute_request와 compute_root 경로를 확인했다. |
| 3 | 앱 root가 document element로 취급되어 :root blockification이 바뀌는가 | 기존 fragment-child Stylo view 생성자를 유지하고, RuntimeSession 실제 실행에서 UA 값 회귀가 없다. |
| 4 | viewport가 HostRoot 요소의 강제 크기가 되어 CSS 300px root가 viewport 800px로 변하는가 | 기존 입력 adapter는 보존하고, runtime 전용 containing-block 정책을 추가했다. Chromium fixture의 root 300×140과 실제 플랫폼 검증을 확인했다. |
| 5 | HostRoot 직속 요소가 여러 개일 때 일부 프레임만 게시되는가 | cascade roots는 기존처럼 각각 계산하지만 layout은 multiple_host_roots로 실패하고 completed frame을 만들지 않는다. all_host_root_elements_are_computed_as_independent_roots test. |
| 6 | 빈 HostRoot가 오류나 가짜 viewport frame으로 보고되는가 | 요청 전 not_configured와 계산된 빈 트리의 empty를 나눴다. no_environment_means_no_implicit_desktop_calculation test. |
| 7 | 직속 text root가 element layout으로 오인되거나 앞선 root 부분 결과가 남는가 | 직접 text root는 cascade 요청 전체를 실패 처리한다. direct_text_root_fails_the_whole_request_instead_of_returning_partial_roots test. |
| 8 | element subtree의 text를 조용히 버리고 layout 성공으로 처리하는가 | text node는 unsupported_text_node이며 computed-style 결과만 유지한다. text_inside_element_fails_layout_without_discarding_computed_style test. |
| 9 | NodeId 정렬이 DOM 순서를 깨뜨리는가 | 역순 ID fixture에서 frame 배열이 DOM preorder를 따른다. runtime_layout_frames_follow_dom_order_not_node_id_order test와 Android/iOS probe. |
| 10 | display:none 자손이 Taffy 기본 frame을 노출하는가 | hidden node와 자손을 0 frame으로 변환한다. 고정 Chromium frame, runtime fixture 및 플랫폼 probe에서 확인했다. |
| 11 | 유효하지만 미지원 property나 custom property가 무시되어 false success가 되는가 | expanded declaration allowlist 밖은 unsupported_inline_property로 layout만 실패한다. color와 custom-property allowlist tests. |
| 12 | 잘못된 CSS declaration 하나가 유효한 형제 선언과 전체 layout을 날리는가 | Stylo error recovery 진단을 보존하고 유효 declaration layout을 유지한다. invalid declaration recovery와 recoverable_inline_parse_diagnostics tests. |
| 13 | flex shorthand allowlist와 계약 목록이 달라 유효 shorthand가 거부되는가 | RuntimeFlexLayoutV1 전용 코드가 Stylo expanded longhand를 보므로 flex: 0 1 auto를 통과시킨다. 계획·0030에 shorthand 경계를 추가하고 Chromium longhand/geometry test를 실행했다. 기존 C04.2 FlexLayoutV1 allowlist는 바꾸지 않았다. |
| 14 | flex: 1이 percentage flex-basis를 지원값으로 오인하는가 | computed flex-basis가 percentage serialization이면 default로 덮지 않고 unsupported_computed_value로 실패한다. runtime_flex_shorthand_with_percentage_basis_fails_closed test. |
| 15 | px 외 단위, auto margin, 음수 padding이 0 또는 다른 값으로 변환되는가 | projector는 허용 serialization만 parse하고 나머지는 computed property 오류를 돌려준다. 기존 css_values_outside_the_flex_projection_fail_closed 및 margin/padding profile tests. |
| 16 | NaN·infinite 좌표나 음수 크기가 JSON completed frame으로 노출되는가 | runtime_frames 검증을 단위 테스트로 추가해 비유한 x/y와 음수 width/height를 거부한다. runtime_frames_reject_non_finite_coordinates_and_negative_sizes test. |
| 17 | Taffy frame 수와 computed element 수, snapshot node set이 어긋나도 부분 JSON이 나오는가 | frame set 일치와 각 DOM node frame 존재를 확인하고 mismatch/missing/invalid로 전체 layout을 거부한다. runtime_frames 구현 및 HostDocument projection의 node-set failure tests. |
| 18 | 환경·문서 변경 뒤 느린 예전 worker 계산이 최신 결과를 덮는가 | full revision key가 다르면 publish하지 않고 새 pending 상태를 유지한다. stale_completion_cannot_replace_a_newer_requested_key test. |
| 19 | C ABI 버퍼 부족·alias·session 해제 경합에서 부분 JSON이나 잘못된 메모리 사용이 생기는가 | 1-byte probe 후 required capacity로 재시도하고 NUL·-3 계약을 실행한다. output/required pointer 비중첩과 free 비동시 조건을 Safety 문서·header·0030 계약에 명시했다. |
| 20 | worker panic 또는 플랫폼 연결 실패가 성공 상태로 남는가 | panic은 catch되어 worker_failed로 전이한다. Rust test와 Android/iOS 실제 V8 앱에서 status 0, schema, 5 frame, Chromium geometry PASS를 확인했다. |

## 검토 중 발견해 수정한 점

- 구현 allowlist는 flex shorthand를 Stylo가 확장한 longhand로 허용했지만 계획에는 shorthand가 빠져 있었다. 계약을 명시하고 기본 shorthand와 percentage basis의 반대 사례를 추가했다.
- HostDocument layout 입력의 root 기본 크기 정책이 viewport를 root 크기로 강제해 CSS 크기를 따르는 runtime 계약과 맞지 않았다. runtime 전용 containing-block 경로를 분리해 fixture로 확인했다.
- 새 root-sizing 필드 추가 뒤 기존 LayoutInput test helper 두 곳이 빠져 전체 test compile이 실패했다. 두 helper를 legacy MatchViewport 정책으로 명시했다.
- projection의 모든 LayoutStyle 필드가 지정된 뒤 남은 struct update와 테스트의 len > 0 표현이 Clippy 경고를 냈다. 코드를 단순화하고 Clippy를 다시 통과시켰다.
- 출력 buffer와 required-capacity pointer alias 조건이 FFI Safety 설명에 없었다. C header와 내부 계약에도 비중첩 조건을 추가했다.
- runtime frame finite/nonnegative 조건이 구현에는 있었지만 직접 단위 테스트가 없었다. 경계 test를 추가했다.
- 새 Taffy 경로를 위한 dependency/import를 과거 C04.2 Flex profile의 allowlist에도 적용할 뻔했다. 적대 검토에서 범위 확대를 발견해 C04.2 계산과 지원 목록을 원상 복구했고, `RuntimeFlexLayoutV1` 전용 allowlist로 경계를 한정했다. 기존 C04.2 테스트를 다시 통과시켰다.
- C04.9가 커지며 기존 Rust 파일을 500줄 넘게 늘리는 변경이 생겼다. 계산 orchestration, runtime style profile, computed value parsing을 각 책임별 모듈로 분리했다. 분리 뒤 test compile에서 빠진 `CssCascadeError` 및 테스트용 타입 import를 고쳤고, 전체 workspace test·Clippy를 다시 통과시켰다.
- runtime cascade 기존 테스트 파일이 새 경로 추가 뒤 519줄이 된 점을 발견했다. 레이아웃 전용 fixture와 실패 경계 test를 별도 `runtime_layout_tests.rs`로 옮겨 두 파일 모두 500줄 이하로 나눴고, 같은 test case를 전체 workspace에서 다시 통과시켰다.
- 변경 대상 `spinon-layout/src/tests.rs`는 기존 엔진 비교 FFI fixture와 입력 검증을 한 파일에 두어 691줄이었다. 기존 엔진 oracle과 invalid-input 시나리오를 책임별 하위 모듈로 분리해 중심 파일을 482줄로 줄였고, 분리 후 Rust test·Clippy를 다시 통과시켰다.
- 최종 재실행에서 Android와 iOS 로그·화면을 새로 수집했다. 둘 다 V8 probe status 0, 5 frames, Chromium geometry PASS이며 화면은 diagnostic UI다. 원본 checksum은 simulator evidence에 기록했다.
- 공식 내부 명세 인덱스에서 0030 행이 표 밖에 놓이고 0027 행이 빠진 점을 발견했다. 새 계약을 번호 순서의 표 안에 넣고 0027 참조도 복원했다.

## 남은 범위

최종 변경의 Rust 252개 test, JavaScript 2개, CSS reference 19개, Android touch analyzer/capture 29개 test와 warnings-as-errors Clippy가 통과했다. 이번 검토는 단일-root element-only 제한 profile을 대상으로 한다. 전체 CSS, 여러 layout root 배치, 텍스트 shaping, author stylesheet, CSSOM, GPU paint, 화면 presentation, 실기기 및 성능 검증은 포함하지 않는다.
