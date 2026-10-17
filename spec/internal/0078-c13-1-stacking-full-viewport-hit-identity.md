# C13.1 중첩 stacking descendant 전체 viewport hit identity

**상태:** fixture 전용 내부 계약 초안 · 숫자 버전 `0.1.0` 고정 · 공개 API 아님

## 목적과 경계

기존 `c13-1-stacking-descendant-clip` app fixture의 `visible`·`clip`·`hidden` 장면에서 13개 frame과 18개 hit/color sample을 유지하면서, 같은 `RuntimeRenderSnapshot`의 540×128 CSS viewport 전체 정수 좌표를 고정 Chrome oracle와 대조한다. 이 문서는 검증용 FFI report와 Android/iOS app reader의 내부 호출 계약을 정의한다.

전체 69,120개 정수 좌표에는 요소 frame 또는 parent border 바깥이지만 viewport 안에 있는 descendant/overlay 영역도 포함된다. 이 계약은 pixel-center query, 물리 픽셀 전체 비교, DOM `elementFromPoint()` API, event dispatch, CSSOM, WPT, 일반 stacking/overflow CSS 지원 또는 C13.1/C13 완료를 선언하지 않는다.

## 고정 입력

- Chrome: Agent Browser `0.31.1`, Chrome for Testing `155.0.8059.85`, macOS arm64.
- Viewport: `540×128` CSS px, DPR `1`, scroll `(0,0)`.
- Query coordinate: 모든 정수 `(x,y)`, `x=0..539`, `y=0..127`. Half-pixel 경계 표본은 독립 control이며 grid에 섞지 않는다.
- Node index 순서: `root`, visible parent/layer/target/overlay, clip parent/layer/target/overlay, hidden parent/layer/target/overlay.
- Oracle JSON SHA-256: `279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2`.
- 확장 node-index byte stream SHA-256: `866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483`.

## Runtime 입력과 처리

`RuntimeGpuHost`는 fixture-test feature에서만 동일 평가의 불변 `RuntimeRenderSnapshot`과 `RuntimeLayoutFrame`을 사용한다. 기존 18개 고정 hit sample을 만든 뒤 같은 snapshot에서 각 정수 좌표를 한 번 조회한다. 조회된 `NodeId`는 고정 frame 순서로 join한다. unknown node, 누락·추가·중복 frame, 잘못된 oracle, viewport 또는 query 계약 불일치는 성공 report를 만들지 않고 layout 오류로 반환한다.

Chrome oracle은 `full_viewport_hit_grid.rs`의 고정 `include_str!`에서 읽는다. 파일 hash, exact top-level schema, browser/fixture/screenshot/viewport metadata, 13개 고유 node ID 순서, 128개 row, 1,136개 canonical RLE run, row별 540점 coverage, 총 69,120점 및 expanded digest를 먼저 검증한다. 이어 11개 viewport edge/fractional controls를 같은 runtime snapshot에 적용한다.

## FFI report

정상 평가 report의 `hit_test_samples_json` 뒤에 다음 marker 하나와 JSON object를 붙인다.

```text
hit_identity_grid_summary_json={...}
```

Object는 아래 exact key를 가진다. 알 수 없는 필드, 누락 필드, 잘못된 타입·개수·digest는 플랫폼 reader가 거부한다.

| Key | 의미 |
| --- | --- |
| `status` | `mismatch_count`가 0이면 `matched`, 그렇지 않으면 `mismatch` |
| `point_count` | 항상 `69120` |
| `matched_count` / `mismatch_count` | 모든 조회점의 합이 `69120` |
| `mismatch_preview_limit` | 항상 `16` |
| `mismatch_preview` | scan 순서에서 처음 발견한 불일치 최대 16개. 각 항목은 정수 CSS `xCssPx`, `yCssPx`, `expectedNodeId`, nullable `actualNodeId`를 가진다. |
| `edge_control_count` / `edge_controls_matched` | 각각 `11`; 실제 좌표와 기대 ID는 report 생성 전에 내부 검증한다. |
| `expected_identity_sha256` | pinned Chrome grid expanded digest |
| `actual_identity_sha256` | runtime 결과의 69,120-byte node-index stream digest. fixture 외 node/None은 mismatch sentinel로 기록한다. |
| `oracle_sha256` | pinned Chrome JSON file digest |

Mismatch가 있어도 전체 point를 끝까지 세고 digest를 만든 뒤 제한된 preview를 기록한다. JSON이 8,192 byte를 넘으면 오류로 반환하며 잘린 report를 성공 처리하지 않는다. Android/iOS reader는 exact key set, 정수·boolean 구분, count 합, preview 경계·고유 좌표, node ID와 세 digest를 검사한다. `status=mismatch`는 표면 표본 수집으로 진행하지 않는다.

## Snapshot·표면·수명

Android/iOS route는 evaluation과 draw가 같은 generation, document revision, render-tree revision, style revision을 가질 때만 기존 18개 surface sample을 읽는다. Android는 API 37 ARM64 AVD `emulator-5554`의 SurfaceView PixelCopy, iOS는 iPhone 17 Pro / iOS 26.2 Simulator의 Metal drawable sample을 사용한다. 이 표면 검증은 고정 18점이며 full viewport hit identity grid와 별도다.

Stale document revision control은 표면 readback 전에 거부되어야 한다. fixture route 충돌은 host initialization 전에 거부되어야 한다. 앱 재실행 근거는 각 플랫폼에서 서로 다른 새 process 3개를 사용한다.

## Build 경계와 오류

- Rust FFI module/payload는 Cargo feature `c13-1-overflow-clip-fixture-test`에서만 컴파일한다. Android/iOS native 빌드에는 `SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=1`을 함께 지정한다.
- 기본 `spinon-ffi` static archive에는 full-viewport marker와 pinned full-grid JSON이 없어야 한다. opt-in archive에는 marker, digest와 oracle metadata가 있어야 한다.
- parser/oracle/frame/snapshot 오류는 FFI 평가 실패로 전달한다. 앱은 error/mismatch, stale revision, route conflict를 성공 status 또는 surface readback으로 승격하지 않는다.
- 내부 숫자 버전은 출시 전 규칙에 따라 `0.1.0`으로 유지한다.

## 현재 실행 근거

- 구현 source: `1375b7978e6d10d200b8d7b1cc0471dda35e5b73`.
- [Chrome oracle·사전 검증·앱 evidence index](./evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/README.md).
- [Android API 37 AVD runs](./evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/android-avd/c13-1-stacking-android-20261017T195210986Z-cb1a2410/README.md).
- [iOS Simulator runs](./evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/ios-simulator/c13-1-stacking-ios-20261017T195135274Z-ad00912d/README.md).

WPT, physical devices, hardware GPU, full drawable parity, compositor/scanout, general CSS compatibility와 product DOM event/CSSOM은 검증하지 않았다.
