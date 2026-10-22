# C11.4 cyclic `minmax(calc())` replaced image multi-span runtime 근거

이 근거는 고정된 intrinsic/definite Grid 네 장면의 구현과 최신 tracked source revision 검증을 기록한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 장면이 모두 통과했고, 이 변경의 PR 적대적 검토도 기록했다. 이 결과는 C11.4 전체 지원이나 실기기 동작을 뜻하지 않는다.

## 빌드와 focused checks

- `mise exec -- cargo test --locked -p spinon-style-to-layout --features c11-4-grid-track-sizing-fixture-test cyclic_minmax_replaced_image_spans_match_all_four_fixed_scenes -- --nocapture`: 1/1 통과
- `mise exec -- cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test fixes_four_scene_ids_and_reverse_order_by_item_identity -- --nocapture`: 1/1 통과
- `mise exec -- cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test exposes_four_fixed_cyclic_minmax_replaced_image_span_scenes -- --nocapture`: 1/1 통과
- `mise exec -- bun test tools/css-reference/fixture-module-boundary.test.mjs`: 18/18 통과
- `mise exec -- cargo fmt --all -- --check`, `git diff --check`, iOS Simulator build, Android build: 통과
- Android Java harness 컴파일과 API 37 AVD의 네 scene capture: 통과. 허용 장면 ID, reversed-order intrinsic input, track trace, node frame, PixelCopy 위치·RGB 검증을 기존 runtime 결과와 맞췄다.
- [artifact-boundary.log](./build/artifact-boundary.log): default `spinon-ffi` static archive에는 C11.4 fixture FFI symbol/scene payload가 없고, `c04-runtime-gpu-fixture-test` static archive에는 둘 다 있다. 정적 archive 결과이며 APK/IPA 또는 release artifact 전체를 증명하지 않는다.

## iOS 26.2 Simulator

- 장치: 전용 iPhone 17 Pro, UDID `00D4D861-56AA-4552-9181-58D88770EDDA`. 캡처 드라이버의 모델 검증 동안만 이름을 `iPhone 17 Pro`로 맞추고 실행 뒤 `Spinon-C114-Cyclic-Image-Span`으로 복구했다.
- 실행: `c114-cyclic-minmax-replaced-image-span-ios-simulator-20261022T170413789Z-d3e5cc33`
- source revision: checkout `2c1a06e959e3d81c16c147d7c9e68af7904d5792`, tracked diff SHA-256 `3b761f6db06f2153b67967874eaf6dd3e9038aa1a7906029cf680ef0b69522e7`
- 앱 SHA-256: `b16a25f4879eb69e9ad09f97bf406bb5da75e543743e70f32de6a9ba7601f08f`
- 네 장면에서 track trace, root-relative node frame, `drawPresented=true`를 확인했다. Intrinsic tracks는 columns `[5,8,8]`, rows `[7,7]`, root `29×20px`; definite tracks는 columns `[10,10,10]`, rows `[10,10]`, root `84×84px`다.
- Metal drawable 표본은 각 scene에서 3/3 통과했다. 이는 compositor 제출 전 표본이다. 전체 화면 pixel 일치, iOS 실기기 및 compositor 이후 출력은 확인하지 않았다.

| Scene | Screenshot | SHA-256 |
| --- | --- | --- |
| Intrinsic forward | [PNG](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/ios-simulator-c114-cyclic-minmax-replaced-image-span-ios-simulator-20261022T170413789Z-d3e5cc33/cyclic-minmax-replaced-image-span-intrinsic-forward.png) | `d65c594cb9fb60b85ef12a5545458961d9630042bae5319c387ed3e46d6e95e8` |
| Intrinsic reverse | [PNG](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/ios-simulator-c114-cyclic-minmax-replaced-image-span-ios-simulator-20261022T170413789Z-d3e5cc33/cyclic-minmax-replaced-image-span-intrinsic-reverse.png) | `1ae7b3b77fbd8a66eb348b588b27ebb36e54fe9b6bd3c1579cb3c26fe2026538` |
| Definite forward | [PNG](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/ios-simulator-c114-cyclic-minmax-replaced-image-span-ios-simulator-20261022T170413789Z-d3e5cc33/cyclic-minmax-replaced-image-span-definite-forward.png) | `bf2420ce86342ca5580bef0885c69501d145a92db542104ca1da0e6ea032c898` |
| Definite reverse | [PNG](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/ios-simulator-c114-cyclic-minmax-replaced-image-span-ios-simulator-20261022T170413789Z-d3e5cc33/cyclic-minmax-replaced-image-span-definite-reverse.png) | `d273cf3f478c4bcde187014829aba5a8881ec27ea8cda7b06c8c7a4e6b4f934d` |

원본 run metadata는 [result.json](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/ios-simulator-c114-cyclic-minmax-replaced-image-span-ios-simulator-20261022T170413789Z-d3e5cc33/result.json), [simulator.json](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/ios-simulator-c114-cyclic-minmax-replaced-image-span-ios-simulator-20261022T170413789Z-d3e5cc33/simulator.json)과 같은 디렉터리의 per-scene runtime log다.

## Android API 37 ARM64 AVD

- 전용 AVD `spinon_c114_cyclic_image_span_20261023`, serial `emulator-5562`; API 37, ARM64, Android 17, QEMU. 실기기는 연결하지 않았다.
- 실행: `c114-cyclic-minmax-replaced-image-span-android-avd-20261022T170248317Z-82130ca8`
- source revision은 iOS run과 동일하다. APK와 설치 APK SHA-256은 모두 `80a0a70d93ae7d6e55a0ac59ad863b09d65855cb7cfb9aaa19e9ac1f9f15352b`다.
- 네 장면에서 expected track trace, frame, `drawPresented=true`가 모두 통과했다. Intrinsic tracks는 columns `[5,8,8]`, rows `[7,7]`, root `29×20px`; definite tracks는 columns `[10,10,10]`, rows `[10,10]`, root `84×84px`다.
- Surface PixelCopy 표본은 각 scene에서 3/3 exact match다. image interior `255,255,64`, root border `240,198,116`, root 바깥 `0,0,0`을 확인했다.
- 첫 ADB 실패는 Spinon 앱 설치 후 문제가 아니라 전용 AVD 설정 문제였다. AVD descriptor가 API 37 system image와 달리 `android-0`을 가리켰고 RAM 1.5 GB, userdata 800 MB 및 `-accel off` TCG 조합으로 system-server watchdog이 boot 완료를 막았다. 설정을 `android-37.0`, RAM 4 GB, userdata 10 GB로 맞추고 macOS HVF를 사용하는 `-accel auto -gpu host`로 재기동해 `sys.boot_completed=1`을 확인했다. 이후 설치 hash가 일치하고 네 장면 캡처가 통과했다.
- 수정 전 AVD 설정 백업, 최종 앱 build/capture와 artifact-boundary 근거는 [build evidence](./build/)에 있다. ADB boot 실패 원인과 설정 수정은 위에 요약했다. 전용 AVD는 보존하며 다른 AVD와 물리 Android 기기는 변경하지 않았다.

| Scene | Screenshot | SHA-256 |
| --- | --- | --- |
| Intrinsic forward | [PNG](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/android-avd-c114-cyclic-minmax-replaced-image-span-android-avd-20261022T170248317Z-82130ca8/cyclic-minmax-replaced-image-span-intrinsic-forward.png) | `8bd24fb7732ef83fd4fa5258ac3f40a1084c7bdfc09ad70e0ba58d2a85065acd` |
| Intrinsic reverse | [PNG](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/android-avd-c114-cyclic-minmax-replaced-image-span-android-avd-20261022T170248317Z-82130ca8/cyclic-minmax-replaced-image-span-intrinsic-reverse.png) | `64ad72823171b14940f1fe713b7a0e5734f37d36c7bcc02b64f2e9a4b54b27ca` |
| Definite forward | [PNG](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/android-avd-c114-cyclic-minmax-replaced-image-span-android-avd-20261022T170248317Z-82130ca8/cyclic-minmax-replaced-image-span-definite-forward.png) | `283d0621c70ed083da31b2a0dbb0727de612d4d34f6cea1f393dc8a227d68f1c` |
| Definite reverse | [PNG](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/android-avd-c114-cyclic-minmax-replaced-image-span-android-avd-20261022T170248317Z-82130ca8/cyclic-minmax-replaced-image-span-definite-reverse.png) | `c41e016c0a2129a3fc0c7cbbbc84863abfe4bc24ee4a2bc15c9861ccd7e9b53d` |

원본 run metadata는 [result.json](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/android-avd-c114-cyclic-minmax-replaced-image-span-android-avd-20261022T170248317Z-82130ca8/result.json), [simulator.json](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/android-avd-c114-cyclic-minmax-replaced-image-span-android-avd-20261022T170248317Z-82130ca8/simulator.json), [설치 APK hash](../c11-4-cyclic-minmax-replaced-image-span-2026-10-23/android-avd-c114-cyclic-minmax-replaced-image-span-android-avd-20261022T170248317Z-82130ca8/installed-apk-verification.json) 및 scene별 runtime log다.

## 실패 관점 검토와 상태 경계

- [runtime 실패 경로 검토 20개](./runtime-code-review-20.md), [구현 diff 검토 20개](./pr-diff-review-20.md)와 [PR diff 적대적 검토 30개](./pr-review-30.md)를 계획 검토와 분리했다. 앞선 검토에서 root min/max/aspect-ratio style 검증 및 Android host 계약 누락을 찾아 fail-closed 검증을 보강했다. 구현 PR [#520](https://github.com/ohah/spinon/pull/520)은 2026-10-23 02:21:48 KST Rebase and merge되어 `861f69862de34858d522b380f25b52dc5310ef5d`를 남겼다. 병합 시 PR check rollup은 비었고 main required status check은 설정되어 있지 않았다.
- 더 넓은 C11.4 scene과 임의 CSS/image 입력, 실기기 및 전체 제품 범위는 남아 있다. 이 slice는 C11.4 checkbox와 공식 `77/213`을 변경하지 않으며 상위 상태를 완료 처리하지 않는다.
