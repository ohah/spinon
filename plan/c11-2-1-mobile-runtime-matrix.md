# C11.2.1 · 모바일 runtime 전체 장면 행렬

**상태:** 계획·구현 검토 완료 · Android API 37 AVD, Android 16/API 36 SM-S731N 실기기, iPhone 17 Pro/iOS 26.2 Simulator에서 각 10/10 scene 통과 · 표면 행렬은 별도 계획에서 관리 · WPT runner/적합성은 X16 별도 미완료  
**상위 계획:** [C11.2.1 명시 Grid line 배치](c11-2-1-explicit-grid-placement.md) · [C11 Grid 전체 계획](c11-css-grid.md)  
**고정 기준:** Chrome 154 · [placement inventory](../tests/fixtures/css/c11/grid-placement-inventory.json) · [Chrome reference](../tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json)  
**계획 실패 관점 검토:** [20개 검토](../spec/internal/evidence/c11-2-1-mobile-runtime-matrix-plan-review-2026-10-12.md)

## 목표와 범위

C11.2.1의 고정 10개 장면을 Android와 iOS의 실제 앱 runtime 경로에서 각각 실행한다. inventory상 구현 후보 6개는 V8 → HostDocument → Stylo → Taffy → WGPU 경로로 계산하고, 나머지 4개는 이번 profile이 거부해야 하는 음성 대조로 실행한다. 이는 새 CSS 기능을 추가하지 않고 이미 구현한 explicit numeric line/span profile의 모바일 연결·실패 경계를 검증한다.

Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 우선 실행 대상으로 한다. AVD는 소프트웨어 renderer 결과이며 Android 실기기 검증을 대체하지 않는다. iOS Simulator도 실제 iPhone 결과가 아니다. 이번 행렬은 geometry·성공/거부 경계·WGPU 제출과 화면 캡처를 확인한다. Surface pixel readback은 별도 계획에서 관리한다. WPT runner·전체 적합성은 X16 범위이며 이 runtime matrix의 완료 조건이 아니다. 성능 우위도 주장하지 않는다.

## 고정 장면과 판정

| inventory 장면 | 기대 결과 | 필수 관측 |
| --- | --- | --- |
| `numeric-lines` | 성공 | root와 3개 node의 Chrome frame, WGPU 제출 |
| `positive-spans` | 성공 | root와 3개 node의 Chrome frame, WGPU 제출 |
| `reversed-lines` | 성공 | root와 2개 node의 Chrome frame, WGPU 제출 |
| `shorthand-cascade` | 성공 | root와 3개 node의 Chrome frame, WGPU 제출 |
| `overlap-source-order` | 성공 | root와 3개 node의 Chrome frame, 겹침 geometry·paint snapshot 순서, WGPU 제출 |
| `nested-placement-modes` | 성공 | root와 7개 node의 Chrome frame, 바깥·중첩 container 모드, WGPU 제출 |
| `deferred-implicit-track` | `invalid_grid_placement` 거부 | `c1121-implicit-item` · `grid-column-start` 진단, 새 frame·WGPU 제출 없음 |
| `deferred-named-line` | cascade 단계 `cascade_failed` 거부 | runtime owner/property는 `None`; 상세 cascade 진단에 `grid-template-columns` 포함, 새 frame·비어 있지 않은 WGPU 제출 없음 |
| `deferred-mixed-auto-explicit` | `invalid_grid_placement` 거부 | `c1121-mixed-auto` · `grid-row-start` 진단, 새 frame·WGPU 제출 없음 |
| `deferred-equal-lines` | `invalid_grid_placement` 거부 | `c1121-equal-item` · `grid-column-end` 진단, 새 frame·WGPU 제출 없음 |

행렬 생성기는 inventory의 scene ID·순서·scope·node preorder와 pinned Chrome reference의 case/node 순서를 대조한다. 검증값을 직접 다시 쓰지 않고 HTML scene에서 제한 runtime source를 생성한다. 생성물에는 inventory·HTML·Chrome reference·generator hash를 넣고 `--check`가 재생성 차이를 거부한다. Runner는 고정 scene ID만 허용하며 임의 JavaScript나 CSS 문자열을 받지 않는다. 기대 거부의 code·fixture node ID·computed property는 companion matrix에 고정한다.

성공 scene은 scene마다 새 앱 process와 새 runtime host에서 실행한다. 각 frame의 node ID·x/y/width/height를 Chrome DPR 1 reference와 `0.5 CSS px` 이하 허용치로 비교한다. 지원 node 누락·중복·순서 변경, 비유한 값, 불완전한 frame, timeout은 실패다. WGPU 표면 제출은 별도 marker와 기대 box 수로 확인한다. 전체 화면 캡처는 사람이 실행 여부를 확인하는 근거일 뿐 pixel parity로 부르지 않는다.

거부 scene도 새 process와 host에서 하나씩 실행한다. 성공으로 바뀌거나 예상 code·node·property 진단이 달라지면 실패다. 같은 process의 이전 revision frame, 화면 또는 draw marker가 거부 결과에 남아 성공처럼 보이지 않는지 확인한다. 앱 crash·marker 누락·수집 timeout은 기대 거부로 간주하지 않는다.

각 launch argument에는 run ID와 scene ID를 함께 싣는다. Android 수집기는 logcat 전체를 비우지 않고 새 process PID와 이 run ID가 모두 일치하는 Spinon marker만 사용한다. iOS도 새 process ID·run ID·scene ID를 모두 확인한다. OS가 PID를 재사용하거나 이전 로그가 남아도 다른 scene 결과와 합쳐지지 않아야 한다.

## 실행·증거 계약

- Android와 iOS 각각 10/10 scene을 실행한다. 플랫폼별 성공 6/6·기대 거부 4/4를 별도로 집계하고 두 플랫폼 수치를 합산하지 않는다.
- 각 scene을 새 앱 process로 실행해 HostDocument, layout cache, surface generation, 이전 오류와 frame이 다음 case로 넘어가지 않게 한다. 앱 binary·source fixture·reference·generator hash를 run manifest에 기록한다.
- Android는 `SPINON_ENABLE_C04_RUNTIME_GPU=1`로 빌드하고 AVD/API/model/renderer를 기록한다. Android 실기기가 연결되면 같은 runner를 실기기에서도 재실행하되 AVD 결과와 별도 run으로 보존한다.
- iOS는 지정 UDID의 iOS 26.2 Simulator를 확인하고 scene별 process ID·launch argument·app executable hash를 기록한다.
- 지원 scene 화면 캡처, 원시 runtime log, frame JSON, 기대/실제 결과, scene별 실패 로그와 전체 summary를 저장한다. 지원·거부 case 모두를 evidence manifest에 포함한다.
- Rust matrix generator/unit tests, C ABI 입력 경계, Android/iOS build 및 runner 결과를 같은 source snapshot에 귀속한다. fixture/reference hash 불일치, 중복·누락 case, 다른 binary 또는 platform 상태 불일치는 run 실패다.
- 전체 surface PixelCopy/Metal readback은 기존 별도 pixel plan의 기준을 따른다. 이 matrix 결과만으로 surface pixel parity, WPT 적합성, 실기기 GPU, 전체 CSS Grid 완료를 주장하지 않는다.

## 완료 기준

- [x] 계획 자체 실패 관점 검토를 독립된 20개로 수행하고 발견을 반영한다.
- [x] matrix 생성물과 negative controls가 고정 inventory/hash, scene order, scope, node preorder 및 임의 입력 거부를 자동 검증한다.
- [x] Android API 37 AVD와 지정 iOS Simulator에서 각각 성공 6/6·기대 거부 4/4를 확인하고, 각 성공 frame·paint order·WGPU marker와 각 거부 code/node/property를 기록한다.
- [x] 최신 source snapshot의 Android APK/iOS executable hash와 fixture/reference hash가 전체 run에 일치한다.
- [x] 구현 후 실제 diff·fixture·runtime log 대상으로 별도의 실패 관점 20개 검토를 수행했다. [검토 결과](../spec/internal/evidence/c11-2-1-mobile-runtime-matrix-2026-10-12/implementation-review-2026-10-12.md). 발견한 named-line cascade 진단 경계를 matrix와 비교기에 반영하고 전체 검사를 다시 실행했다.
- [x] 상태 대장, C11.2.1 parent plan, 내부 evidence index, public roadmap source와 Tailnet preview를 결과·미완료 경계에 맞춰 동기화했다. GitHub Pages는 배포하지 않았다.

이 계획의 AVD·Simulator 결과만으로 Android 실기기, C11.2.1 전체 완료, 전체 surface pixel parity, WPT 적합성 또는 CSS Grid 전체 지원을 뜻하지 않는다. Android 실기기 10-scene run과 표면 비교는 별도 run으로 보존한다. WPT runner·적합성은 X16에서 관리한다.

## 2026-10-13 현재 소스 후속 실행

같은 현재 소스의 10-scene 행렬을 Android API 37.1 AVD, Android 16/API 36 SM-S731N 실기기, iPhone 17 Pro/iOS 26.2 Simulator에서 각각 실행했다. 세 대상 모두 성공 6/6·기대 거부 4/4, 21 node, 최대 geometry 오차 0 CSS px를 기록했다. [통합 요약](../spec/internal/evidence/c11-2-1-current-source-rerun-2026-10-13/README.md) · [Android 실기기 상세](../spec/internal/evidence/c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T161006179Z-4ffd28bc/README.md).
