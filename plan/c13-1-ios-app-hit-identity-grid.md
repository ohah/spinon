# C13.1 iOS Simulator 앱 전체 hit identity grid · 2026-10-15

## 목표

기존 iOS C13.1 compact app route는 같은 `RuntimeRenderSnapshot`의 41개 frame과 Chrome에서 고정한 52개 정수 좌표 hit sample을 확인한다. 후속 PR #164는 Android API 37 ARM64 AVD 앱 route에서 300×100 CSS viewport의 전체 정수 hit identity 30,000점을 검사했다. 이 계획은 기존 compact C13.1 FFI report의 `hit_identity_grid_summary_json`을 iOS app route에서 검증해, iPhone 17 Pro / iOS 26.2 Simulator의 실제 앱 실행에서도 같은 고정 Chrome identity grid를 확인한다.

이는 fixture-test 전용 iOS Simulator 앱 실행 결과다. DOM event dispatch, fractional hit behavior, iOS 실기기, Metal drawable readback, WPT 적합성 또는 C13.1 전체 완료를 뜻하지 않는다. C13 runtime/layout 구현과 Chrome oracle은 변경하지 않는다.

## 구현 전 고정 oracle

- 기준 HTML/JS는 `tests/fixtures/css/c13/c13-1-compact-surface-matrix.html` 및 `.js`이며 SHA-256은 각각 `431f4b4452969adcae80c4e5f89e5e45b1e8756fc3c4d8995b7a5eccfbc6f355`, `d36bc1f7d3c791cdc91f6d878c3961e067efc2f88ca1486db8a065c5fd69c760`이다.
- Chrome oracle는 Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, agent-browser `0.31.1`, `300×100` CSS px, DPR 1, scroll `(0,0)`으로 고정됐다. 정수 `(x=0..299, y=0..99)` 30,000점의 `elementFromPoint(x,y)` 결과는 100개 RLE 행과 870 runs로 모두 덮는다. 41개 Node ID 순서와 52개 기존 hit sample subset을 포함한다.
- expected identity SHA-256은 `93cccd042e82d8cd30cb7a8ab34a5154073d4a9eccbc4038294d3ca5b9dfce87`, grid JSON SHA-256은 `35e88e57edc16e526e389317b0d18ef227e35d1356a2353b92476c75ef6414f6`, Chrome observation SHA-256은 `515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299`이다.
- 입력 원본·RLE validator·기존 Android 앱 결과는 [Chrome grid 기준과 검증 자료](../spec/internal/evidence/c13-1-android-app-hit-identity-grid-2026-10-14/README.md) 및 [Android 앱 실행 근거](../spec/internal/evidence/c13-1-android-app-hit-identity-grid-runtime-2026-10-14/README.md)에 있다. 이번 단계에서 oracle을 다시 캡처하거나 expected 값을 구현 결과로 만들지 않는다.

## 계획 수립 시점의 경계

- `spinon_runtime_gpu_host_eval_c13_1_compact_surface_matrix_fixture`는 고정 oracle과 실제 runtime snapshot을 비교한 전체 grid 요약을 report에 넣는다. Android app은 해당 요약의 count·digest·preview를 검사하고 한 줄 summary와 최대 16개 mismatch를 기록한다.
- iOS `evalRuntimeGpuC13_1CompactSurfaceMatrixFixture`는 현재 frame과 52 sample을 검사하지만 전체 grid 요약은 앱 수준에서 검사하지 않는다. 요약 필드가 빠져도 현재 iOS app route가 30,000점 검증을 통과한 것으로 보일 수 있다.
- 이 단계는 Swift/Objective-C++ app integration과 테스트에 한정한다. 새 runtime profile, fixture payload, 공개 API 또는 추가 FFI symbol을 만들지 않는다. 기존 `c13-1-overflow-clip-fixture-test` opt-in 경계를 유지한다.

## 구현 범위

1. `--spinon-c131-compact-hit-identity-grid` 단독 iOS simulator route를 추가한다. 기존 compact fixture host와 단일 evaluation을 재사용하고, 다른 C13.1 또는 C11 fixture argument와 동시 선택하면 host 초기화 전에 fail-closed한다.
2. iOS app은 FFI report의 `hit_identity_grid_summary_json` 객체를 구조적으로 추출한다. 잘린 JSON, 중첩 object/배열 경계, 누락·중복 필드, 잘못된 타입, boolean·소수·문자열 수치, 범위 밖 preview 좌표, 잘못된 preview Node ID 타입, 허용 개수 초과를 성공으로 변환하지 않는다.
3. 성공 조건은 `status=matched`, `point_count=30000`, `matched_count=30000`, `mismatch_count=0`, empty preview이며 expected·actual identity digest가 모두 같은 pinned identity SHA이고 grid oracle·Chrome observation digest도 각각 pinned SHA와 같은 것이다. `matched_count + mismatch_count`가 전체 점 수와 같은지와 preview limit 16도 확인한다. 실패 요약에는 status, count, digest와 최대 16개 mismatch 좌표만 남기고 30,000개 항목을 로그로 출력하지 않는다.
4. runtime report의 generation 및 단일 실행 ID를 frame, 52 sample, full-grid summary marker에 함께 남겨 서로 다른 앱 run이나 stale callback의 로그를 합쳐 성공 처리하지 않는다. 실행 ID는 매 simulator 실행에서 새로 발급한다.
5. 순수 report parser/validator의 malformed/missing marker, 잘못된 count, digest mutation, preview overflow·좌표 범위 위반 mutation을 각각 검사한다. JSON 파서는 escape와 중첩 구조를 보존하고 prefix 일치만으로 전체 grid 검증을 통과시키지 않는다.
6. 내부 계약 0060, 공식 상태 대장, C13.1 계획 및 evidence README를 갱신한다. C13.1과 공개 CSS 지원 상태는 미완료로 유지한다.

## 통과 기준

- Chrome oracle validator와 기존 Rust FFI hit-grid tests가 지정 hash와 30,000점 경계를 계속 통과한다.
- iPhone 17 Pro / iOS 26.2 Simulator feature-on 앱에서 새 route를 3번 새 run ID로 실행한다. 각 실행에서 `status=0 layout=ready`, 41/41 frame, 기존 52/52 hit sample, full grid 30,000/30,000 match, mismatch 0, 두 identity digest와 grid/observation digest 4개 exact를 모두 확인한다. 앱 screenshot, simulator/runtime metadata, build 및 launch logs를 보존한다.
- iOS app parser negative controls는 malformed/truncated report, grid summary 누락, altered expected/actual/oracle/observation SHA, 합이 맞지 않는 count, 잘못된 status와 preview 좌표를 전부 거부한다. 잘못된 argument 조합은 host 생성 전 reject marker를 남긴다.
- 동일 앱 report의 frame·52 sample·grid summary가 같은 runtime generation에 귀속되는지 로그에서 확인한다. 앱을 다시 실행하면 새 run ID를 발급해 이전 로그와 구분한다. 프로세스별 generation 번호가 재사용될 수 있으므로 실행 사이 generation 숫자의 차이는 합격 기준으로 삼지 않는다.
- iOS 앱 screenshot은 기능 화면 증거로 PR 본문에 inline 표시한다. PR 본문의 image URL은 PR head commit artifact를 가리키고 agent-browser에서 URL, 표시 상태, natural pixel dimensions를 확인한다.
- 관련 Swift/Objective-C++ compile, parser tests, Chrome oracle validator, Rust focused tests, `cargo fmt --all -- --check`, workspace tests/Clippy, iOS Simulator feature-on build 및 앱 runtime을 기록한다. Android 실기기·iOS 실기기 연결은 사용하지 않는다.

## 실행 결과 · 2026-10-15

- `--spinon-c131-compact-hit-identity-grid`를 앱 진입점과 C13.1 compact fixture host에 연결했다. 앱 파서는 FFI report의 고정 summary JSON을 fail-closed로 확인하며 frame, 52 hit sample, full-grid 결과에 같은 app run ID와 runtime generation을 기록한다.
- iPhone 17 Pro / iOS 26.2 Simulator에서 새 PID 3개로 실행했다. 각 실행의 41 frame, 52/52 fixed sample, 30,000/30,000 full-grid identity가 일치했고 identity·grid·Chrome observation digest가 pinned 값과 같았다. 세 실행 모두 WGPU Metal surface에 41 box를 제출했다.
- route 중복 negative control은 runtime host 생성과 eval 전에 거부됐다. 화면에는 `완료 · C13.1 Chrome hit identity 30,000/30,000 · 52/52`가 표시된다.
- 초기 재현에서 `AppDelegate`가 새 route flag를 runtime GPU demo로 넘기지 않아 앱이 흰 기본 화면에 남았고, 고친 뒤에는 environment update lane이 완료 상태를 일반 layout 상태로 덮었다. route 분기와 성공 상태 보존을 수정한 뒤 세 번 모두 화면과 로그를 다시 캡처했다.
- [실행 증거](../spec/internal/evidence/c13-1-ios-app-hit-identity-grid-2026-10-15/c13-1-ios-hit-grid-20261014T154338634Z-f1835c60/README.md)는 build/runtime logs, 실행별 screenshot, 고정 digest, device metadata, frame/sample별 run ID·generation 상관관계, parser 23개 negative controls, FFI tests 8/8, workspace tests·Clippy와 rustfmt 결과를 보존한다.
- 성공 화면·full-grid parser는 iOS 26.2 Simulator와 fixture-test build에 한정된다. 실기기, Metal drawable readback, DOM event dispatch, WPT, physical pixel parity와 C13.1 전체 완료를 증명하지 않는다.

## 제외

Android app 재검증, iOS Metal drawable pixel readback, Android 788×263 전체 physical-pixel byte parity, fractional/subpixel hit query, DOM event/CSSOM scroll, WPT, 실기기·hardware GPU, C13.1 전체 완료와 공개 CSS support 선언은 이 작업 범위가 아니다. PR #164 AVD 결과는 기존 Android baseline으로 인용하며 iOS 결과와 합산해 동일 플랫폼 행렬이라고 부르지 않는다.

## 순서

1. 이 문서의 별도 계획 실패 관점 20개와 계획 PR 변경 관점 20개를 각각 완료한다.
2. 고정된 Chrome RLE oracle validator와 source hash를 재검증한다.
3. fixture-only iOS app route, bounded fail-closed parser, parser mutation tests를 구현한다.
4. iOS Simulator에서 세 번 새 실행을 수행하고 screenshot/log/build 근거를 저장한다.
5. 계획 검토와 겹치지 않는 신규 구현 실패 관점 20개와 PR 변경 관점 30개를 별도로 수행한다.
6. 최신 `main`, required CI, mergeability를 확인한 뒤 Rebase and merge한다.
