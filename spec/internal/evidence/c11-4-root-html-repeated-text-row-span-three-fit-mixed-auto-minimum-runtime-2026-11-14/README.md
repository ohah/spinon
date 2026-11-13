# C11.4 three-fit mixed automatic minimum · 앱 화면 확인

## 범위

Chrome 157 대표 장면 54,448(mask 16)을 C04.12 Root HTML 실제 앱 경로에 넣어 Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 확인했다. 이 결과는 화면 smoke이며 브라우저 parity, Metal/PixelCopy readback 또는 전체 C11.4 완료 판정이 아니다.

fixture HTML SHA-256은 `c2c15cfb1454d80ee56d2d19d945c962dd3aef2acde889fb5637a9c1bf7a874b`다. 두 플랫폼 로그가 앱이 읽은 source hash, `layout=ready`, draw/present 성공을 기록한다.

## 결과

| 플랫폼 | 빌드·실행 | 화면 |
| --- | --- | --- |
| Android API 37 ARM64, `spinon_api37_2_presenttime_probe_20261018`, SwiftShader | [빌드](./android-build.log) · [실행 로그](./android-runtime.log) · layout-ready, 14 boxes, 5 text draws, presented | [캡처](./android-mask16.png) |
| iPhone 17 Pro, iOS 26.2 Simulator | [빌드](./ios-build.log) · [실행 로그](./ios-runtime.log) · layout-ready, 14 boxes, 5 text draws, presented | [캡처](./ios-mask16.png) |

빌드는 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`을 사용했다. focused RuntimeSession 테스트는 같은 fixture의 `RenderSnapshot`이 Grid 네 변 모두 1 CSS px opaque RGB `(229,72,77)` border를 포함하고, Chrome geometry와 D payload의 10px overflow 및 E의 automatic-minimum floor를 만족하는지 확인한다. [최종 테스트 로그](./simulator-fixture-snapshot-check-pass02.log).

스크린샷에는 빨간 테두리와 경계를 넘어가는 payload가 보인다. 시각 캡처는 surface readback이 아니며 색상·pixel 단위 parity를 주장하지 않는다. 이미지 해시는 [manifest](./screenshot-sha256.txt)에 있다.

## 런타임·인접 회귀

| 범위 | 결과 | 로그 |
| --- | --- | --- |
| three-fit mixed Chrome RuntimeSession 표본 | 6,912/6,912 통과, 628.86초 | [행렬 표본](./runtime-matrix-test-pass02.log) |
| 기존 three-fit uniform endpoint | 11,520/11,520 통과 | [uniform 행렬](./three-fit-uniform-runtime-matrix.log) |
| three-fit auto-placement | 5,760 scenes 통과 | [회귀](./three-fit-auto-placement-regression.log) |
| one-fit row-span | 2,880 pairs 통과 | [회귀](./one-fit-row-span-regression.log) |
| two-fit row-span | 3,840 scenes 통과 | [회귀](./two-fit-row-span-regression.log) |
| four-fit mixed | 3,840 scenes 통과 | [회귀](./four-fit-mixed-regression.log) |
| five-fit mixed | 3 chunks, 합계 3,984 scenes 통과 | [1](./five-fit-mixed-chunk01.log) · [2](./five-fit-mixed-chunk02.log) · [3](./five-fit-mixed-chunk03.log) |
| six-fit mixed | metadata가 oracle 138,240 scenes를 확인; rejection/recovery와 첫 11,520-scene chunk 통과 | [metadata](./six-fit-metadata-regression.log) · [거부/복구](./six-fit-rejection-regression.log) · [chunk 01](./six-fit-chunk01-regression.log) |
| simulator RenderSnapshot fixture | 1 test 통과: red border 네 변, Chrome geometry, D overflow, E floor | [최종 focused test](./simulator-fixture-snapshot-check-pass02.log) |

RuntimeSession은 three-fit mixed 전체 oracle 138,240개를 실행하지 않았다. six-fit도 metadata 전체 검증과 한 11,520-scene chunk만 실행했다. 각 로그의 실제 scene 분모가 지원 범위다.

## 정적 검사

- `cargo fmt --all -- --check`와 `git diff --check` 통과: [format](./format-check-final-pass.log) · [diff](./git-diff-check-final.log).
- 캡처된 text log 20개는 줄 끝 공백과 EOF의 반복 빈 줄만 정리했고 나머지 로그 내용은 유지했다. [정리 기록](./log-whitespace-normalization.txt).
- style-to-render 기본/fixture, style-to-layout fixture, spinon-layout all-targets, spinon-runtime fixture lib Clippy가 `-D warnings`로 통과: [render 기본](./clippy-style-to-render.log) · [render fixture](./clippy-style-to-render-fixture.log) · [style-to-layout](./clippy-style-to-layout-fixture.log) · [layout](./clippy-layout-final.log) · [runtime lib](./clippy-runtime-fixture-lib.log).
- Runtime 전체 all-targets Clippy는 현재 변경과 무관한 기존 경고 6개로 실패했다. `anonymous_text_hyphens.rs:117`, `anonymous_text_text_transform.rs:507`의 argument count 2개와 변경되지 않은 five-fit matrix의 needless borrow 4개다. [로그](./clippy-runtime-fixture-all-targets-blocked.log).

## 정리와 복원

- 임시 iOS Simulator `Spinon-C11.4-mask16-20261114`는 캡처 뒤 종료하고 삭제했다.
- Android AVD는 `-read-only -no-snapshot-load -no-snapshot-save`로 실행하고 종료했다.
- Android/iOS Root HTML 자산은 원본 SHA-256 `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`와 byte-for-byte로 복구했다. [복원 기록](./asset-restoration.txt).
- 실행 전 존재하던 APK와 iOS 앱 executable도 기존 SHA-256으로 복구했다. [빌드 산출물 복원 기록](./build-artifact-restoration.txt).
- 최종 확인 때 booted Simulator, ADB device, 이 작업의 앱·빌드 process는 없었다.
- 작업용 iOS Simulator는 삭제했고 read-only Android AVD는 중지했다. 공용 CoreSimulator/ADB 서비스와 다른 workspace의 기존 브라우저 세션은 이 작업 자원이 아니어서 종료하지 않았다.

## 계약·검토

- 내부 계약 candidate는 [0059 C11.4 절](../../0059-c11-grid-layout.md)에 있다. public API/FFI와 numeric contract version은 바꾸지 않는다.
- 계획 검토 20개와 [구현/runtime 검토 20개](../../../../plan/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-implementation-review-20.md)를 마쳤다. 이 candidate의 [PR diff 실패 관점 30개](./pr-diff-review-30.md)도 별도로 기록했다.
- 공식 구현 상태 `78/214`와 C11.4/C11 미완료는 유지한다.
