# C10.3.5 구현 실패 경로 검토와 실행 결과

**검토일:** 2026-10-11 · **기준 Chrome:** 154.0.8037.98 · **내부 계약:** 0056 / `0.1.0`

계산 트리, Stylo typed projection, scene paint 순서, FFI/Android/iOS 진입점, pinned reference 비교기와 최종 표면 증거를 독립 실패 경로별로 다시 확인했다.

| # | 독립 실패 경로 | 대조 내용과 판정 |
| ---: | --- | --- |
| 1 | layout projection을 위해 HostDocument source parent/order를 바꾸는가 | layout helper는 source child map에서 parent 관계를 읽고 계산 전용 adjustment만 반환한다. paint test는 `snapshot.children()`이 전후 동일함을 확인한다. |
| 2 | 모든 absolute node에 containing-block owner의 Block static position을 적용하는가 | absolute child의 source parent가 Flex box인지 별도로 검사한다. Flex containing block만 있는 Block wrapper 후손은 Flex helper로 들어가지 않는다. |
| 3 | Flex child를 line/gap/flex sizing에 참여시켜 sibling frame을 움직이는가 | Flex static-position adjustment는 계산 결과 frame에만 적용한다. fixture 결과에서 in-flow frames와 owner가 Chrome 기준과 일치하고 flow 불변 테스트가 통과한다. |
| 4 | parent가 정한 `align-items`를 child `align-self`가 이길 때 잘못된 값을 쓰는가 | `auto`는 parent 값을 따르고 child의 `center/start/end/stretch` typed profile을 분기한다. baseline은 구체 오류다. |
| 5 | row/column 및 reverse 축에서 main/cross 축을 뒤집는가 | row, row-reverse, column, column-reverse를 분리해 계산한다. `wrap-reverse` 교차축과 pinned Chrome 20-case 비교도 포함한다. |
| 6 | definite inset 축에 static-position adjustment를 다시 더하는가 | 각 축의 양 inset이 모두 `auto`일 때만 adjustment를 만들고 혼합 inset test에서 기존 C12.2 해법을 유지한다. |
| 7 | Flex child width/height를 auto·intrinsic size로 임의 추정하는가 | fixed-size parent/child만 성공하고 나머지는 NodeId가 있는 `UnsupportedPositioning`으로 종료한다. 실패 시 부분 output이 없다. |
| 8 | padding/border edge를 content box 시작으로 사용하거나 두 번 더하는가 | content origin을 border+padding으로 구하고 frame 외곽에서 border/padding을 빼 내부 크기를 산출한다. pinned Chrome node별 frame으로 비교한다. |
| 9 | signed/auto margin을 Flex free space에 잘못 계산하는가 | fixed px margin은 부호를 보존하고 auto는 static-position 계산에서 0으로 취급한다. 비율 margin은 성공값으로 추정하지 않고 거부한다. |
| 10 | 음수 여유 공간이나 `space-around/evenly` 단일 child offset을 일반 Flex 규칙으로 추정하는가 | 고정 Chrome 관측을 전용 helper 분기로 고정한다. 20-case 기준에는 overflow와 분배 정렬 case가 들어 있다. |
| 11 | Flex owner가 실제 containing block보다 안쪽일 때 좌표계가 섞이는가 | source Flex frame에서 static position을 구하고 absolute containing-block frame 차이와 별도 처리한다. 다른 owner fixture에서 결과 frame과 owner를 함께 검사한다. |
| 12 | Block wrapper 후손이 ancestor Flex의 `justify-content`/`align-self`를 상속하는가 | wrapper는 C12.2 Block static-position 경로를 유지한다. 혼합 definite inset case는 C12.2 owner math와 함께 회귀 검사한다. |
| 13 | `display:none` ancestor 아래 노드를 viewport에 되살리거나 source에서 제거하는가 | layout/paint node set과 source identity 검증에서 hidden node를 유지하되 scene box에서 제외한다. |
| 14 | CSS position typed 값이 legacy profile까지 열리는가 | `parse_positioning` 변경은 Block Positioning 또는 여섯 runtime Flex profile에 한정된다. adapter/profile 테스트가 나머지 경계를 유지한다. |
| 15 | 여섯 profile 중 한 profile만 typed position projection을 받는가 | `direct_absolute_flex_child_is_supported_by_all_six_runtime_flex_profiles`가 여섯 profile을 개별 검사한다. |
| 16 | Chrome reference 수정/수집이 현재 fixture나 capture 도구와 분리되는가 | 20 case·70 node reference는 고정 Chrome 실행 파일, fixture/tool digest와 WPT revision을 검사한다. reference 자동 overwrite는 허용하지 않는다. |
| 17 | render scene이 Flex `order`를 source tree 조회와 합치거나 absolute authored order를 적용하는가 | source child vector를 보존하고 in-flow child만 안정 `order` 정렬, positioned child는 source preorder로 별도 paint phase에 추가한다. 겹치는 node의 top-to-bottom 순서를 Chrome probe와 비교한다. |
| 18 | 중첩 positioned descendant가 누락되거나 두 번 제출되는가 | 각 absolute subtree에서 nested absolute child를 분리한 뒤 positioned preorder에 한 번 추가한다. nested fixture는 scene ID uniqueness와 순서를 검사한다. |
| 19 | runtime 비교기가 로그 중복, root 차원, 누락 NodeId를 무시하는가 | comparator는 Android/iOS backend 표식, `presented boxes=16`, root `320×240`, 16개 연속 NodeId, fixture source 순서와 각 rect를 확인한다. geometry 0.75px 변조 negative control은 허용치 초과로 거부했다. |
| 20 | simulator/실기기 첫 layout만 맞고 실제 GPU surface 표시 또는 backend가 다른가 | Android SM-S731N에서 Vulkan/`presented boxes=16`, iPhone 17 Pro iOS 26.2 Simulator에서 Metal/main-thread surface/`presented boxes=16`을 확인했다. 앱 화면·로그·15 node 비교 결과를 별도 저장했다. |

## 검토 중 찾아 수정한 부분

- 첫 Android runtime fixture가 `background` shorthand를 그대로 전달해 runtime paint profile의 확장 속성 `background-position-x`에서 실패했다. 단색을 유지하도록 runtime fixture 입력만 `background-color`로 정규화하고, 첫 실패 로그·화면을 별도 보존한 뒤 성공 빌드/실행을 다시 캡처했다.
- iOS 앱 로그에서 renderer 생성 결과가 보이지 않았다. Swift host에서 surface 준비와 renderer 보고 문자열을 기록하도록 보강했고, 두 번째 실행 로그에서 `backend=Metal`을 직접 확인했다.
- paint 테스트 추가로 기존 `tests/runtime.rs`와 layout 테스트 파일이 권장 500줄을 넘었다. positioned Flex paint 검사와 owner 조합을 별도 test module로 나눴다.

## 실행 결과와 남은 한계

- `cargo test --locked --workspace --all-features`: 255 passed, 1 ignored. ignored는 macOS host V8 초기화가 되지 않는 기존 C12.2 fixture이며 Android/iOS 실제 runtime 경로로 별도 확인했다.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: 통과.
- `cargo fmt --all -- --check`, `bun run test:css-reference`: 통과, CSS reference 155 tests.
- Android 실기기와 iOS Simulator 각각 15/15 fixture node 일치, 최대 absolute frame 오차 0 CSS px, WGPU 16 boxes presented.
- WPT suite, 전체 20 case 모바일 실행, iOS 실기기, Chrome과 GPU pixel-by-pixel 비교, hardware GPU 성능은 검증하지 않았다.
- Android 빌드는 성공했으며 compile SDK 37.2 / AGP 8.13.2 조합 경고가 있었다. iOS build 성공 로그에는 고정 V8 archive의 중복 timestamp debug-map 경고가 남았다.

플랫폼 raw 로그와 캡처는 [실행 evidence 디렉터리](./c10-3-5-positioned-flex-2026-10-11/README.md)에 있다.
