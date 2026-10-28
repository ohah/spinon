# C11.4 fit-content implicit column 구현 적대 검토 20개

대상은 pinned WPT `fit-content-track-sizing.html` 한 assertion을 위한 Rust runtime fixture, FFI report, Android/iOS fixture host, 최종 시뮬레이터 실행이다. 계획 검토 및 PR diff 검토와 다른 구현 중심 관점 20개를 따로 확인했다. 발견된 결함은 수정 후 영향 범위를 다시 검사했다.

| # | 실패 관점 | 적대 확인과 결과 |
|---:|---|---|
| 1 | 유사하거나 접미사가 붙은 scene ID가 새 route로 진입 | FFI allowlist는 exact ID만 반환하고 `;throw` 변형 및 미등록 ID를 거부한다. 정확한 route test 1/1 통과. |
| 2 | fixture route가 기본 제품 archive로 새어 나옴 | `c04-runtime-gpu` archive에서 scene ID·node ID·Rust marker 네 종류가 모두 0개다. Fixture-test positive control에는 네 종류 모두 있다. |
| 3 | 앱 host가 fixture feature가 꺼진 상태에서 scene을 허용 | Android route는 `SPINON_C04_RUNTIME_GPU_FIXTURE_TEST` 빌드 설정을 확인하고, Rust fixture는 전용 Cargo feature로 격리된다. 일반 CSS route는 추가하지 않았다. |
| 4 | root의 다른 namespace/tag 또는 잘못된 문서 위치가 고정 fixture로 처리 | HTML namespace, `div`, 문서 root의 직접 자식 조건이 모두 다르면 `prepare`가 거부한다. |
| 5 | extra/missing/reparented grid item 또는 child | root 자식 1개, item 자식 1개와 양쪽 parent handle을 각각 검사한다. 이 fixture의 3-node tree 외에는 성공 경로가 없다. |
| 6 | ID 충돌이나 computed-style node 누락/추가 | 세 node ID를 순서와 관계없이 고정 ID로 확인하고, style ID 집합이 세 node와 일대일인지 검사한다. `style_for_node`는 중복도 거부한다. |
| 7 | CSS 문자열은 맞지만 typed style은 다른 값 | Root의 display/dimensions/grid declarations와 `ComputedCssGridLayout`을 함께 확인한다. width, height, gap, explicit tracks, auto tracks, flow, repeat, areas, line names가 고정값이어야 한다. |
| 8 | implicit track을 explicit track 또는 다른 cap과 혼동 | typed LayoutInput은 explicit `[60px]`, auto columns/rows `fit-content(40px)`, track-sizing placement를 exact 비교한다. 41px cap·extra explicit track·column flow 변조 unit control은 거부된다. |
| 9 | item placement 또는 automatic minimum이 달라져도 통과 | `grid-column` start `Line(2)`, end `Auto`, computed `min-width:0`, typed minimum 0을 확인한다. Columns contribution은 `Minimum=0` 및 `SpecifiedMinimum`으로 고정한다. |
| 10 | child의 크기나 box edge가 max-content 입력을 바꿈 | child `400×10px`, min/max dimensions, margin/padding/border를 검사한다. root-relative child frame과 400px contribution이 끝까지 유지된다. |
| 11 | style snapshot과 다른 document/render-tree revision의 layout output 결합 | generation, document, render-tree, style, environment revision을 `RuntimeUaCascadeKey`와 대조하며 HostDocument 이외의 source를 거부한다. |
| 12 | frame 집합에 누락·중복·extra node가 있거나 tolerance가 넓음 | 전체 frame 개수가 3이고 대상 node map도 3개여야 한다. root/item/child 네 좌표 모두 `1e-4 CSS px` 이내다. |
| 13 | 다른 Grid root나 누락된 축 trace를 성공으로 취급 | trace 하나, 대상 root 하나, Columns/Rows 두 pass만 허용하고 축 sequence를 각각 0/1로 검사한다. |
| 14 | Columns final size가 맞아도 sizing path나 contribution이 잘못됨 | 여섯 stage base 배열, gutter 위치, explicit `60px`과 implicit `40px` 구분 및 `Minimum=0`, `MinContent=400`, `MaxContent=400`을 검사한다. base·minimum·max-content 변조 unit control이 실패한다. |
| 15 | Rows frame만 맞고 cross-axis contribution이 다른 경우 | 이번 검토에서 기존 validator가 row contribution을 보지 않던 결손을 찾아 수정했다. `Minimum=10` (`AutomaticMinimum` flags), `MinContent=10`, `MaxContent=10`, item/span/axis crossing을 Rust와 Android/iOS matcher가 검사한다. 잘못된 source와 값에 대한 unit control이 실패한다. |
| 16 | `StretchAutoTracks.applied`를 실제 track 성장으로 오해 | matcher는 Columns `false`, Rows `true`를 보존하고 row base가 10px인 것을 따로 확인한다. 문서에 `applied`의 Taffy 진단 의미를 명시했다. Raw internal `growthLimit` 자체는 이 slice에서 검증 대상이 아니며 규범 trace 일치 주장을 하지 않는다. |
| 17 | FFI report가 runtime 검사와 다른 node/frame/contribution을 내보냄 | 세 frame과 각 축 stage/contribution을 같은 completed output에서 직렬화한다. Host matcher가 row/column contribution 필드 누락·순서·값을 거부한다. |
| 18 | Android malformed JSON, missing fields, non-finite geometry가 성공 | `JSONTokener` parse failure, 누락 객체, 잘못된 scalar/frame 및 trace mismatch는 `false`로 닫힌다. `Double.isFinite`와 exact field/cardinality checks를 사용한다. |
| 19 | iOS NSNumber/type 혼합이나 stale report가 성공 | JSON field type, finite `Double`, node set, computed style, trace stage, contribution, revision source와 same-revision presentation identity를 모두 확인한다. |
| 20 | box 결과만 맞고 실제 root 밖 overflow는 잘리거나 다른 surface를 읽음 | Android PixelCopy와 iOS Metal drawable에서 root 안, child 안, root 밖 child, surface clear 네 RGBA 표본이 각각 4/4 exact다. iOS readback은 compositor 제출 전이라고 제한을 기록했다. |

## 수정 후 확인

- 전체 workspace fixture-feature Clippy가 `clone_on_copy` 두 건을 찾아 수정했다. 최종 run 통과.
- 처음 작성한 Rows stage oracle가 실행 trace와 달랐고, `StretchAutoTracks.applied`를 축별로 잘못 해석했다. 이를 Taffy 내부 회귀 기준으로 표시하고 independent WPT/CRD 기대값과 분리했다.
- 처음 FFI test filter는 0개를 선택했다. 테스트 이름을 특정해 allowlist test 1/1을 재실행했다.
- row contribution 검증 결손을 수정한 뒤 Android API 37 ARM64 emulator와 iPhone 17 Pro/iOS 26.2 Simulator를 다시 빌드·실행했다. 최종 run ID와 hashes는 [runtime README](./README.md)와 [run metadata](./run-metadata.json)에 있다.

## 남은 한계

이 검토는 하나의 opt-in WPT assertion이다. 전체 WPT suite, 일반 author CSS admission, 임의 Grid input, Android 실기기·hardware GPU, iOS compositor 이후 픽셀, C11.4 전체 완료를 증명하지 않는다. Rust/runtime implementation review이며 다음 [PR diff 검토](./pr-diff-review-30.md)는 별도 실패 관점을 사용한다.
