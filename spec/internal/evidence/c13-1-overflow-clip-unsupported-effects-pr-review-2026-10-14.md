# C13.1 clip unsupported effects · PR 변경 관점 30개

대상 diff는 새 test module, test wiring, C13 progress 문서와 evidence다. implementation review 20개와 별도로, 이 변경이 product behavior·ABI·공개 지원 범위를 넓히지 않았는지 확인했다.

| # | 변경 관점 | 확인 결과 |
|---:|---|---|
| 1 | 새 테스트가 production build에 포함됨 | PASS — 상위 `ua_cascade::tests` module은 `#[cfg(test)]`다. |
| 2 | module path가 runtime test tree 밖을 가리킴 | PASS — 새 소스는 `runtime_layout_tests/` 아래에 있다. |
| 3 | 기존 C13 test module 이름과 충돌 | PASS — `c13_1_unsupported_effects_tests`는 기존 float/worker 모듈과 다르다. |
| 4 | `mod.rs` 구조를 새로 도입 | PASS — 기존 `#[path]` 패턴을 사용한다. |
| 5 | production Rust/CSS adapter를 바꿈 | PASS — 변경된 것은 test-only source다. |
| 6 | Cargo feature/default artifact를 바꿈 | PASS — manifest 변경이 없다. |
| 7 | FFI/JNI/header/build flag를 바꿈 | PASS — 플랫폼 boundary 변경이 없다. |
| 8 | parent case에서 `overflow:clip`을 빠뜨림 | PASS — 공통 clip root 문자열을 모든 inline case에서 쓴다. |
| 9 | absolute descendant case에서 position property가 누락 | PASS — `position:absolute`, `left`, `top`이 들어 있다. |
| 10 | browser pixel baseline을 근거 없이 재사용 | PASS — 이 gate는 unsupported authored effect rejection만 비교한다. |
| 11 | root/cascade 오류를 layout error로 잘못 기록 | PASS — inline root count를 먼저 검사하고 layout `Err`를 확인한다. |
| 12 | child node ID를 hard-code해 order 변화에 취약 | PASS — helper가 돌려준 child handle에서 ID를 구한다. |
| 13 | root node ID를 hard-code함 | PASS — root handle에서 실제 ID를 가져온다. |
| 14 | border radius 구현 이름 변화에 취약 | PASS — exact corner longhand 순서를 고정하지 않는 `radius` marker다. |
| 15 | transform 오류를 다른 geometry property로 숨김 | PASS — `transform` 문자열을 요구한다. |
| 16 | failure code가 unsupported 아닌 값이어도 통과 | PASS — `unsupported_inline_property`를 exact 비교한다. |
| 17 | successful layout이 return돼도 test가 묵인 | PASS — `unwrap_err()`가 성공 result를 실패시킨다. |
| 18 | 두 stylesheet case가 같은 CSS 선언을 반복 | PASS — radius와 transform을 독립 rule로 확인한다. |
| 19 | stylesheet 경로에서 element 대신 text node를 선택 | PASS — helper가 style element와 `.painted` element를 분리한다. |
| 20 | author stylesheet parse rejection이 사라짐 | PASS — `compute_request` 자체가 unsupported author CSS error를 반환해야 한다. |
| 21 | CSS property source가 모두 inline으로만 검증 | PASS — stylesheets는 runtime author stylesheet collector/cascade 경로를 탄다. |
| 22 | force-full test가 이전 layout state에 의존 | PASS — `make_layout_snapshot`으로 각 case의 새 document를 만든다. |
| 23 | browser UA/fixture digest를 runtime error 입력과 혼합 | PASS — unrelated browser capture field를 추가하지 않았다. |
| 24 | evidence criteria가 구현 결과를 expected로 역산 | PASS — test 전 acceptance criteria를 먼저 파일로 기록했다. |
| 25 | plan의 C22/C23 fail-closed gate가 계속 미완료로 표시 | PASS — C13.1 progress 문구는 해당 거부 경로가 확인됐고 geometry parity는 owner 이후라고 구분한다. |
| 26 | C13.1 전체 완료를 암시 | PASS — 전체 matrix/iOS/WPT/negative controls는 남기고 C13 미완료 상태를 유지한다. |
| 27 | 구현 리뷰 20개를 PR 리뷰 횟수로 대신 | PASS — 두 review 파일이 독립적으로 연결된다. |
| 28 | simulator가 필요 없는 변경에서 simulator 결과를 허위 기재 | PASS — app build를 새로 실행하지 않았다고 명시한다. |
| 29 | test/log가 마지막 코드 버전과 다름 | PASS — focused/workspace/Clippy 실행은 현재 test source 뒤에 수행했다. |
| 30 | whitespace 또는 formatter 오류가 남음 | PASS — `cargo fmt --check`와 staged `git diff --check`를 재확인 후 기록한다. |

Workspace test는 24 suite에서 730 passed, 0 failed, 4 ignored였고 focused test와 workspace Clippy도 통과했다. 이 PR review는 merge approval을 뜻하지 않는다.
