# C11.4 explicit `align-content:stretch` 구현 PR diff 검토 30개

계획 PR diff 검토와 별개로, 구현 PR에서 변경한 Rust 코드·runtime fixture test·versioned internal contract·실행 evidence를 각기 다른 경계로 재검토했다. 초안 이후 projection, layout, placement 세 거부 경계를 찾아 모두 수정한 뒤 final diff와 재실행 결과를 확인했다.

| # | diff 관점 | 확인 결과 |
|---:|---|---|
| 1 | projection computed longhand의 exact matching | 값은 computed `align-content`의 `normal` 또는 `stretch`뿐이다. |
| 2 | candidate element 구분 | 기존 fixed single-grid candidate helper의 조건을 보존한다. |
| 3 | 다른 alignment longhand 보호 | `align-items`, `justify-content`, `justify-items` 초기값 gate를 유지한다. |
| 4 | existing normal behavior | `normal` 경로를 그대로 허용하고 old fixture regression을 유지한다. |
| 5 | `place-content` shorthand side effect | `justify-content:stretch`를 새로 허용하지 않으므로 shorthand whole-value admission은 없다. |
| 6 | node scope | allowance는 candidate node set membership에서만 계산한다. |
| 7 | style projection route | RuntimeRootHtmlTextPaintV1 외 computed profile는 projection exception 대상이 아니다. |
| 8 | runtime selector/API/FFI churn | 신규 route ID, API, parser token, FFI 또는 fixture selector가 없다. |
| 9 | placement-mode identity | 신 allowance는 existing `RootHtmlAutoRowStretchV1` 하나로 묶인다. |
| 10 | layout intrinsic validator | `Stretch` 예외는 해당 placement mode에서만 통과한다. |
| 11 | legacy profile isolation | `RootHtmlIntrinsicTrackSizingV1` auto-row rejection 회귀를 추가했다. |
| 12 | shared placement validator | 다른 intrinsic profile이 candidate allowance를 상속하지 않음을 확인했다. |
| 13 | disallowed start-like values | `normal`·`flex-start`·safe-start 기존 처리와 candidate `stretch` 예외가 충돌하지 않는다. |
| 14 | validator error property attribution | 실패는 기존 computed `align-content` node/value 진단 경로로 남는다. |
| 15 | explicit/implicit track boundary | explicit row 1, positive implicit rows 2 trace를 보존한다. |
| 16 | track algorithm coupling | 기대값 5px 상수를 track algorithm에 넣지 않고 Taffy/Grid sizing을 재사용한다. |
| 17 | content box and free space | direct unit test의 definite 40×40 input과 10px free-space scene이 명확하다. |
| 18 | normal/stretch geometry | `[10,15,15]` tracks 및 frames 비교가 양쪽 alignment 모두를 검사한다. |
| 19 | alternate alignment contrast | `start`가 `[10,10,10]`인 별도 oracle과 rejected runtime scene을 제공한다. |
| 20 | legacy behavior | 기존 one-row test와 combined intrinsic-row profile regression을 분리해 둔다. |
| 21 | color-based box identification | runtime test가 unique fixed/grid/text colors를 세고 item order를 y로 정렬한다. |
| 22 | node/revision identity | normal/stretch render boxes의 NodeIds/frame tuple 및 cascade/layout key equality를 검사한다. |
| 23 | source order and draw order | two text glyph source values, source revision, and paint command order assertions가 있다. |
| 24 | opt-in diagnostics | track contributions, explicit/implicit count, stage before/after assertions는 fixture-test feature에만 둔다. |
| 25 | unsupported shape mutation | text 변경, reverse order, extra child와 다른 computed values의 rejection 및 no completed layout을 확인한다. |
| 26 | recovery transition | same-session light→dark rejected start→light stretch recovery 순서를 assertion한다. |
| 27 | fixture provenance | source path·precomparison link·fixture hash가 evidence 문서와 같은 값을 가리킨다. |
| 28 | simulator target scoping | API 37 ARM64 emulator와 iPhone 17 Pro/iOS 26.2 UDID를 명시하며 real device로 확대 해석하지 않는다. |
| 29 | restoration and screenshots | 이전 assets/app 설치물의 SHA 검증, visual-smoke limits, iOS linker warnings를 기록했다. |
| 30 | status/contract claims | C11/C11.4 checkbox와 `78/214`, internal `0.1.0`은 유지하며 이 slice를 full conformance로 부르지 않는다. |

최종 검토에서 merge blocker는 남지 않았다. `cargo fmt --all -- --check`, workspace all-target Clippy, focused Rust cases, Chrome precomparison, Android/iOS Simulator execution, restoration record를 각각 별도 근거로 연결한다. GitHub CI는 local results와 혼동하지 않으며 PR check rollup이 비어 있으면 CI-green이라고 쓰지 않는다.
