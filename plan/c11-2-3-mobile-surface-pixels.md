# C11.2.3 · Android/iOS 표면 픽셀 검증 계획

**상태:** 구현·Android 실기기·iOS Simulator 실행 및 구현 검토 완료 · 제한 C11.2.3 pixel gate 통과
**대상:** [C11.2.3 dense·column 자동 배치](c11-2-3-dense-column-placement.md)의 누락된 표면 검증 관문
**비교 기준:** 고정 Chromium 154 DPR 1 · C11.2.3 runtime matrix v1 · HTML fixture v6
**계약 버전:** 내부 계약과 crate 숫자 버전은 `0.1.0`을 유지한다.

**실행:** Android SM-S731N 실기기 `c1123-a-mv5yld3e-4ded3cc6`, iPhone 17 Pro/iOS 26.2 Simulator `c1123-i-mv5ymb0x-10eed1fb`. 플랫폼별 25 scene·107 node에서 성공 19·기대 거부 6을 확인했고, 19 scene·99 표본의 표면 RGBA는 양쪽 모두 exact 일치했다. 상세 기록은 [Android](../spec/internal/evidence/c11-2-3-dense-column-placement-android-physical-runtime-matrix-2026-10-13/c1123-a-mv5yld3e-4ded3cc6/README.md), [iOS](../spec/internal/evidence/c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv5ymb0x-10eed1fb/README.md), [구현 실패 관점 검토](../spec/internal/evidence/c11-2-3-mobile-surface-pixels-implementation-review-2026-10-13.md)를 따른다.

## 목적과 범위

C11.2.3의 배치 geometry는 Android 실기기와 iOS Simulator에서 확인했지만, 그 결과만으로 GPU surface에 올바른 장면이 그려졌다고 판정할 수는 없다. 이 계획은 같은 고정 scene의 flat-color 표본을 Android `SurfaceView` PixelCopy와 iOS WGPU/Metal drawable readback으로 수집해 Chrome 154 DPR 1 결과와 대조한다.

대상은 runtime matrix의 `expected.outcome == "success"`인 19개 scene이다. 각 scene은 새 앱 process와 새 runtime host에서 한 번씩 실행한다. 여섯 expected-rejection scene은 기존 geometry 행렬의 거부 근거를 참조하며 표면 readback을 요청하지 않는다. 19개 성공 scene의 표본은 모두 비교하며, 임의 subset이나 화면 screenshot만으로 완료 처리하지 않는다.

이번 관문은 제한된 C11.2.3 profile의 표면 그리기를 확인한다. C11.2.3 전체 CSS 지원, WPT 적합성, iOS 실기기, OS compositor 이후의 화면 색상, GPU 성능을 뜻하지 않는다.

## 고정 입력과 증거 결합

| 입력 | 고정 자료 | 검증 |
| --- | --- | --- |
| HTML·색상·data-node ID | `tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html` | HTML parse 오류, 중복 ID, scene별 노드 집합 |
| runtime scene·성공/거부 구분 | `tests/fixtures/css/c11/grid-auto-placement-c1123-runtime-matrix-v1.json` | schema, case 수, outcome, source hash |
| Chrome geometry·screenshot | `tests/fixtures/css/references/c11-2-3-grid-auto-placement-precomparison-v7.json` 및 연결 PNG | reference/fixture/PNG SHA-256, Chrome 154, DPR 1, 360×800 CSS px |
| 앱 draw·NodeId·surface 세대 | Android/iOS C11.2.3 scene 실행 로그 | 실행별 run ID, 앱 PID, scene, revision, surface generation |
| 수집 산출물 | PixelCopy PNG 또는 drawable RGBA 결과·원본 로그·비교 JSON | 실행 경로·PNG hash·각 표본의 좌표와 RGBA |

표본 fixture는 위 입력에서 결정적으로 생성한다. 기본은 Chrome 화면에서 3×3 이웃까지 같은 RGBA인 data-node 내부점이다. 한 축 크기가 3 CSS px 미만인 opaque flat-color box만 예외로 정수 경계가 정렬된 1×1 표본을 허용하고 `single-pixel-thin-box`로 표시한다. 이 예외는 Chrome PNG의 해당 픽셀도 class 색과 exact 일치해야 한다. 그리드 배경과 track gap도 현재 geometry·색상으로 안정적인 내부점이 있을 때 포함한다. 표본은 노드 ID, scene ID, CSS 좌표, 기대 RGBA와 원본 PNG 출처를 갖는다. 숨겨진 노드, 가려진 영역, viewport 밖 부분은 표본을 만들지 않고 이유를 기록한다. scene 하나의 표본 수는 1–64개로 제한한다.

## 플랫폼별 수집

### Android 실기기

- 대상은 현재 연결되고 확인된 Samsung SM-S731N / Android 16·API 36이다. AVD·에뮬레이터 결과를 실기기 결과에 섞지 않는다.
- 앱은 scene별 고유 run ID로 실행하고 C11.2.3 V8 fixture를 기존 host/FFI 경로로 평가한다.
- surface generation·renderer handle·크기·scene이 일치하는 presented frame 뒤에만 `SurfaceView` PixelCopy를 요청한다. PixelCopy 재시도 중 generation이 바뀌면 그 수집을 실패 처리한다.
- PixelCopy PNG와 Android 기기 픽셀 좌표를 보존한다. CSS 중심 좌표를 surface의 실제 픽셀 크기로 변환하며 기기 density를 고정 가정으로 대체하지 않는다.
- 완료 조건은 scene별 PNG 유효성, 고정 sample ID 집합과 1:1 일치, 각 sample의 정확한 RGBA 일치다.

### iOS Simulator

- 대상은 iPhone 17 Pro / iOS 26.2 Simulator다. iOS 실기기 근거로 표현하지 않는다.
- 기존 C11 surface readback 요청/콜백 경로를 재사용하되 C11.2.3 전용 FFI 진입점과 marker를 연결한다.
- 현재 drawable 크기와 surface generation으로 CSS 중심 좌표를 변환한다. readback 요청·완료가 같은 scene, runtime revision, renderer 및 generation에 속해야 한다.
- 결과는 WGPU가 Metal drawable에 제출하는 표면의 compositor 이전 픽셀이다. `simctl screenshot`, UIKit screenshot, view tree 또는 present marker는 표면 RGBA readback을 대체하지 않는다.

## 판정 기준

| 검증 대상 | 통과 기준 | 실패 판정 |
| --- | --- | --- |
| 고정 자료 | 모든 입력 hash·schema·scene ID가 계획 자료와 일치 | 변경되거나 출처가 불명확하면 capture 전에 중단 |
| 성공 scene | Android/iOS 각각 19/19 scene에서 capture와 비교 완료 | 누락·중복 scene, sample 수 불일치, stale run/generation은 실패 |
| 예상 거부 | 6개 거부 scene에서 runtime 오류·부분 frame 부재를 유지하고 pixel request가 0회 | 성공 pixel로 취급하거나 실패 scene 표면을 비교하면 실패 |
| 좌표 | Chrome DPR 1 CSS 표본 중심을 각 drawable/surface 크기에 맞춰 같은 반올림 규칙으로 변환 | 고정 DPR 가정, crop offset 누락, 범위 밖 좌표는 실패 |
| RGBA | 모든 표본의 네 채널이 Chrome DPR 1과 exact, 채널당 최대 오차 0 | 평균값·일부 채널·일부 표본의 통과로 상쇄하지 않음 |
| 동시성·수명 | scene마다 새 process/host, 단일 run ID, 동일 revision·surface generation | 이전 scene 로그·캡처·callback 혼입은 실패 |
| 플랫폼 한계 | Android 실기기와 iOS Simulator 결과를 별도 기록 | screenshot·AVD를 실제 표면·실기기 결과로 승격하면 실패 |

색상 허용 오차는 capture 전에 고정한다. 기존 C11.2.2의 flat-color 표본 기준과 맞춰 exact RGBA를 사용하고, 다른 채널 차이가 관찰되면 기준을 완화하지 않고 원인과 실제 색 공간·surface format을 조사한다.

## 실행 순서

1. [x] 고정 입력 hash를 확인하고 결정적 C11.2.3 pixel matrix를 생성·검증한다.
2. [x] fixture generator와 Android/iOS marker/comparator의 positive·negative 입력을 검사한다. Android는 잘못된 density와 실제 bitmap 비율이 다른 synthetic PNG도 비교한다.
3. [x] Android PixelCopy와 iOS Metal drawable readback을 연결했다. Android C11 capture는 density 대신 실제 PixelCopy 크기와 320×240 CSS viewport로 좌표를 계산한다.
4. [x] Android SM-S731N 실기기에서 성공 19 scene을 새 process별로 capture·비교했다.
5. [x] iOS Simulator에서 같은 19 scene을 새 process별로 capture·비교했다.
6. [x] 두 플랫폼에서 기대 거부 6 scene이 draw/readback을 요청하지 않는지 확인했다.
7. [x] 원본 로그, PixelCopy PNG, scene별 결과, fixture/hash manifest를 내부 evidence에 저장했다.
8. [x] 실제 구현 변경을 별도 20개 실패 관점으로 검토하고 좌표 환산 및 iOS counter 검증 결함을 수정한 뒤 관련 행렬을 재실행했다.
9. [x] 구현 계획·상태 대장·내부 계약·roadmap HTML을 결과와 한계에 맞췄다. Tailnet 미리보기만 동기화하고 GitHub Pages는 배포하지 않는다.

## 제외 및 보고 경계

- iOS 실기기 및 Android 외 기기군
- OS compositor/color management 뒤의 최종 디스플레이 픽셀
- 텍스트, 이미지, alpha blend, transform, clip 또는 복잡한 edge raster의 pixel parity
- 1px 표본을 포함한 장면의 box 전체 또는 viewport 밖 연장 부분
- 전체 fixture 25개 모바일 실행, 전체 CSS Grid/WPT 적합성, 성능 우위
- 기준을 바꾸어 통과시키는 색상 tolerance, screenshot-only 판정

실행 불가한 관문은 미검증으로 남긴다. 표면 검증이 통과해도 CSS Grid 전체 완료로 표시하지 않는다.
