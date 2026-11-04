# C11.1 Root HTML one-axis placement runtime evidence

## 판정

같은 parsed Root HTML fixture의 Android API 37 AVD SurfaceView PixelCopy 표본 6/6과 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable 표본 6/6이 Chrome 156 expected RGBA와 exact match다. 기본 Android APK와 iOS Simulator app의 artifact 경계도 opt-in flag를 끈 빌드에서 확인했다. 이 evidence는 현재 checkout의 제한 구현 후보를 검증하며 PR 병합이나 C11.1/C11 완료를 뜻하지 않는다. 공식 완료 `72/213`은 유지한다.

## Rust 검증 및 구현 검토

- Focused layout 5/5, style-to-layout 6/6, RuntimeSession 2/2가 통과했다. 결과는 [`one-axis-layout-tests.log`](one-axis-layout-tests.log), [`one-axis-projection-tests.log`](one-axis-projection-tests.log), [`one-axis-runtime-tests.log`](one-axis-runtime-tests.log)에 있다.
- sparse/dense placement regression 77/77은 [`layout-auto-placement-suite.log`](layout-auto-placement-suite.log)에 있다. scoped `spinon-layout`, `spinon-style-to-layout`, `spinon-runtime` all-target Clippy와 `cargo fmt --all -- --check`는 [`rust-scoped-clippy.log`](rust-scoped-clippy.log), [`rustfmt-check.log`](rustfmt-check.log)에 있다.
- 구현/runtime 실패 관점 검토 20개는 [`implementation-runtime-review-20.md`](implementation-runtime-review-20.md)에 기록했다. 실제 PR diff 적대 검토 20개는 PR 생성 후 별도로 수행한다.

## Android PixelCopy

- 대상은 `emulator-5554`, API 37 ARM64 AVD `sdk_gphone64_arm64`, 1080×2400이다. `ro.kernel.qemu=1`이며 실기기는 사용하지 않았다.
- opt-in simulator APK 빌드는 [`android-pixelcopy-build-pass03.log`](android-pixelcopy-build-pass03.log)와 [`android-pixelcopy-fixture-package-pass05.log`](android-pixelcopy-fixture-package-pass05.log)에서 성공했다. APK SHA-256은 `c63b234beb4022c2f80b51e35556d67f65677226f8ffc6cd6cbdf02d4a0e656a`다. opt-in service provider와 Java PixelCopy adapter class가 APK에 들어갔고 staged fixture HTML hash는 `93cf59bc6aafd00850e717be550d7560fda21595e66178683dc5769603846724`다. Android adapter는 FFI surface-pixel symbol을 호출하지 않고 framework PixelCopy를 사용한다.
- [`android-pixelcopy-run-pass04.log`](android-pixelcopy-run-pass04.log)은 기본 Root HTML `index.html` route, layout-ready boxes 8, draw-presented, viewport 411.42856×841.52374 CSS px, surface 1080×2209 physical px를 기록한다. ANGLE/Vulkan/SwiftShader software renderer다.
- PixelCopy가 읽은 [`android-surface-pixelcopy-pass02.png`](android-surface-pixelcopy-pass02.png)는 SHA-256 `f33a7e89efe46f08ab3bb20f85edab22d66db456483c9d2cd2f31a533d693db4`다. 독립 PNG 재검사도 red column-first, visibility-hidden occupancy, blue column-last, orange row-first, purple row-second, grid 밖 white 표본을 모두 exact로 읽었다.
- 별도 OS screenshot [`android-pixelcopy-screen.png`](android-pixelcopy-screen.png)의 SHA-256은 `3af1d1f83ef438f782414230a992f1b1629efe2a07b07c0694d0eab6116d2fd1`이다. 이 screenshot은 PixelCopy image와 구분한다.

## iOS Metal drawable

- 대상은 iPhone 17 Pro, iOS 26.2 Simulator, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`다.
- [iOS Simulator build 요약](ios-simulator-build-summary-pr-review.txt)에서 opt-in flag와 raw Xcode log hash, build 성공을 확인했다. [`ios-metal-runtime.log`](ios-metal-runtime.log)는 fixture bytes, 기본 `index.html` route, layout-ready boxes 8, draw-presented를 기록한다. viewport는 402×778 CSS px, drawable은 1206×2334 physical px, scale은 3이다.
- Metal drawable 표본 6/6이 exact match다. sample별 RGBA와 revision/generation identity는 runtime log에 남겼다. 이는 OS compositor 제출 전 drawable texture의 readback이다.
- 별도 OS screenshot [`ios-metal-screen.png`](ios-metal-screen.png)의 SHA-256은 `0c6b6fc6b2072b6bbbe189da9e3c3ed41f7bfba1942486739e1f3cf35adda82c`다.

## 기본 artifact 경계

- 기본 Android 빌드 [`android-default-artifact-build-pr-review.log`](android-default-artifact-build-pr-review.log)는 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=0`에서 성공했다. APK SHA-256은 `7df7c66a34be418c21ec6cfd3c91ed241bb8cfcbd25b315d9e79e3dc8d5813f1`다. [기본 artifact 재검사](artifact-boundary-pr-review.log)에서 APK DEX·assets·ARM64 library에 C11 one-axis marker·intent key·PixelCopy adapter class·fixture payload·dedicated C11 surface-pixel FFI symbol이 없고, opt-in ServiceLoader provider resource도 제외됨을 확인했다. 공통 ServiceLoader 인터페이스·dispatch는 기본 DEX에 포함되며 opt-in provider가 없으면 C11 캡처를 등록하지 않는다. library SHA-256은 `eb65c6ad482008312d352d3001ed81359ceaa9f19b3efad824c48e4258a52f62`다.
- 기본 iOS build는 [iOS Simulator build 요약](ios-simulator-build-summary-pr-review.txt)의 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=0` 기록에서 성공을 확인했다. app executable SHA-256은 `ea623decb9b14526470a71063263ff09a9b621f199fab2f132b1189ab3fa2a60`, 기본 `index.html` SHA-256은 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`다. app bundle 전체 8개 파일을 검사했고 one-axis fixture marker와 dedicated C11 surface-pixel FFI symbol은 발견되지 않았다([artifact 재검사](artifact-boundary-pr-review.log)).
- opt-in positive controls는 Android PixelCopy와 iOS Metal 실행 log의 6/6 결과다. simulator fixture asset은 ignored build asset/app bundle에만 수동 배치했으며 저장소의 기본 앱 asset을 바꾸지 않았다.

## 남은 범위

- Android 결과는 SwiftShader AVD이며 Android 실기기/hardware GPU 근거가 아니다. iOS는 compositor 이후 화면 픽셀을 읽지 않았다.
- 구현/runtime 실패 관점 검토 20개와 실제 PR diff 검토 20개, 최종 PR/merge 확인은 아직 남아 있다.
- 이 제한 후보는 C11.1/C11을 완료 처리하지 않으며 공식 `72/213`도 바꾸지 않는다.
