# C11.1 parsed Root HTML `calc()` implicit columns runtime

이 기록은 제한된 `grid-auto-columns: calc(length + percentage)`를 기존 Root HTML column-flow fixture-test route에서 확인한 Android/iOS Simulator 결과다. C11.1/C11 전체 완료나 공식 구현 집계 변경을 뜻하지 않으며 공식 완료 `72/213`을 유지한다.

## 입력과 판정값

- 두 플랫폼에서 사용한 source는 [calc precomparison fixture](../../../../tests/fixtures/css/c11/root-html-grid-column-flow-calc-auto-columns-precomparison.html)이며 SHA-256은 `3071fc655beaae8bc7d7ef176893b1bef4844d08b40b442e5091b1c958d7185d`다.
- 기존 C11.1 fixture-test entrypoint가 `RuntimeGpuHost::new_with_root_html`로 이 Root HTML을 읽게 하려고 빌드 중 `root-html-grid-column-flow-fr-auto-columns-runtime.html`의 내용만 임시 교체했다. 두 앱 빌드 후 원본을 복구했고 원래 SHA-256 `d1fe552977c3de7c6107c493ea728c5780c536efc88130c57a3955c630ed4164`를 확인했다. Android/iOS 기본 HTML asset도 원래 동일한 SHA-256 `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`로 복구됐다.
- Grid content width `240px`, height `63px`, explicit columns `[30,40]px`, rows `[20,40]px`, column/row gap `5/3px`, 12개 `10×10px` item을 사용했다. `grid-auto-columns`는 `calc(10% + 5px) calc(2% + 2px)`다.
- 독립 산술 기대값은 implicit tracks `[29,6.8,29,6.8]px`, 전체 columns `[30,40,29,6.8,29,6.8]px`, item column starts `[0,35,80,114,125.8,159.8]px`, row starts `[0,23]px`다.

## Android Simulator

- Android API 37 ARM64 AVD `spinon_api37_0_applied_txn` (`emulator-5556`)에만 설치했다. 실기기 대상 설치는 하지 않았다.
- `SPINON_ENABLE_C04_RUNTIME_GPU=1` 및 기존 `SPINON_ENABLE_C11_1_ROOT_HTML_PERCENT_COLUMNS_FIXTURE_TEST=1`로 native library와 debug app을 빌드했다. 두 빌드는 성공했다. 이 opt-in fixture-test app은 제품 빌드와 구분한다.
- Runtime: `SPINON_C111_ROOT_HTML_PERCENT_COLUMNS_EVAL status=0`, `layout=ready`, `boxes=15`; detailed frame log에서 위 기대 좌표를 모두 확인했다. `SPINON_C0410_DRAW`는 `presented boxes=15`를 기록했다.
- [AVD 화면](android/avd-screen.png) · [runtime log](android/avd-runtime.log).

## iOS Simulator

- iPhone 17 Pro, iOS 26.2 Simulator (`9DCCE836-7659-4914-B6C1-13F54BFFA9DB`)에서 실행했다.
- 같은 C04 runtime 및 C11.1 fixture-test build flags로 simulator app 빌드가 성공했다.
- `SPINON_C0410_ENVIRONMENT`는 최신 화면 요청에서 `status=0`, `layout=ready`, `boxes=15`와 15개 node frame을 기록했다. item의 x/y 좌표는 Android와 같다. 시작 시 도착 순서가 늦은 이전 환경 요청 하나는 `status=-12`로 거부됐고 그 뒤 최신 요청이 성공했다. `SPINON_C0410_DRAW`는 `presented boxes=15`를 기록했다.
- [iPhone 17 Pro 화면](ios/iphone17pro-screen.png) · [runtime log](ios/iphone17pro-runtime.log).

## 검증 경계

- 화면 캡처는 Simulator visual smoke 증거다. Android PixelCopy와 iOS Metal drawable pixel readback을 수행하지 않아 surface pixel parity를 주장하지 않는다. 실기기, hardware GPU, iOS compositor 이후 픽셀 결과도 확인하지 않았다.
- 기존 C11.1 fixture-test route와 opt-in build feature를 재사용했으며 새 FFI/API를 추가하지 않았다. Simulator runtime은 제한 calc fixture 한 장면만 확인했다. scalar, negative clamp, over-100%, zero basis, mixed list, calc+fr와 거부 경계는 별도 focused Rust tests 범위다.
- 변경 crate focused tests는 cascade profile 13/13, projection 10/10, parsed Root HTML calc E2E 6/6, layout column-flow 25/25와 기존 CSS math ceiling 1/1이 통과했다. Clippy와 `cargo fmt --all -- --check`도 통과했다. PR check 또는 CI 통과를 주장하지 않는다.
- [구현/runtime 실패 관점 검토 20개](implementation-runtime-review-20.md)와 [구현 PR diff 실패 관점 검토 30개](implementation-pr-diff-review-30.md).

## Artifact hashes

화면, 로그, fixture, Android APK와 iOS executable SHA-256은 [hashes.txt](hashes.txt)에 기록했다.
