# C12.4 · iOS Simulator 전체 scene runtime matrix 계획

**상위:** [C12.4 쌓임 맥락 계획](c12-4-stacking-context.md) · [상태 대장](../spec/STATUS.md)
**계획 검토:** [20개 실패 관점](../spec/internal/evidence/c12-4-ios-runtime-matrix-plan-review-2026-10-11.md)
**구현 검토:** [행렬 실행·비교 실패 관점](../spec/internal/evidence/c12-4-ios-runtime-matrix-implementation-review-2026-10-11.md)
**실행 결과:** iPhone 17 Pro / iOS 26.2 Simulator에서 26/26 case, 21 supported·5 expected rejection, 지원 node 64개 frame 최대 오차 0 CSS px. [실행 원본](../spec/internal/evidence/c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/README.md) · [비교 JSON](../spec/internal/evidence/c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/comparison.json)

## 목적

Android API 37 AVD에서 실행한 고정 26-scene·77-node C12.4 runtime matrix를 iPhone 17 Pro / iOS 26.2 Simulator에서도 동일 입력·동일 고정 Chromium 154 reference로 실행한다. 이 작업은 iOS runtime geometry와 지원/거부 진단의 교차 플랫폼 비교다. Metal surface pixel, iOS 실기기, WPT 적합성, 전체 CSS 지원을 증명하지 않는다.

## 구현 계약

- 입력은 `build-c12-4-runtime-matrix.mjs`가 생성한 JSON 하나를 쓴다. 인벤토리 SHA-256, case 순서, source, 기대 분류는 Android와 동일해야 한다.
- 전용 iOS probe는 화면 캡처용 GPU demo를 열지 않는다. 고정 iPhone 17 Pro / iOS 26.2 Simulator 앱 프로세스에서 background queue로 각 case마다 C12.4 host를 새로 만든다.
- 각 host에 viewport `320×240 CSS px`, scale `1`, light mode를 설정하고 case source를 평가한다. `defer` 경로로 매 host를 해제한다.
- 21 supported case는 `status=0` 및 `layout=ready`를 모두 요구한다. 5 rejected case는 API 실행 성공을 거부로 오인하지 않고 `op=eval`, 실패 status, 기대 속성/API 진단을 모두 요구한다.
- run ID가 들어간 구조화 JSON을 앱 Documents에 원자적으로 기록한다. capture script는 기존 산출물을 지운 뒤 새 run ID의 결과만 기다리고, 누락·중복·잘못된 run ID는 실패 처리한다.
- 공통 비교기는 Android log와 iOS JSON을 모두 읽으며 기존 26 case 순서, 77 source node, node ID, 각 frame의 x/y/width/height, 최대 절대 오차 `0.5 CSS px`, 거부 진단을 같은 규칙으로 확인한다. 현재 Android 결과와 신규 iOS 결과를 각각 pinned Chromium 154 기준에 비교한다.
- 기록물에는 Simulator UDID·기기명·OS, 앱/기준 입력 hash, case별 report, 비교 결과, 빌드·실행 로그를 남긴다. 화면 이미지나 Metal pixel로 가장하지 않는다.

## 실행 및 완료 기준

`mise exec -- bun run test:css-reference`, `mise exec -- bun run build:ios-sim`의 C04.10 활성화 빌드, iOS Simulator capture, comparator, 비교기 negative tests, Rust workspace test·clippy·fmt 및 문서 링크 검사를 실행한다. Simulator가 고정 UDID/OS가 아니거나 앱 host 생성·환경 설정·평가가 하나라도 실패하면 matrix는 미통과다. 일치 시에도 이 계획만으로 C12.4를 완료 처리하지 않는다.

## 명시적 한계

Simulator 결과는 iPhone 실기기 CPU/GPU·메모리 결과가 아니다. frame 비교는 CSS geometry만 검증하며 native surface pixel, WPT testharness, CSS 규격 전체 적합성을 검증하지 않는다. 다른 플랫폼의 하드웨어 renderer 증거와 합산하지 않는다.
