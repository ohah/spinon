# C14.2 이미지 비율 sizing 구현 검증 · 2026-10-14

## 변경

- Optional natural width/height/ratio metadata와 CSS replaced sizing solver를 추가했다. 부분 지정 축, natural/default object size, CSS 2.2 min/max 10개 분기, min-over-max, padding/border와 box-sizing을 계산한다.
- Taffy leaf 측정과 최종 layout에 같은 solver를 연결했다. Flex 부모가 측정한 natural cross-axis 값을 definite CSS size로 오인하던 회귀를 수정했다. Chrome 155 비교에서 definite width + `max-height`는 `48×20`, definite height + `max-width`는 `30×24`다. definite parent cross size의 stretch, max-height 제한, flex-start와 absolute flow/visual pass도 검증했다.
- 범위 밖 ratio-only auto/auto contribution, `aspect-ratio:auto <ratio>`, authored ratio+min/max, percent/calc sizing은 fail-closed한다. C14 상위, decode/runtime integration, WPT 및 GPU 표시 결과는 이 구현으로 완료되지 않는다.

## 검증

- `cargo test --locked --workspace`: 742 passed, 4 ignored, 0 failed. 전체 원본은 [workspace-test.log](workspace-test.log)다.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과. 원본은 [clippy.log](clippy.log)다.
- `cargo fmt --all -- --check`, `git diff --check`: 통과.
- Chrome Headless 155 agent-browser의 일반 fixture 21개 PNG sizing case와 Flex fixture 8개를 CSS px tolerance `0.05`로 HostDocument→Stylo→Taffy 비교했다. 조건·fixture/inventory/observation hashes는 [사전 비교 근거](../c14-2-image-ratio-precomparison-2026-10-14/README.md)에 있다.
- Android API 37 ARM64 AVD에서 layout intrinsic filter 18/18, CSS 2.2 min/max table 1/1을 통과했다. [AVD 근거와 원본 로그](../c14-2-image-ratio-android-avd-2026-10-14/README.md).
- [구현 실패 관점 20개](../c14-2-image-ratio-implementation-review-2026-10-14.md)를 수행했다. Flex natural auto-axis measurement에서 실제 결함을 재현하고 수정 후 브라우저와 AVD에서 재검증했다.

## ADB 환경 복구

- `adb devices -l`에서 `emulator-5554` 한 대만 확인했다. product/model은 `sdk_gphone16k_arm64`, ABI `arm64-v8a`, API 37, page size 16384, `sys.boot_completed=1`이다. 실기기는 연결하지 않았다.
- 앞서 ADB discovery 실패를 만든 원인은 ADB daemon이 아니라 존재하지 않는 `spinon_api37_2.avd` 경로를 가리키며 지원하지 않는 `arm` architecture를 선언한 사용자 AVD 설정이었다. 이번에는 정상 `spinon_api37_1_compat16k` AVD와 명시적인 `-s emulator-5554`를 사용했다. 사용자 AVD 파일은 수정하지 않았다. 상세 재현은 [C14.1 AVD 복구 기록](../c14-1-android-avd-bootstrap-2026-10-14/README.md)에 있다.
- Android 결과는 Rust unit-test executable의 ABI 실행이다. APK, V8, 앱 runtime, Android UI, GPU pixels, 성능 또는 실기기 검증은 아니다.

이 변경은 C14.2 구현 후보이며 아직 stacked PR 검토·병합 전이다. C14.1/C14.2와 상위 C14 완료를 의미하지 않는다.
