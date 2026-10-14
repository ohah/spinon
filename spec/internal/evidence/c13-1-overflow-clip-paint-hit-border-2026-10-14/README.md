# C13.1 paint/hit border-strip 경계 · 2026-10-14

## 구현 전 판정 기준

이번 단위는 같은 overflow geometry를 쓰는 static/absolute clip의 Chrome paint pixel과 `elementFromPoint()` 결과를 runtime hit query와 대조한다. 기준 oracle와 사전 조건을 고정한 뒤 Rust 구현을 변경했다. 첫 fixture의 positioned wrapper가 static 사례 hit 영역까지 덮는 control 오류를 발견해 wrapper를 제거했고, root의 margin collapse가 absolute case만 10px 내리는 차이도 `display: flow-root`로 없앴다. 그 뒤 재수집한 fixture JSON과 여섯 query가 현재 기준이다.

고정 환경은 agent-browser `0.31.1`, Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, V8 `15.5.35.20`, macOS `26.5.1`, DPR `1`이다. 전체 version output은 [chrome-version.txt](chrome-version.txt)에 있다.

사전 판정 조건:

1. 두 clip parent는 각각 `x=10..130`, `x=200..320`, `y=10..106`; 두 child는 `x=24..164`, `x=214..354`, `y=24..144`여야 한다.
2. static 사례는 `(125,40)`에서 child, `(128,40)`의 오른쪽 border strip에서 parent, `(131,40)`에서 root가 hit되어야 한다.
3. absolute 사례는 `(315,40)`와 child 안에 겹친 오른쪽 border strip `(318,40)`에서 child, `(321,40)`에서 root가 hit되어야 한다.
4. `(128,40)`과 `(318,40)`의 paint pixel은 모두 border `#f0c674`; `(125,40)`과 `(315,40)`은 child `#57a6d9`여야 한다.
5. Rust runtime은 여섯 target을 재현하고, paint clip edge는 static `[14,126]`, absolute `[204,316]`을 유지해야 한다. painter 순서 조정으로 실제 paint 결과를 바꾸지 않는다.
6. 이는 고정 Chrome 관찰과 runtime hit query의 여섯 점 대조다. DOM event dispatch·전체 CSS hit-test 적합성·전체 C13.1 완료를 뜻하지 않는다.

## Chrome 관찰 결과

agent-browser로 새 [controlled fixture](../../../../tests/fixtures/css/c13/overflow-clip-hit-border-c13-1.html)를 열어 rect와 `elementFromPoint()`를 직접 수집했다. 정확한 geometry와 여섯 결과는 [oracle-fixture-observation.json](oracle-fixture-observation.json)에 있다. 두 사례 모두 expected hit target과 실제 target이 일치했다.

[controlled screenshot](chrome-controlled-border-cases.png)은 `1280×633`, SHA-256 `b23090f55834e8100c929a0fc53a8c9709ac1090005880ec18823d0815ea9229`이다. 주요 RGBA pixel은 `(128,40)=(240,198,116,255)`, `(318,40)=(240,198,116,255)`, `(125,40)=(87,166,217,255)`, `(315,40)=(87,166,217,255)`다. 모든 표본과 기존 precomparison screenshot hash는 [screenshot-pixels.txt](screenshot-pixels.txt)에 둔다.

원래 조사한 두 live runtime fixture 표본도 보존했다. static `(193,28)`은 border color와 parent hit, positioned precomparison `(335,158)`은 parent border paint와 child hit이었다. 해당 화면은 [static](chrome-runtime-static.png), [positioned precomparison](chrome-precomparison-positioned.png)이고 직접 query는 [static JSON](static-clip-boundary.json), [positioned JSON](positioned-clip-boundary.json)에 있다. 현재 작업의 통제된 static/absolute fixture는 이 두 사례와 같은 hit 차이를 재현한다.

## runtime 재현과 수정

- wrapper가 있는 초기 synthetic control의 실패는 제품 근거로 세지 않는다. wrapper가 topmost hit box가 되는 잘못된 트리였으며 결과는 [discarded harness log](runtime-false-failure-wrapper-harness.log)에 남겼다.
- wrapper 제거 후 수정 전 Rust runtime은 static `(128,40)`에서 Chrome의 parent 대신 child를 반환했다. [실패 로그](runtime-red-reproduction.log)가 paint와 hit clip 경계가 섞인 실패를 보존한다.
- static hit clip은 overflow clip의 visual edge를 사용하게 해 border strip에서 자식 hit을 막았다. positioned clip은 기존 border-box 확장 동작을 보존했다.
- 이 수정 뒤 absolute `(315,40)`에서 renderer가 paint order와 hit order를 공유해 parent가 child보다 위에서 반환되는 별도 실패를 재현했다. [실패 로그](runtime-red-positioned-order-reproduction.log)에 당시 box geometry와 순서가 남아 있다.
- C13.1 overflow profile에 한해 runtime box에 독립 `hit_test_order`를 두고 document parent/child 우선순위를 계산했다. 형제 순서는 기존 painter order를 따라가며 `paint_order`와 화면 paint는 그대로 둔다. C12 stacking profile은 기존 `hit_test_order == paint_order`를 유지한다.
- 최종 [focused runtime test](runtime-green-focused.log)가 Chrome JSON의 여섯 표본을 모두 통과했다. 두 paint clip x 경계도 사전 판정값과 일치한다.

## iOS Simulator 재검증

사용자가 허용한 Simulator만 사용했다. 이미 부팅된 iPhone 17 Pro / iOS 26.2 Simulator에서 C13.1 opt-in 앱을 이 worktree 소스로 다시 빌드하고 기존 runtime fixture route를 재실행했다. 실기기는 연결하지 않았고, 실행 후 앱만 종료했으며 Simulator는 부팅 상태로 두었다.

- [빌드 로그](ios-simulator-rebuild.log)는 `SPINON_V8_DIR`를 고정 V8 checkout으로 지정한 C13.1 opt-in Xcode build의 `** BUILD SUCCEEDED **`를 기록한다. 링크 단계에 기존 `RawWindowMetalLayer` duplicate symbol 경고가 있으나 빌드는 성공했다.
- [Simulator log](ios-simulator.log)와 [독립 비교 결과](ios-simulator-verification.txt)는 `status=0`, 13/13 frame(Chrome 최대 오차 0 CSS px), 기존 앱 hit query 15/15 및 viewport 밖 null 4/4를 기록한다.
- [화면 캡처](ios-simulator.png)의 SHA-256은 `9fa1854d0b2ec78c996d7852363a049e3ea2b5287d607a8f249c896d351e0b03`이다. 이는 Simulator 화면과 Metal surface 제출만 보이며 drawable readback 또는 scanout을 증명하지 않는다.
- 이 캡처 시점의 기존 앱 route에는 controlled fixture의 여섯 border-strip 좌표가 아직 없었다. 그 실행은 13-frame/15-query control로만 해석하며, 뒤이어 추가한 분리 route의 여섯 점 결과는 아래 [border fixture 검증](#ios-simulator-border-fixture-검증--2026-10-14)에 기록한다. Android 앱의 새 border 경계 대조는 남아 있다.

## workspace 검증

[Workspace 테스트](workspace-tests.log)는 24개 suite에서 733 passed, 0 failed, 4 ignored다. [Workspace Clippy](workspace-clippy.log)는 `-D warnings`로 통과했고 `cargo fmt --all -- --check`와 `git diff --check`도 통과했다. 실제 iOS Simulator re-run은 위 절에 범위를 한정해 기록했다.

이번 border app 확장 후에도 `cargo test --locked --workspace`는 24개 suite 733 passed, 0 failed, 4 ignored였고, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `git diff --check`가 통과했다. `c13-1-overflow-clip-fixture-test` opt-in FFI null/invalid-output test 1/1, fixture module boundary test 5/5도 통과했다. 원본은 [workspace tests](border-workspace-tests.log), [workspace Clippy](border-workspace-clippy.log), [FFI boundary](border-ffi-boundary-test.log), [module boundary](border-fixture-boundary-test.log), [rustfmt](border-rustfmt.log), [diff check](border-diff-check.log)에 있다.

## 범위 한계

이 단위는 background-only runtime scene에서의 경계 표본과 내부 hit query만 검증한다. Metal drawable readback, PixelCopy, DOM event dispatch, 더 넓은 stacking/overlap 행렬, WPT 후보 분류는 이 근거에 포함하지 않는다. C13.1의 전체 matrix와 상위 C13은 계속 미완료다.

## 실패 관점 검토

[구현 경로 20개](../c13-1-overflow-clip-paint-hit-border-implementation-review-2026-10-14.md)와 [PR 변경 30개](../c13-1-overflow-clip-paint-hit-border-pr-review-2026-10-14.md)는 기존 C13.1 검토와 별도로 기록했다. 새 순서 순회는 반복형이지만 기존 1,024-node 깊이 검증은 C12 profile 경로이므로 C13 hit-order의 깊이 한도는 아직 별도 측정하지 않았다.

## iOS Simulator border fixture 검증 · 2026-10-14

기존 13-node app route와 분리한 opt-in 5-node scene을 추가했다. Chrome controlled fixture 및 [oracle observation](oracle-fixture-observation.json)이 고정 기준이며, 이전에 적어 둔 다음 pass criteria를 iPhone 17 Pro / iOS 26.2 Simulator에서 확인했다.

- [Simulator build log](ios-border-simulator-rebuild.log)는 `SPINON_ENABLE_C04_RUNTIME_GPU=1` 및 `SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=1`로 수행한 build의 `** BUILD SUCCEEDED **`를 기록한다. 링크에 기존 `RawWindowMetalLayer` duplicate-symbol 경고가 남아 있다.
- [실행 log](ios-border-simulator.log)는 `--spinon-c131-overflow-clip-border` route의 `status=0 layout=ready boxes=5`, 정확한 5 frame geometry, `geometry status=ok frames=5`, 그리고 6개 query의 `status=matched`를 기록한다. 기대 target은 `static-target`, `static-clip`, `root`, `positioned-target`, `positioned-target`, `root` 순서다. 전체 [launch output](ios-border-simulator-launch.txt)도 보존했다.
- [Simulator 화면](ios-border-simulator.png)의 SHA-256은 `9888f12ab5cd6ae4a7ad189a1241a378a00120fc6b32c277fdbe45ae87f43379`다. app report와 보이는 Metal scene을 확인하지만 drawable readback 또는 compositor/scanout 증거는 아니다.
- 실행 뒤 app process만 종료했으며 iPhone 17 Pro Simulator는 booted 상태로 남겼다. 실기기를 연결하지 않았다.
- 이전 iOS route의 13-frame/15-query control은 별도 argument와 기존 320×240 viewport를 그대로 사용한다. 새 route는 400×160 CSS px로 실행했다. 두 route를 함께 지정하는 경우도 앱에서 명시적으로 거부한다.
- macOS host-native runtime FFI 실행은 이 환경에서 `V8 Isolate를 만들지 못했습니다`로 시작 단계에서 실패했다. 이는 iOS Simulator 앱 runtime 실패가 아니며, host-native runtime 통과로 세지 않았다. FFI null/invalid-output 경계 테스트와 iOS Simulator runtime report는 별도다.
- [FFI feature isolation](border-feature-isolation.txt)은 matching LLVM 22 `llvm-nm` 및 `strings`로 기본 archive에 신규 border symbol 두 개와 payload 문자열이 없고, opt-in archive에는 둘 다 있음을 확인한다. 이 static archive control은 최종 app 크기나 제품 build 동작을 입증하지 않는다.
- [기존 앱 route regression](ios-existing-c13-route-regression.log)은 같은 앱 binary의 기존 route에서 13 frames와 15 query를 다시 확인한다. [route conflict negative control](ios-border-route-conflict.log)은 border route와 기존 C13 route를 함께 지정했을 때 FFI host를 만들기 전에 앱이 거부함을 기록한다.

이 결과는 새 여섯 좌표의 iOS Simulator 내부 hit query까지 검증한다. Android app의 새 border coordinates, drawable readback, event dispatch, scanout, 전체 C13.1 gate는 남아 있다.

물리 기기는 사용하지 않았다. 이 app boundary 결과를 기능 완료나 Android/iOS 전체 parity로 확대하지 않는다.
