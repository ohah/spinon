# C11.1 모바일 runtime 10-scene 교차 실행 계획

**상위:** [C11 CSS Grid 계획](c11-css-grid.md) · [공식 상태 대장](../spec/STATUS.md)  
**계약:** [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)  
**계획 검토:** [실패 관점 20개](../spec/internal/evidence/c11-1-mobile-runtime-matrix-plan-review-2026-10-12.md)  
**상태:** Android 실기기·iOS Simulator 10-scene geometry matrix 10/10·40/40 통과 · Android API 37 AVD·iOS Simulator 표면 pixel matrix도 10/10 scene·36/36 exact · Spinon WPT runner와 실기기 표면 행렬은 별도 미완료

## 목적

고정 Chromium oracle의 10개 장면·40개 노드를 Android와 iOS runtime 경로에 각각 투입해 Grid의 fixed/percentage/`fr` track, row auto-placement, 빈/단일 item, Block·Flex 혼합 부모/자식 dispatch가 앱의 V8 → HostDocument → Stylo → Taffy → WGPU 경로에서도 계산되는지 확인했다.

이 작업은 새 Grid 문법을 추가하지 않는다. `grid-layout.html`, `grid-layout-inventory.json`, 고정 `c11-grid-precomparison-v2.json`을 입력으로 사용하며 기존 reference를 덮어쓰지 않는다. C11.1 subset 외 입력을 새로 지원하는 것처럼 취급하지 않는다.

## 고정 입력과 출력

- 기준은 Chrome 154의 기존 10 scene·40 node·DPR 1/2 reference다. mobile geometry 비교는 DPR 1 reference를 사용하고 각 대상 NodeId의 `x`, `y`, `width`, `height` 절대 오차를 각각 `0.5 CSS px` 이하로 판정한다.
- 런타임에 실행할 scene 집합은 inventory의 10개 ID와 정확히 일치해야 한다. 각 scene의 예상 preorder NodeId, parent 관계, node 수를 HTML fixture와 inventory 양쪽에서 검증한다.
- Android와 iOS는 scene마다 새 앱 process, 새 V8 host, 새 HostDocument로 실행한다. 한 scene의 document, style/cache, render snapshot을 다음 scene에서 재사용하지 않는다.
- 장면마다 run ID, platform/device, app artifact SHA-256, fixture/reference SHA-256, renderer, raw runtime log, geometry 비교 JSON, 화면 PNG를 보존한다. Android와 iOS 결과는 별도 판정한다.
- Android의 기존 `fixed-tracks-gap` PixelCopy 7 sample exact 결과는 계속 별도 surface pixel gate로 보존한다. 이번 10-scene geometry 행렬의 성공을 10-scene PixelCopy 또는 iOS Metal pixel readback으로 확대하지 않는다.

## 입력 변환과 경계

- HTML fixture를 제한적으로 읽어 각 `data-scene`의 `data-node` 요소와 직계 구조를 추출한다. 원본의 `.scene-root`를 무시하지 않는다. runtime profile에서 원본 absolute style은 cascade 오류로 거부되므로, 고정 360×800 `display:flex` fixture adapter wrapper를 만들고 Chrome 대비 모든 data-node frame과 wrapper frame을 검증한다. 첫 실기기 행렬에서 wrapper가 없으면 `grid-under-block`의 부모 margin이 x=10px 대신 x=0px가 됨을 확인했다. 이 fixture adapter의 일치 결과는 일반 Flex wrapper나 absolute CSS 지원 증거가 아니다. 변환기는 임의 HTML을 받아들이는 제품 parser가 아니며 이 fixture의 지원 subset만 처리한다.
- HTML tree는 이미 문서 workspace dependency graph에 잠긴 `parse5` `7.3.0`을 root devDependency로 명시해 읽는다. 이는 reference/test generator 전용이며 mobile runtime·번들·제품 dependency에는 들어가지 않는다. HTML parsing을 취약한 정규식으로 대체하지 않는다.
- 변환기는 알 수 없는 element, 중복 ID, 중첩 누락, inventory 불일치, 알 수 없는 class 또는 예상하지 못한 class CSS를 명시 오류로 거부한다. 노드·속성을 조용히 생략하거나 기본 Block으로 바꾸지 않는다.
- `track-surface`, `blue`, `red`, `green`, `yellow`, `purple`, `cyan`, `orange`, `pink` class는 입력 stylesheet에서 순수 `background-color` 값만 확인해 runtime fixture의 inline paint 값으로 옮긴다. Grid layout 문법은 stylesheet 재작성으로 바꾸지 않고 노드의 원래 inline layout style을 보존한다. palette가 바뀌거나 class가 추가되면 변환 테스트와 reference 검토가 실패해야 한다.
- 생성 runtime source는 저장소 안의 고정 fixture에서만 만든다. 모바일 intent/argument는 정확한 scene ID allowlist를 통과하고 native FFI에는 검증된 정수 index만 전달한다. 호출자가 임의 JavaScript, 파일 경로 또는 임의 ID를 runtime에 주입하는 API를 만들지 않는다.
- 생성된 source table에는 HTML, inventory, palette parser/generator 입력의 digest와 scene별 NodeId 목록을 기록한다. 생성 결과가 source와 어긋나면 build/test 단계에서 실패시킨다.
- fixture adapter wrapper는 각 scene에서 렌더 box 하나를 만들기 때문에 별도로 기록하고 frame을 검사한다. 40개 Chrome reference `data-node`와 adapter wrapper 10개는 구분해 비교하며, wrapper용 NodeId가 fixture preorder 앞에 배치되는지 확인한다.

## 모바일 실행 및 비교

1. Android/iOS 런타임 argument는 fixture의 scene ID만 허용한다. 없는 ID·대소문자 변형·중복 인자는 host 생성 전에 실패하고 성공 marker를 남기지 않는다.
2. 플랫폼은 한 scene당 하나의 runtime host를 생성하고 V8 평가 결과, Grid layout ready, wrapper 및 fixture NodeId frame, WGPU 제출 box 수를 기록한다. 데이터 node 누락·중복, wrapper frame 불일치, scene ID 불일치, 잘못된 box 수, renderer 미준비는 실패다.
3. collector는 현재 run marker와 현재 process ID를 확인한 뒤 로그를 읽는다. 이전 launch 로그나 PNG를 재사용하지 않는다. scene 순서의 첫/마지막, 재실행, 중간 scene 실패 뒤 복구를 포함한다.
4. comparator는 inventory/reference/HTML/source hash, scene ID 집합, node ID preorder/parent, wrapper frame, 데이터 frame field와 tolerance를 각각 확인한다. 평균 오차로 개별 실패를 감추지 않는다. 행렬의 일부 scene만 존재하면 전체 행렬을 실패 처리한다.
5. geometry matrix는 기능 및 경로 일치의 증거다. 이 결과만으로 pixel exactness, 전체 Grid, Spinon WPT 적합성, hardware 성능, 공개 CSS 지원 또는 C11 완료를 주장하지 않는다.

## 구현 순서

- [x] 10-scene source generator와 deterministic artifact를 추가하고 HTML·inventory·palette·reference 간 ID 및 hash 계약을 검사한다. `.scene-root` wrapper를 제거하지 않는다.
- [x] Rust FFI에 고정 scene index 선택 경계를 추가했다. 기존 단일 `fixed-tracks-gap` 호출은 유지하고 잘못된 정수 범위를 fail-closed한다.
- [x] Android launch argument 및 수집기를 scene allowlist와 새 process 실행으로 확장했다.
- [x] iOS launch argument 및 시뮬레이터 수집기를 같은 scene allowlist와 새 process 실행으로 확장했다.
- [x] Android/iOS 10 scene·40 node의 독립 geometry 비교를 실행했다. Android 기존 PixelCopy canary 7/7 exact 결과를 별도 surface gate로 보존했다.
- [x] 변경 구현을 별도 실패 관점으로 검토했다. iOS draw marker 형식 불일치가 발견되어 matcher와 회귀 테스트를 고친 뒤 matrix를 다시 통과시켰다. [구현 검토](../spec/internal/evidence/c11-grid-mobile-runtime-implementation-review-2026-10-12.md)
- [x] `plan/c11-css-grid.md`, `spec/STATUS.md`, 내부 evidence index, public roadmap HTML 및 Tailnet이 읽는 원본을 같은 결과·남은 경계에 동기화한다. GitHub Pages는 배포하지 않는다.

## 완료 판정

- Android SM-S731N 실기기와 iPhone 17 Pro/iOS 26.2 Simulator 각각 10/10 scene·40/40 fixture node 집합이 완전하고, 360×800 fixture adapter wrapper 10개의 frame을 별도 검증하며 고정 Chromium reference 대비 모든 fixture frame field가 허용치 이내다. Android 실기기 최신 재실행은 APK SHA-256 `bc588a8a…438555e9`다. [최신 Android 행렬과 PixelCopy canary](../spec/internal/evidence/c11-grid-android-physical-device-2026-10-12/README.md) · [iOS 결과](../spec/internal/evidence/c11-grid-ios-simulator-2026-10-12/c11-grid-ios-matrix-20261011T205619778Z-2032fc2e/README.md).
- 잘못된 scene ID, stale run, 누락·중복·순서 변경 node, source/hash drift, 한도 초과 frame 오차, scene 하나가 빠진 행렬은 comparator negative control에서 실패한다.
- Android/iOS WGPU 제출과 화면 PNG가 각각 보존되며, Android `fixed-tracks-gap` PixelCopy 기존 7/7 exact 결과를 같은 source snapshot으로 다시 확인한다.
- 남은 경계는 두 플랫폼에서 나머지 9개 scene 전체의 pixel 비교, C11 Spinon WPT runner, 타 기기군 및 공개 CSS 지원이다. Android fixed scene PixelCopy와 iOS fixed scene Metal drawable 표본은 각각 7/7 exact다. 이 geometry 작업만으로 C11.1 전체나 C11 완료를 표시하지 않는다.
- 나머지 9개 scene 전체의 pixel 비교는 [별도 표면 pixel 계획](c11-1-mobile-surface-pixel-matrix.md)에서 Android API 37 AVD와 iOS Simulator 각각 10/10 scene·36/36 exact로 실행했다. Android AVD는 ANGLE/SwiftShader이며 Android 실기기 측정으로 보지 않는다. Spinon WPT runner, 타 기기군, 전체 CSS 및 공개 CSS 지원은 여전히 미완료다.
