# C12.2 Block absolute · 실행 근거

## 비교 대상

- Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`의 고정 reference와 동일한 V8 JavaScript fixture를 비교했다.
- fixture는 23개 case, 82개 DOM 요소다. 앱 런타임이 삽입한 `<style>` 요소 한 개는 화면 상자가 없어 0×0 frame으로 별도 확인한 뒤 82개 비교 대상에서 제외한다.
- viewport는 360×800 CSS px다. 판정은 각 DOM 요소의 `x`, `y`, `width`, `height`를 node ID와 고정 순서로 대조하며 허용 오차는 field별 0.5 CSS px다.
- CSS 기준은 DPR 1·2로 캡처했다. 모바일 runtime geometry 로그는 CSS px이며 surface pixel과 혼합하지 않는다.

## Android 실기기

- 기기: Samsung SM-S731N, Android 16 / API 36.
- wgpu surface log: Vulkan, Samsung Xclipse 940, 900×675 surface texture.
- 결과: `layout=ready`, 80개 paint box, 83개 raw frame, WGPU `presented boxes=80`.
- Chromium reference와 비교: 82/82 node 일치, 최대 오차 `0.0125 CSS px` (허용치 `0.5 CSS px`).
- [Android 화면](./android-physical.png) · [원시 로그](./android-physical.log)

## iOS Simulator

- 기기: iPhone 17 Pro Simulator, iOS 26.2, Xcode 26.2.
- surface texture: 1080×2400, CSS viewport: 360×800.
- 결과: `layout=ready`, 80개 paint box, 83개 raw frame, WGPU `presented boxes=80`.
- Chromium reference와 비교: 82/82 node 일치, 최대 오차 `0.0125 CSS px` (허용치 `0.5 CSS px`).
- [iOS 화면](./ios-simulator.png) · [원시 로그](./ios-simulator.log)

## 구현 검토에서 수정한 결함

iOS 미리보기의 첫 실행은 360×800으로 계산한 뒤 `viewDidLayoutSubviews`가 320×240 canvas bounds를 다시 환경에 적용했다. 이 때문에 초기 frame 로그는 기준과 같아도 최종 WGPU 장면의 viewport는 달랐다. C12.2 fixture의 canvas와 환경 입력을 함께 360×800으로 고정하고, 비교 도구가 최종 environment 및 `presented` 로그도 요구하도록 수정한 뒤 iOS 앱을 재빌드·재설치해 비교를 다시 실행했다.

## 실행한 검사와 한계

- `mise exec -- cargo test --locked --workspace --all-features` 통과. V8 호스트 바인딩이 필요한 C12.2 FFI 단위 테스트 한 개는 호스트 초기화 불가 사유로 `ignore`; 실제 Android/iOS 앱 경로에서 대신 실행했다.
- `mise exec -- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` 통과.
- `mise exec -- bun run test:css-reference` 통과: 152 tests.
- `mise exec -- bun test tools/css-reference/c12-2-absolute-block.test.mjs` 통과: 11 tests.
- WPT suite는 실행하지 않았다. RTL, 논리 inset, Grid/Flex static-position, text/replaced intrinsic sizing, paint order·hit-test·scroll/clip은 이 제한 구현의 완료 범위가 아니다.
- Android 실기기 좌표 비교는 V8→Stylo→Taffy→wgpu 경로를 확인하지만 전체 WPT/브라우저 적합성, 픽셀 동일성, 성능 결과를 주장하지 않는다.
