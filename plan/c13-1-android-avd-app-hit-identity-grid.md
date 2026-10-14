# C13.1 Android AVD 앱 전체 hit identity grid · 2026-10-14

## 목표

기존 C13.1 Android compact runtime route는 Chrome의 52개 고정 좌표에서 `RuntimeRenderSnapshot::hit_test_css_point` 결과를 확인한다. 후속 PR #162는 같은 300×100 CSS surface의 30,000 CSS pixel center RGBA를 Android PixelCopy와 exact 비교했다. 이 계획은 Android app/FFI route에서 fixture hit function이 **모든 30,000 정수 CSS 좌표**에 대해 Chrome `elementFromPoint(x,y)`와 같은 fixture NodeId를 반환하는지 대조한다.

이는 앱에 연결된 integer-point hit identity 검증이며 DOM event dispatch, subpixel/fractional hit behavior, 전체 C13.1 CSS 지원이 아니다. 기존 Rust host의 `1280×1100` 1,408,000-point scan, PixelCopy CSS pixel-center surface grid, Android app의 fixed 52 hit samples와 각각 독립 결과로 유지한다.

## 구현 전 비교와 oracle

- 기준 fixture는 `tests/fixtures/css/c13/c13-1-compact-surface-matrix.html` 및 `.js`다. 파일 SHA-256은 각각 `431f4b4452969adcae80c4e5f89e5e45b1e8756fc3c4d8995b7a5eccfbc6f355`와 `d36bc1f7d3c791cdc91f6d878c3961e067efc2f88ca1486db8a065c5fd69c760`다.
- 기준 Chrome observation SHA-256 `515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299`, Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, agent-browser `0.31.1`, viewport `300×100`, DPR 1, scroll `(0,0)`이다.
- Oracle `elementFromPoint` query convention은 정수 `(x,y)`, `x=0..299`, `y=0..99`다. 30,000개 topmost identities는 100 row RLE, 870 runs, 41 registered nodes이며 1-byte node-index stream SHA-256은 `93cccd042e82d8cd30cb7a8ab34a5154073d4a9eccbc4038294d3ca5b9dfce87`다. 미등록 hit 0, 모든 row coverage 300, 기존 fixed 52-point subset 52/52다.
- CSS pixel center `x+0.5,y+0.5`의 별도 probe에서는 마지막 열과 마지막 행에 걸친 399점이 Chrome `null`이었다. W3C CSSOM View 알고리즘 및 root fallback과 차이가 있는 관찰이라 [사전 비교 기록](../spec/internal/evidence/c13-1-android-app-hit-identity-grid-2026-10-14/README.md)에 남기고 이번 oracle에서 제외한다. 정수 query 결과를 fractional 좌표 규범으로 일반화하지 않는다.
- 실행 전 세부 source hash·capture script·전체 RLE·validator·mutation 결과는 [precomparison evidence](../spec/internal/evidence/c13-1-android-app-hit-identity-grid-2026-10-14/README.md)에 고정했다.

## 구현 범위

1. 별도 fixture-test FFI evaluation은 기존 compact route host snapshot을 사용해 100×300의 각 정수 CSS point를 정확 한 번 query하고 expected Chrome node index와 비교한다. window size, viewport, root, node registry와 scene/frame NodeId join을 먼저 검증한다.
2. 현재 JNI evaluation report buffer는 32,768 bytes다. 모든 point를 개별 JSON/Logcat row로 출력하지 않는다. FFI는 `point_count`, `match_count`, `mismatch_count`, oracle digest와 최대 16개 mismatch sample을 bounded summary로 반환하고, 초과 용량·serialization failure는 오류로 닫는다.
3. Chrome oracle RLE는 fixture-test 전용 source data로 고정하고 그 source hash, Chrome observation hash, viewport, node order, row coverage, run adjacency, 52 fixed subset을 검증한다. expected grid가 변조되거나 누락되면 AVD 성공으로 처리하지 않는다.
4. Android native bridge는 full-grid report를 기존 52 sample validation과 분리된 log marker로 남기고 query마다 Logcat을 쓰지 않는다. activity route는 기존 `spinon_c131_compact_surface_matrix`의 generation/host lifecycle을 재사용하고 route collision/second host를 만들지 않는다.
5. 기본 `spinon-ffi` artifact에는 신규 full-grid symbol과 oracle payload가 없어야 한다. 기존 `c13-1-overflow-clip-fixture-test` positive artifact에만 포함되어야 한다. C header, Rust `cfg`, Gradle/JNI native define은 같은 feature 경계를 지킨다.

## 판정 기준

- Chrome oracle validator와 mutation tests가 모두 통과해야 한다. source hash, browser revision, viewport, node order, 30,000 count, RLE coverage, expanded digest 또는 fixed sample subset 불일치는 fail-closed한다.
- Rust focused test에서 전 좌표 match 30,000/30,000, mismatch 0을 확인한다. single point identity, row gap/overlap, x/y bounds, missing frame, duplicate node, altered oracle hash negative controls는 구체 실패가 되어야 한다.
- Android API 37 ARM64 AVD(`ro.kernel.qemu=1`, API 37, ARM64)에서 C13.1 fixture-test APK를 설치/실행하고 동일 runtime generation에서 41/41 frame, 52/52 fixed samples, 30,000/30,000 hit grid, 52/52 PixelCopy를 확인한다. AVD software backend 정보, APK hash, run ID, PID와 capture hash를 기록한다.
- feature boundary, Rust workspace, Clippy, rustfmt, Android build/AVD route, iOS Simulator build의 shared Rust FFI compile, `git diff --check`를 확인한다. iOS route 동작은 이 Android integration PR의 판정 범위가 아니다.
- Android 앱 전체 화면과 PixelCopy surface를 evidence에 보관하고, PR 본문에 Android 실행 화면을 inline 첨부해 agent-browser에서 URL·natural dimensions·완료 로드를 검사한다.
- C13.1 state는 미완료로 둔다. fraction query, DOM event dispatch, WPT, iOS full-grid app query, physical-device/hardware GPU 및 scroll/CSSOM을 통과로 표시하지 않는다.

## 제외

이 작업은 `document.elementFromPoint` 외 DOM event dispatch, fractional CSS coordinate/hit edge, Android touch/mouse routing, iOS runtime full-grid query, WPT execution, scroll state/CSSOM, C13.1 완료, 788×263 physical byte parity를 구현하지 않는다.

## 순서

1. 계획 및 [별도 20개 계획 실패 검토](../spec/internal/evidence/c13-1-android-app-hit-identity-grid-2026-10-14/plan-review-20.md)를 완료하고 계획 PR을 병합한다.
2. 고정 Chrome RLE oracle 및 validator에서 계획 pass criteria를 다시 검사하고, 구현 전 20개 리뷰 범위와 교차 확인한다.
3. fixture-only Rust/FFI/JNI/app route를 구현하고 적대적 failure controls를 실행한다.
4. 새 20개 implementation review와 독립 PR delta review를 추가한 뒤 최신 main에 rebase하고 checks/mergeability를 확인해 Rebase and merge한다.
