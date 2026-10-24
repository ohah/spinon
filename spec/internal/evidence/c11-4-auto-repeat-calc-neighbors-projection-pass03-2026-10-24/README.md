# C11.4 auto-repeat `calc()` neighbor 현재 소스 runtime · 2026-10-24

## 실행 결과

현재 작업 트리의 typed projection 경계와 typed runtime error mapping을 포함한 14-scene opt-in fixture를 양 플랫폼에서 실행했다. 각 플랫폼에서 11 positive scene의 frame·used tracks/collapse·trace·draw/present가 11/11 통과했고, 세 fail-closed control도 정확한 code·property·node와 frame 부재를 확인해 14/14를 통과했다.

- Android API 37 ARM64 AVD `emulator-5558`: `PixelCopy` 표본 24/24 exact.
- iPhone 17 Pro / iOS 26.2 Simulator `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`: compositor 제출 전 Metal drawable 표본 24/24 exact.
- `calc-neighbor-col-fill-overflow`: 240px root 바깥 x=244에서 시작하는 2×2 item과 root 내부·외부 경계 포함 표본을 두 플랫폼에서 각각 5/5 exact로 확인했다.
- rejection controls: `extra-term`과 negative-percentage-term은 `unsupported_computed_value` / `grid-template-columns` / node 1, `max-content`는 `invalid_grid_track_list` / `grid-template-columns` / node 1이다.

이들은 고정 fixture의 simulator 결과다. 실기기, Android hardware GPU, iOS compositor 제출 이후 pixels, 일반 CSS admission, WPT 또는 전체 C11.4 적합성을 뜻하지 않는다.

## 빌드와 재현

두 앱은 다음 fixture-test 옵션으로 빌드했다.

```sh
SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 \
SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 \
mise exec -- bun run build:android

SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 \
SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 \
mise exec -- bun run build:ios-sim
```

원본 build/capture log는 [`android-build.log`](./android-build.log), [`ios-build.log`](./ios-build.log), [`android-capture.log`](./android-capture.log), [`ios-capture.log`](./ios-capture.log)에 있다. 플랫폼별 `result.json`, `simulator.json`, scene log 및 screenshots는 같은 이름의 하위 directory에 보존했다. 전체 실행 artifact는 [SHA256SUMS](./SHA256SUMS)로 검증할 수 있다.

## 소스와 남은 artifact 경계

두 `simulator.json`은 commit `81ad15c7f502c82494bc0f6fdad38ec72307d51d`와 tracked diff SHA-256 `0f6c3897613af55d199643fa855e573a3b077528dfa039c9b4b79e417c92726a`를 기록한다. untracked Rust fixture/test 입력의 해시는 [`source-files.sha256`](./source-files.sha256)에 별도 고정했다. 앱 SHA-256은 Android `c13ffe4596595e450cb7b292dced4dccbc26a994664620e292e3397d3a63d7c5`, iOS `5598f15a6f16005dddfb0202fd4ba9f9c1ab03df60b7a21e7c3d736aa17acedb`다.

기본 product FFI archive에서 fixture symbol/string/payload가 빠지는지는 아직 확인하지 않았다. 이 작업 트리는 미커밋이며 PR/merge가 없다. C11.4 상위 체크는 미완료이고 공식 집계 `72/213`은 유지한다.
