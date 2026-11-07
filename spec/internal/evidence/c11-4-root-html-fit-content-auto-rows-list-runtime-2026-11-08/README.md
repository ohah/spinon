# C11.4 Root HTML fit-content auto-row 목록 실행 근거

## 후보와 비교 입력

- 기준 HEAD: `e788203c304b72473f5b43d3195646dc12373e4e` (`origin/main`과 동일)
- 구현 patch SHA-256: `530dbbd943757df59a932c296fa27d31dab141fd7707eecedaa1d6b3fcc9ca81` (Simulator build 시 tracked code diff)
- Simulator build 뒤에는 `#[cfg(test)]` Rust 파일만 책임별 하위 모듈로 나누고 빈 목록·unsupported numeric negative assertion을 추가했다. 제품 Rust 구현 파일은 바뀌지 않아 두 앱의 실행 결과와 fixture hash는 같은 제품 코드에 해당한다. 최종 분할 소스의 focused test log는 아래 실행 근거 디렉터리에 갱신했다.
- runtime HTML fixture: `tests/fixtures/css/c11/root-html-grid-auto-rows-fit-content-list-runtime.html`
- fixture SHA-256: `c6f9f0244e29c3f74164d00560c0d40ea7ca7f3f594ed01cd3a6daa3dc505031`
- 사전 비교는 Chrome 157과 독립 계산의 6 scene 결과를 [precomparison](../c11-4-root-html-fit-content-auto-rows-list-precomparison-2026-11-08/README.md)에 고정했다. CSS 입력은 definite 40×100px Grid content box이고 percentage basis는 100px block size다.
- 허용 track은 finite non-negative `fit-content(px|%)`, fixed-px, direct percentage다. 목록 순서·항목 종류와 첫 implicit row 위치를 보존한다. 일반 CSS admission이나 full C11.4 support로 확대하지 않는다.

## Android API 37 AVD

`mise exec -- bun run build:android`가 `BUILD SUCCESSFUL`로 끝났다. APK 안의 `assets/index.html` hash는 고정 fixture와 일치했고 `root-app.js`는 같은 Root HTML session의 `document.body`에서 `setAttribute`를 수행했다. 기본 앱 실행에서 Android PID `14892`가 다음 결과를 기록했다.

```text
SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass root_app_marker=ready status=0 layout=ready boxes=12 generation=2 document_revision=2 environment_revision=0
```

- 기기: `sdk_gphone64_arm64`, API 37, `emulator-5554`
- 후보 APK SHA-256: `33d9c6e7725c369cd4cba545b05e2595c4c6a909a5fbb1359c36c59463321e26`
- `assets/root-app.js` SHA-256: `110a40fda95b309631082bd634c09f9a0f055b1d0ae525316cba955bb04086e0`
- 화면: [Android Simulator](./simulator/android/screenshot.png) · [boot log](./simulator/android/current-source-run.log) · [device details](./simulator/android/device-and-run.txt)

## iOS Simulator

`mise exec -- bun run build:ios-sim`가 `** BUILD SUCCEEDED **`로 끝났다. 빌드된 `index.html`은 fixture hash와 일치했다. iPhone 17 Pro/iOS 26.2 Simulator에서 PID `43304`가 같은 기본 앱 boot marker와 `layout=ready boxes=12`를 기록했다.

- Simulator: `ACA7BF91-D144-4DA8-B1BF-7F6B34007F8B`
- 후보 executable SHA-256: `b66c88f4ac42c45fa236f4e3c94d68cfd922366c63e9206f71748210cc214e80`
- `index.html` SHA-256: `c6f9f0244e29c3f74164d00560c0d40ea7ca7f3f594ed01cd3a6daa3dc505031`
- `root-app.js` SHA-256: `110a40fda95b309631082bd634c09f9a0f055b1d0ae525316cba955bb04086e0`
- 화면: [iOS Simulator](./simulator/ios/screenshot.png) · [boot log](./simulator/ios/root-html-boot.log) · [device details](./simulator/ios/device-and-run.txt)

## 실행 과정에서 확인한 경계

첫 Android build는 Gradle `prepareSpinonBootstrap` 단계에서 `examples/bootstrap`을 다시 bundle해 generated asset만 바꾼 fixture를 무시했다. 이전 화면·log는 [attempt 01](./simulator/android/attempt-01-asset-mismatch/)에 보존했다. fixture source를 일시적으로 app Bootstrap input에 적용한 뒤 빌드해야 실제 package에 들어간다.

두 번째 실행은 fixture HTML을 사용했지만 Root HTML JavaScript API에 `document.getElementById`가 없어 `TypeError`로 멈췄다. 해당 log와 화면은 [attempt 02](./simulator/android/attempt-02-dom-api-mismatch/)에 보존했다. script를 지원되는 `document.body`와 `setAttribute`로 바꾼 뒤 Android와 iOS 실행이 통과했다.

전용 C11.4 AVD를 시작했지만 adb console/device endpoint가 열리지 않아 그 프로세스를 종료했다. 기존 Android API 37 AVD와 앱이 실행 중인 iPhone 17 Pro Simulator를 사용하지 않도록 다른 booted iPhone 17 Pro Simulator를 골라 확인했다. Android는 기존 `emulator-5554` AVD를 사용했으며 실기기 검증은 아니다.

두 화면은 앱 기본 Root HTML route의 **visual smoke**다. `layout=ready`와 box count는 route가 실행된 사실을 보이며, 화면은 PixelCopy/Metal surface readback이나 compositor 이후 pixel parity를 증명하지 않는다. Track size와 item frame은 focused Rust/Layout/RuntimeSession 검사가 판정한다. iOS build log의 기존 V8 debug-map 경고는 있었지만 Xcode 결과는 `BUILD SUCCEEDED`이며 runtime boot marker도 통과했다.

## 복원 확인

Simulator test를 위해 `examples/bootstrap/index.html`과 `root-app.js`를 일시 교체했다. 실행 뒤 두 파일은 각각 원본 hash `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`, `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`로 복원했다. generated Bootstrap files, Android APK/output metadata와 기존 iOS app bundle도 [run manifest](./simulator/run-manifest.txt)에 기록된 원래 hash로 복원했다. Android에는 원래 설치 APK를 다시 설치하고 앱을 재시작했으며 iOS에는 기존 app bundle을 다시 설치했다. 앱의 저장 데이터 내용은 조사하지 않았다.

임시 백업은 자동 검토가 파일 제거 명령을 차단해 삭제하지 않고 보존했다. 정확한 경로는 [backup-location.txt](./simulator/backup-location.txt)다.

## 범위와 상태

Focused implementation/runtime failure review는 [20개 항목](./implementation-runtime-review-20.md)이다. 별도의 PR diff 검토 30개, PR merge, merge 이후 roadmap/Tailnet 동기화는 아직 완료되지 않았다. C11.4/C11 checkbox와 공식 `78/214`는 미완료 상태로 유지한다.
