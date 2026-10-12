# C11.1 후속 · iOS Metal surface 픽셀 표본 계획

**상위 계획:** [C11 CSS Grid](c11-css-grid.md) · [C11.1 모바일 runtime 행렬](c11-1-mobile-runtime-matrix.md)

**상태:** 구현·검증 완료 · 고정 iOS Simulator Metal drawable 표본 7/7 exact · 내부 계약 숫자 버전 `0.1.0` 고정

**계획 검토:** [독립 실패 관점 기록](../spec/internal/evidence/c11-1-ios-metal-surface-pixels-plan-review-2026-10-12.md)

**구현 검토:** [20개 실패 관점과 경계](../spec/internal/evidence/c11-1-ios-metal-surface-pixels-implementation-review-2026-10-12.md) · [실행 evidence](../spec/internal/evidence/c11-1-ios-metal-surface-pixels-2026-10-12/run-c11-ios-metal-20261011T205937099Z-a996f7ef/README.md)

**목표:** iPhone 17 Pro / iOS 26.2 Simulator에서 기존 C11.1 `fixed-tracks-gap` V8→Stylo→Taffy→WGPU 장면이 제출한 `CAMetalLayer` 표면의 실제 픽셀을 직접 읽고 고정 Chromium 154 표본과 대조한다. Chrome oracle viewport는 360×800 CSS px/DPR 1이고 Spinon draw viewport는 320×240 CSS px다. Grid 장면과 표본은 두 표면의 (0,0)에 놓인 공통 scene-local 영역에 있다. Simulator screenshot은 보조 자료로만 보존한다.

## 검증 대상과 범위

- 장면은 이미 고정된 C11.1 `fixed-tracks-gap` 하나, Spinon draw viewport 320×240 CSS px, 5개 data node와 runtime wrapper다. Chrome reference viewport는 별도로 고정된 360×800 CSS px/DPR 1이며, 두 장면의 공통 좌상단 Grid 영역만 비교한다. 새 레이아웃 의미나 공개 CSS 지원을 추가하지 않는다.
- 표본 ID·CSS 좌표·기대 RGBA는 pinned Chrome 154 DPR 1 `fixed-tracks-gap` PNG에서 직접 추출해 새 고정 JSON fixture에 둔다: `grid-background`, `item-blue`, `item-red`, `item-green`, `item-yellow`, `column-gap`, `row-gap`. fixture는 Chrome reference JSON과 PNG의 경로·SHA-256, oracle viewport와 DPR, CSS 표본 좌표를 고정한다. Android PixelCopy 값은 교차 확인만 하며 유일한 기대값 원본이 아니다.
- 고정 시뮬레이터는 iPhone 17 Pro / iOS 26.2 (`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`)다. 다른 Simulator, 다른 booted 기기, 실기기 결과로 대체하지 않는다.
- 표면 픽셀은 WGPU가 제출한 실제 iOS drawable texture를 `COPY_SRC`로 복사하여 읽는다. 이 값은 WGPU surface texture의 compositor 제출 전 내용이며 화면 합성기·색상 관리 뒤의 물리 display pixel을 뜻하지 않는다. `simctl screenshot`, UIKit screenshot, SwiftUI/UIKit view 색상 또는 단순 draw marker는 surface pixel 증거가 아니다. screenshot은 보조 이미지로만 보존한다.
- CSS 표본 좌표 `(x,y)`는 C11 drawable 로컬 원점 기준이며 device pixel `(floor((x+0.5)×scale), floor((y+0.5)×scale))`로 바꾼다. `scale`은 UIKit 화면 전역값으로 추정하지 않고 draw report의 실제 drawable 너비와 320 CSS px viewport 비율로 계산하며 높이 방향 비율과도 일치해야 한다. Chrome DPR 1 좌표는 Chrome PNG에서 같은 CSS `(x,y)`다. oracle viewport 360×800과 iOS draw viewport 320×240의 차이를 임의 offset으로 보정하지 않는다. 각 sample의 ID, 실제 device coordinate, RGBA, 기대값과 채널별 오차를 기록하고 7/7을 각각 검사한다. 허용 채널 오차는 0이다.
- 결과는 C11.1 fixed scene의 7개 표면 표본만 말한다. 나머지 9개 scene pixel, 전체 Grid, iOS 실기기, 성능, WPT 적합성 또는 전체 CSS 지원으로 확대하지 않는다.

## 구현 경계

1. 기존 C12.4 전용 readback plumbing은 새 C11 코드에서 C12 이름으로 호출하지 않는다. WGPU의 internal surface-pixel copy/readback mechanism을 internal test-only feature로 공유하고, C11·C12의 fixture constructor와 C ABI entry는 각 소유 영역에 둔다.
2. surface readback batch는 1–64개 표본만 받는다. FFI의 포인터·count 쌍은 null, 0, 상한 초과, `count×2` overflow를 검사하고, 빈 목록·drawable 밖 좌표·미지원 texture format·이미 제출되지 않은 이전 요청은 명시적으로 실패한다. C12.4의 기존 단일 표본 흐름은 동일 입력에서 계속 동작해야 한다.
3. C11 readback 진입점은 별도 compile-time test flag로만 포함한다. 기본 Android/iOS 앱에서는 flag와 API가 비활성 상태여야 한다. 앱 개발자가 쓰는 공개 API, DOM API, CSS 지원 surface에 추가하지 않는다.
4. Swift는 고정 C11 scene과 sample allowlist만 허용한다. run ID, 프로세스 PID, surface generation, runtime layout revision, drawable 크기 및 display scale을 같은 실행 로그에 묶는다. 종료·resize·재구성으로 generation이 바뀌면 해당 결과를 실패 처리한다.
5. Node 수집기/비교기는 현재 프로세스/run의 성공 eval·draw marker, 정확한 7개 sample ID/좌표 집합, pinned Chromium/source hash, 보조 screenshot의 PNG signature/hash, 채널별 exact RGBA를 모두 검사한다. 누락·중복·오래된 run·잘못된 PNG·개별 색상 오차는 aggregate 성공으로 감추지 않는다.

## 실행 순서와 완료 기준

1. [x] 이 계획을 독립적인 20개 실패 관점으로 검토하고 Chrome viewport 360×800과 Spinon draw viewport 320×240의 차이, drawable 좌표 공간 및 배율 산출을 명시했다.
2. [x] 표본 JSON을 Chrome reference screenshot의 ID·좌표·색·DPR·path/hash에 고정하고 원본 PNG decode 검사와 comparator negative control 29개를 추가했다.
3. [x] Rust WGPU readback이 한 surface frame에서 1–64개 표본을 aligned buffer로 복사한다. 빈·초과·바깥 좌표, 포맷, mapping 오류를 실패로 처리하고 C12.4 단일 표본 경로를 유지했다.
4. [x] internal Cargo/Xcode flag, Rust C ABI, Objective-C++와 Swift 경로를 C11 Grid 전용으로 연결했다. 일반 iOS matrix build log에서 flag 기본값 0을 확인했다.
5. [x] 고정 iPhone 17 Pro/iOS 26.2 Simulator 새 앱 프로세스에서 7/7 Metal drawable 표본이 Chrome DPR 1 PNG와 exact 일치했다. app binary, source/reference/fixture hash, unified log, screenshot, comparison JSON을 저장했다.
6. [x] Rust surface readback 경계 3개, FFI pointer/count 2개, comparator 29개 test를 실행했다. iOS capture 실패 시 build/install/launch/unified log와 실패 단계를 보존한다.
7. [x] 구현 코드를 계획 검토와 분리한 20개 실패 관점으로 검토하고 revision·viewport·drawable 비율·compile flag·C12 회귀 경계를 확인했다.
8. [x] C11 plan, `spec/STATUS.md`, 내부 evidence index, public roadmap와 Tailnet preview를 7/7 결과 및 미해결 경계에 맞춰 동기화했다. GitHub Pages는 배포하지 않는다.

다음 조건을 모두 만족해야 이 하위 관문을 완료한다: 같은 고정 Simulator와 현재 app binary에서 실제 Metal surface readback 7/7, 각 좌표·RGBA exact, build/source/oracle hash가 보존됨, test-only compile gate 기본 비활성, C12.4 단일 표본 경로 회귀 없음, comparator와 0·1·7·64·65개/경계 좌표/잘못된 FFI 입력 negative control 통과, 계획·구현 review와 공식 상태 문서가 서로 일치함. 이 완료는 WGPU texture가 모니터에서 동일한 색으로 보이거나 iOS 최종 compositor pixel까지 같음을 보장하지 않으며, C11.1의 iOS pixel sample 하나를 닫을 뿐 C11 전체 또는 C11.1의 10-scene pixel 비교를 완료하지 않는다.

## 제외 범위

- Android PixelCopy 전체 장면 비교, C11.1 10 scene 전부의 pixel 비교, iOS 물리 기기, 전체 WPT runner, 성능 측정, CSS 범위 확장.
- C12.4의 8-case reverse-Flex 색상 동작 변경.
- 앱 릴리스·OTA·공개 API 추가나 내부 계약 숫자 버전 상승.
