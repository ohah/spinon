# C13.1 · 중첩 stacking descendant 앱 전체 viewport hit identity

**상태:** Chrome 155 integer-grid 사전 oracle 고정 · Android/iOS fixture runtime 구현·Simulator 실행 완료 · PR review/merge 대기
**선행 결과:** [중첩 stacking descendant fixture](./c13-1-stacking-descendant-clip-controls.md) · [전체 viewport host hit 결과 PR #160](https://github.com/ohah/spinon/pull/160)
**내부 계약:** [0078 · C13.1 전체 viewport hit identity](../spec/internal/0078-c13-1-stacking-full-viewport-hit-identity.md)
**Chrome 사전 근거:** [캡처·RLE grid·경계 관찰](../spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/README.md)
**계획 실패 관점 20개:** 같은 evidence 디렉터리의 [`plan-review-20.md`](../spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/plan-review-20.md)

## 목표와 제품 경계

기존 중첩 stacking descendant runtime app route는 `visible`·`clip`·`hidden` 각 case에서 6개씩, 총 18개의 고정 hit 좌표를 확인한다. 이 후속은 동일한 540×128 CSS viewport의 모든 정수 좌표를 Android/iOS 앱에 연결된 runtime snapshot에서 조회해 pinned Chrome 155 `elementFromPoint()` 결과와 exact 비교한다.

이는 세 개 고정 case의 fixture-test 검증 범위를 넓힌다. 새로운 CSS 지원, 일반 stacking·overflow 동작, DOM event dispatch, public CSSOM, WPT conformance, 실기기 지원 또는 C13.1/C13 완료를 주장하지 않는다. 기존 13개 frame·18개 hit/RGBA 표본은 그대로 유지한다.

## 고정 입력과 oracle

- Fixture: [`overflow-clip-stacking-descendants-c13-1.html`](../tests/fixtures/css/c13/overflow-clip-stacking-descendants-c13-1.html), SHA-256 `6b691ddf5afd44543b438b47200f9a6f85981fa3d3c30e1619185a65d320791e`.
- Browser: Agent Browser `0.31.1`, Chrome for Testing `155.0.8059.85`, macOS `26.5.1` arm64. Full-version metadata와 UA를 [oracle JSON](../spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/chrome-full-viewport-hit.json)에 고정했다.
- Viewport: `540×128` CSS px, DPR `1`, scroll `(0,0)`.
- Query: `document.elementFromPoint(x, y)`의 정수 좌표 `x=0..539`, `y=0..127`. 이는 CSS pixel center query가 아니다. 18개 기존 half-pixel sample은 별도 oracle로 유지하며 정수 결과로 fractional semantics를 추론하지 않는다.
- Node map은 fixture DOM 순서로 `root`, visible parent/layer/target/overlay, clip parent/layer/target/overlay, hidden parent/layer/target/overlay의 13개 ID를 사용한다. ID index 바이트는 위 순서에 대응한다.
- 기대 표본은 `540 × 128 = 69,120`점, 128개 연속 row다. 각 row는 `[nodeIndex, runWidth]` 쌍의 canonical RLE로 저장하며 한 row의 run 합은 정확히 540이어야 한다. 전체 확장 node-index stream SHA-256은 `866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483`이다. 현재 oracle은 1,136 runs다.
- 기존 Chrome capture는 정수 viewport right/bottom edge `(539,64)`, `(0,127)`, `(539,127)`에서 `root`를 반환한다. `(539.5,0.5)`, `(0.5,127.5)`, `(539.5,127.5)`는 `null`이다. 네 경계 밖 control도 `null`이다. 이 fractional 차이는 계획에 명시적으로 보존하고 full-grid query에는 혼합하지 않는다.
- Chrome screenshot은 `540×128` PNG, SHA-256 `a9727d516837df6c8a5d77e624aee3463b08ebfb991d8eb622d52d026c64585c`다. screenshot은 브라우저 oracle 장면만 나타내며 앱·GPU 증거가 아니다.

## 구현 범위

1. `c13-1-stacking-descendant-clip` opt-in fixture host의 immutable `RuntimeRenderSnapshot`에서 각 정수 CSS point를 정확히 한 번 query한다. query 결과 `NodeId`를 13개 고정 frame의 `data-id` inventory에 join한다. node 누락·중복·추가, frame 순서 변경, viewport 변경은 full-grid 비교 전에 오류로 닫는다.
2. Chrome RLE JSON은 fixture 전용 코드에서만 읽는다. validator는 schema/source/browser version/fixture hash/viewport/DPR/scroll/node order/69,120점/row coverage/canonical adjacent runs/expanded digest/edge controls를 검증한다. 미등록 node index, 과소·과다 row, run gap·overlap, 변조 hash는 거부한다.
3. Rust 비교기는 모든 point identity를 exact 대조하고, 보고에는 `point_count`, `match_count`, `mismatch_count`, expected/actual expanded digest 및 최대 16개 mismatch만 쓴다. JNI/Logcat과 iOS log에 점별 69,120개 행을 싣지 않는다.
4. Android/iOS fixture route는 같은 eval/draw generation, document/render-tree/style revision의 frame을 사용한다. 현재 18개 hit identity와 18개 RGBA surface 표본 및 stale-document/route-collision control을 보존한다. full-grid report는 새 scene evaluation에 결합해 stale snapshot 결과를 성공 처리하지 않는다.
5. 새 report와 RLE payload는 기존 C13.1 fixture-test opt-in 경계에만 둔다. default/product archive에 새 FFI symbol·고유 JSON/payload가 없는 negative control과 fixture-test artifact positive control을 확인한다.

## 판정 기준

- 고정 validator 및 mutation tests 통과, Chrome oracle 69,120점 RLE 무결성 exact.
- Rust fixture host `69,120/69,120` exact `NodeId`, mismatch 0, expanded digest 일치.
- Android API 37 ARM64 AVD `emulator-5554`와 iPhone 17 Pro / iOS 26.2 Simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 각각 서로 다른 새 app process 3개가 `69,120/69,120` exact identity를 보고한다. 각 run은 13 frame, 기존 18 hit·18 RGBA 표본, 동일 revision tuple, APK/app/library hash를 보존한다.
- App route는 중복/누락 point, unknown node, altered expected digest, row gap/overlap, stale document revision, 잘못된 route 단독·충돌, output capacity 초과를 성공으로 만들지 않는다. 잘못된 run의 일부 match를 누적된 성공 결과에 합치지 않는다.
- 기본/opt-in artifact 경계를 각각 검사한다. Android surface는 PixelCopy, iOS는 Metal drawable 표본을 유지하되 전체 viewport hit identity를 full-surface pixel parity로 해석하지 않는다.
- Android와 iOS Simulator 화면을 evidence에 저장한다. PR 본문에는 Chrome oracle와 검증된 Simulator 화면을 inline 첨부할 수 있으면 올리고, Agent Browser로 실제 이미지 로드를 확인한다. browser process 종료를 확인하고 terminal-browser/tmux는 건드리지 않는다.
- `cargo fmt --all -- --check`, 관련 C13.1 fixture/runtime/FFI tests, workspace tests, Clippy, `git diff --check`, Android API 37 AVD build/run, 지정 iOS Simulator build/run을 기록한다. 실행하지 않은 WPT·실기기·hardware GPU·OS compositor/scanout은 미검증으로 남긴다.

## 구현 실행 결과

- 구현 source commit은 `1375b7978e6d10d200b8d7b1cc0471dda35e5b73`이다. Rust comparator는 같은 `RuntimeRenderSnapshot`에서 69,120개 integer point를 한 번씩 조회하고, pinned Chrome oracle의 1,136-run grid·11 edge controls·expanded digest를 검증한다. 불일치 preview는 16개로 제한한다.
- Android API 37 ARM64 AVD `emulator-5554`와 iPhone 17 Pro / iOS 26.2 Simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 각각 신규 process 3개가 69,120/69,120 hit identity를 기록했다. 기존 13 frame·18 hit sample과 각각 18 PixelCopy/Metal RGBA 표본을 유지했다. 앱 executable/APK hash, runtime revision tuple, screenshot, 원본 log와 surface artifact를 [증거 인덱스](../spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/README.md)에 보존했다.
- Stale document revision은 Metal/PixelCopy readback 전에 거부되고, 중복 fixture route는 host 초기화 전에 거부됐다. default `spinon-ffi` static archive에서 full-grid marker/oracle 문자열이 빠지고 C13.1 opt-in feature archive에는 들어감을 확인했다.
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, focused FFI 6/6·renderer 1/1 tests, fixture module boundary 6/6, Android build/AVD capture, iOS Simulator build/capture가 통과했다. 전체 workspace test와 WPT는 실행하지 않았다.
- [구현 실패 관점 20개](../spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/implementation-review-20.md)와 별도의 [PR 변경 관점 30개](../spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/pr-delta-review-30.md)를 기록한다. 이 구현 slice는 fixture 전용이며 C13.1/C13 완료로 승격하지 않는다.

## 제외 범위

- half-pixel/fractional hit identity. Chrome edge 관찰상 정수 viewport grid와 다른 결과를 내므로 별도 작업 전까지 범위를 섞지 않는다.
- pointer/touch/wheel dispatch, click activation, accessibility target order, DOM `elementFromPoint()` API, CSSOM, WPT 실행.
- 전체 WGPU surface의 재비교. 해당 입력의 full physical-pixel parity는 별도 근거가 있으며 현재 후속은 hit identity만 추가한다.
- 새로운 overflow, stacking, z-index, clipping CSS 입력; general document compatibility; physical device/hardware GPU/compositor/scanout.
- 이 검증 slice를 C13.1 또는 C13 완료로 승격하는 상태 체크.

## 순서

1. 계획 실패 관점 20개와 계획 PR의 별도 30개 변경 검토를 완료하고 계획을 병합한다.
2. Rust full-grid comparator와 RLE mutation test를 추가하고 focused/workspace gates를 실행한다.
3. opt-in Android/iOS FFI route, bounded report, stale/collision/overflow controls와 artifact boundary를 구현한다.
4. Android API 37 AVD와 iOS Simulator에서 각각 새 process 3개로 검증하고 화면·로그·hash·revision tuple을 보존한다.
5. 계획 검토와 별개인 새 implementation failure review 20개와 독립 PR delta review 30개를 기록한다. 결함을 수정하면 해당 관점을 다시 실행한다.
6. 최신 `main`, CI/review/mergeability를 다시 확인해 PR을 Rebase and merge한다. STATUS에는 통과한 fixture 범위만 더하고 C13.1/C13 상위 체크를 유지한다.
