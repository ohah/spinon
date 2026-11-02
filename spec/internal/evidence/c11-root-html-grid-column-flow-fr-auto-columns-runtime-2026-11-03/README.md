# C11.1 parsed Root HTML `fr` implicit columns runtime

이 자료는 C11.1 `RootHtmlColumnFlowV1`에서 percentage와 `fr`가 섞인 `grid-auto-columns`를 기본 parsed Root HTML 경로로 실행한 Android·iOS Simulator 증거다. C11.1 전체 또는 상위 C11 완료를 뜻하지 않으며 공식 집계 `72/213`도 올리지 않는다.

## 입력과 기대값

- Android와 iOS는 동일한 [runtime fixture](../../../../tests/fixtures/css/c11/root-html-grid-column-flow-fr-auto-columns-runtime.html)를 `RuntimeGpuHost::new_with_root_html`에 전달한다. SHA-256: `d1fe552977c3de7c6107c493ea728c5780c536efc88130c57a3955c630ed4164`.
- 하나의 비중첩 Grid에 width `300px`, explicit columns `[30, 40]px`, rows `[20, 40]px`, gap `5px 3px`, auto-column pattern `10px 10% 1fr`와 12개 고정 크기 item을 둔다.
- 네 개 implicit column은 `[10, 30, 155, 10]px`가 된다. item column starts는 `[0, 35, 80, 95, 130, 290]px`, row starts는 `[0, 23]px`다. Percentage basis는 300px이며 남은 155px가 이 장면의 단일 `1fr`에 배분된다.
- Chrome 156 사전 비교는 별도 [precomparison 기록](../c11-root-html-grid-column-flow-fr-auto-columns-precomparison-2026-11-02/README.md)에서 다룬다. 여기의 화면 캡처는 브라우저 대조나 pixel equality 증거가 아니다.

## Android Simulator

- Android API 37 ARM64 AVD `emulator-5556`만 사용했다. ADB에는 실기기도 보였지만 설치·실행 명령은 모두 `-s emulator-5556`으로 고정했다.
- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C11_1_ROOT_HTML_PERCENT_COLUMNS_FIXTURE_TEST=1 tools/build-android.sh` 및 같은 환경의 `tools/build-android-app.sh`가 성공했다.
- APK SHA-256: `397df320b77fdca0ed7afcddbc120a7d5a18b781129e642acf862e5587ea6819`.
- 실행 보고는 `status=0`, `layout=ready`, `boxes=15`, `presented boxes=15`이고 item x 좌표는 `[0, 35, 80, 95, 130, 290]px`다. [화면](android/avd-screen.png) · [runtime log](android/avd-runtime.log).

## iOS Simulator

- iPhone 17 Pro, iOS 26.2 Simulator (`9DCCE836-7659-4914-B6C1-13F54BFFA9DB`)에서 실행했다.
- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C11_1_ROOT_HTML_PERCENT_COLUMNS_FIXTURE_TEST=1 tools/build-ios-sim.sh`가 `BUILD SUCCEEDED`로 끝났다.
- App executable SHA-256: `76cbb0804cb219ad657c6bbc76c2131524958efc04fae0c9df402b134e3b551a`.
- 환경 갱신은 `status=0`, `layout=ready`, `boxes=15`; draw는 `presented boxes=15`다. Android와 같은 item x·y 프레임을 기록했다. [화면](ios/iphone17pro-screen.png) · [runtime log](ios/iphone17pro-runtime.log).

## 테스트와 artifact 경계

- `cargo test --locked -p spinon-layout root_html_column_flow --lib`: 18개 통과.
- `cargo test --locked -p spinon-style-to-layout root_html_column_flow --lib`: 8개 통과.
- `cargo test --locked -p spinon-style-to-layout parsed_root_html_percentage_auto_rows --lib`: 2개 통과.
- `cargo test --locked -p spinon-style root_html_profile_ --lib`: 10개 통과.
- `cargo test --locked -p spinon-runtime c11_1_root_html_fr_auto_columns_succeed_through_the_default_runtime_route --lib`: 1개 통과.
- `cargo fmt --all -- --check`, `git diff --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo clippy --locked -p spinon-ffi --lib --features c11-1-root-html-percent-columns-fixture-test -- -D warnings`가 통과했다.
- 기본 `spinon-ffi` staticlib에서는 기존 C11.1 opt-in fixture 심볼과 새 runtime fixture 제목 문자열을 찾지 못했다. `c11-1-root-html-percent-columns-fixture-test` opt-in build에서는 기존 심볼과 `C11.1 fr implicit columns runtime` 제목이 있었다. 기존 opt-in 경로를 재사용했으며 새 FFI 심볼은 추가하지 않았다. 이 검사는 staticlib 경계만 확인하며 앱 배포 artifact 크기 검사는 아니다.

## 범위와 보존한 실패

- 화면은 simulator 앱 route의 렌더링 확인이다. Android PixelCopy 및 iOS Metal drawable readback을 하지 않아 surface pixel parity를 주장하지 않는다. 실기기·hardware GPU·iOS compositor 이후 픽셀 결과도 확인하지 않았다.
- 첫 실행은 Chrome 사전 비교용 다중·중첩 Grid 페이지를 단일 Grid profile로 보내 `CascadeFailed`가 났다. 이 입력은 계획 범위의 단일 비중첩 Grid 제한 밖이다. 진단 화면과 로그를 [보존](android/initial-nested-grid-failure.png)했으며, 제품 수정의 실패 증거로 분류하지 않았다. 이후 단일 Grid runtime fixture로 같은 기본 Root HTML 경로를 통과시켰다.
- 이 runtime 장면 하나만 simulator에서 확인했다. list cycling, scalar, subunit, zero-factor 자동 최소값, 1e38 factor 및 width/min/max/aspect-ratio 거부는 별도 Rust focused tests 범위이며 여기서 simulator matrix 완료로 확대 해석하지 않는다.

## 리뷰

- [구현/runtime 실패 관점 검토 20개](implementation-runtime-review-20.md).
- [실제 구현 diff 적대 검토 30개](pr-diff-review-30.md).

## 병합 기록

- 제한 구현 PR [#1066](https://github.com/ohah/spinon/pull/1066)은 `2026-11-02T15:41:43Z` Rebase and merge로 병합됐다. merge SHA는 `2dde385c5f240c8b8837b9e55d9c5431f5f9294b`다.
- GitHub PR check rollup은 비어 있으며 `main`의 required status checks는 설정되지 않았다. CI 통과를 주장하지 않는다.
- C11.1/C11은 미완료이고 공식 완료 수 `72/213`을 유지한다. 세부 내용은 [병합 기록](merge-PR1066.md).
