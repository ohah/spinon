# Taffy 0.15.0 전환 뒤 기존 C11 Grid Simulator 회귀

**기준 후보:** `91ae318556f5f9020d235b4753a43da524d8cfef` 위의 C11.4 작업 트리. `spinon-layout`의 Taffy 의존성을 registry `0.14.0`에서 vendored path `0.15.0`으로 바꿨다. trace flag만 opt-in이며 Taffy 0.15.0 계산 코드는 C11.1~C11.4를 포함한 모든 `spinon-layout` profile에서 사용된다.

## Android API 37 ARM64 AVD

- 실행: `c1133-matrix-20261019T033459682Z-49847934`, 대상 `emulator-5554`, Android 17 / API 37.
- 고정 Chrome 154 DPR 1 기준 26 scene·65 node: 성공 19, cascade fallback 5, 기대 거부 2. 보이는 frame 61개의 최대 차이는 0 CSS px다.
- `basic-rectangles` PixelCopy 표본 2/2 exact, 최대 RGBA channel 차이 0이다. renderer는 ANGLE/Vulkan/SwiftShader다.
- APK SHA-256: `cf6a5e4f4134c449a67a94365963bfd9b6672eee6076a2bfbd1d5fb3bab6c94f`.
- [원본 README·result.json·scene log·화면](../c11-3-3-grid-template-areas-runtime-2026-10-19/android-avd-c1133-matrix-20261019T033459682Z-49847934/README.md)

## iPhone 17 Pro / iOS 26.2 Simulator

- 최종 실행: `c1133-ios-matrix-20261019T033846431Z-8a7d5acf`, 고정 UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- 같은 26 scene·65 node 결과에서 성공 19, cascade fallback 5, 기대 거부 2, 보이는 frame 61개 최대 오차 0 CSS px다.
- `basic-rectangles` Metal drawable 표본 2/2 exact, 최대 RGBA channel 차이 0이다. readback은 compositor 제출 전이다.
- 앱 실행 파일 SHA-256: `abbe7f5c0c3e06133f6339524efef245d56f6720701e74f7b86d3f20960164a9`.
- [원본 README·result.json·scene log·화면](../c11-3-3-grid-template-areas-runtime-2026-10-19/ios-simulator-c1133-ios-matrix-20261019T033846431Z-8a7d5acf/README.md)

처음 iOS 실행은 runtime에 진입하기 전에 `SpinonRunner.mm`의 C11.2.3 fixture 심볼이 선언되지 않았다는 컴파일 오류로 끝났다. 실행 스크립트가 `SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=1`을 빌드 child process에 전달하지 않은 harness 문제였다. [실패 로그](../c11-3-3-grid-template-areas-runtime-2026-10-19/ios-simulator-c1133-ios-matrix-20261019T033559226Z-a05436b6/failure.txt)를 보존했고, [iOS matrix script](../../../../tools/css-reference/capture-c11-3-3-grid-template-areas-ios-simulator-matrix.mjs)가 feature를 명시하고 build flag를 검사하도록 고친 뒤 외부 환경 설정 없이 최종 실행을 통과했다.

## C11.2.3 dense/column profile on iOS Simulator

- 실행: `c1123-i-mvepifpk-43cc2435`, 같은 iPhone 17 Pro / iOS 26.2 Simulator.
- 고정 Chrome fixture 25 scene·107 node: 성공 19, 기대 거부 6, geometry 최대 오차 0 CSS px.
- 표면 검사는 성공 19 scene·99 표본에서 최대 RGBA channel 차이 0을 기록했다. 각 scene은 새 앱 process와 HostDocument를 사용했다.
- 앱 실행 파일 SHA-256: `abbe7f5c0c3e06133f6339524efef245d56f6720701e74f7b86d3f20960164a9`.
- [원본 README·comparison·scene logs·화면](../c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-19/c1123-i-mvepifpk-43cc2435/README.md)
- 첫 실행은 worktree에서 Node generator가 설치된 `parse5`를 찾지 못해 runtime 전에 중단됐다([실패 기록](../c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-19/c1123-i-mvephwr5-043a4800/failure.txt)). worktree `node_modules/parse5`를 기존 main checkout 설치본에 연결한 뒤 통과했다. 이 ignored symlink는 제품·PR 변경이 아니다.

## 해석 범위

이 검증은 기존 C11.3.3 Grid profile의 Android AVD·iOS Simulator 경로와 C11.2.3 dense/column profile의 iOS Simulator 경로가 Taffy 0.15.0 후보에서 계속 동작함을 보인다. C11.1~C11.3의 모든 독립 profile, Android 실기기·hardware GPU, iOS 실기기·compositor 이후 출력, WPT 또는 전체 CSS Grid 적합성을 대신하지 않는다.

## Fixture ABI 기본·양성 산출물 경계

`rustc 1.96.1 (aarch64-apple-darwin)`의 정적 라이브러리에서 C11.4 fixture 심볼과 장면 payload가 기본 제품 feature에는 빠지고 fixture-test 양성 조합에는 들어가는지 확인했다. 일치하는 LLVM 도구는 같은 toolchain의 `llvm-nm`이다.

| 조합 | 명령 | 산출물 SHA-256 | 바이트 | 관찰 |
|---|---|---|---:|---|
| 제품 runtime, fixture 없음 | `CARGO_INCREMENTAL=0 cargo build --locked -p spinon-ffi --no-default-features --features c04-runtime-gpu` | `b798fee89be12a1f9c1e75fba366343d15e747380cc32c0d2eb41df750971370` | 564,954,832 | `llvm-nm`에서 C11.4 생성·평가 ABI가 없고 `strings`에서 `c11_4_grid_track_sizing_runtime_source`, `auto-fixed-width-item` payload가 없다. |
| fixture-test 양성 대조 | `CARGO_INCREMENTAL=0 cargo build --locked -p spinon-ffi --no-default-features --features c04-runtime-gpu-fixture-test` | `46373a2a28ab4d3276b96b58304f7bd868fb8503d436ee44d3db06ed38d8f6b2` | 568,863,584 | C11.4 생성·평가 C ABI 두 개가 `llvm-nm`에 있고 `c11_4_grid_track_sizing_runtime_source`, `auto-fixed-width-item` 고정 scene 문자열이 `strings`에 있다. |

두 파일은 macOS ARM64 Debug 정적 라이브러리다. 크기 차이는 iOS·Android 앱 bundle 또는 release 증가량을 뜻하지 않는다. 첫 양성 대조 시도는 디스크 공간 부족으로 중단됐고, 캐시를 삭제하지 않은 채 공간을 확보한 뒤 같은 명령을 다시 실행해 통과했다.
