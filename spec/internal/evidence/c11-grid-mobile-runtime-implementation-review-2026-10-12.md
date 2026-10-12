# C11.1 mobile runtime 연결 구현 검토 · 2026-10-12

**검토 범위:** C11.1 runtime host 생성·V8 평가·Android/iOS platform route·WGPU 표면 제출·Android PixelCopy 수집기·10-scene Android/iOS matrix collector를 검토했다. layout-core와 Stylo adapter의 이전 검토를 재사용하지 않았다.

## 실패 관점 20개

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | C11 인자가 MainActivity에서 다른 실험 route로 빠지는가 | `spinon_c11_grid_fixture`를 runtime GPU 화면 분기에서 확인했고 SM-S731N에서 route가 실행됐다. |
| 2 | Android GPU 실험 build flag가 꺼져도 성공 화면을 표시하는가 | 기존 `SPINON_C04_RUNTIME_GPU` gate를 통과해야 runtime 화면을 만든다. 미활성 화면은 오류 상태로 종료한다. |
| 3 | iOS AppDelegate 인자와 Swift controller 인자가 일치하는가 | `--spinon-c11-grid`를 두 경계에서 추적했고 iPhone 17 Pro Simulator에서 같은 인자로 scene이 실행됐다. |
| 4 | Rust host 생성의 output 포인터·capacity 오류가 성공 handle로 둔갑하는가 | C ABI가 null/zero capacity를 거부하고 host 생성 report가 부족하면 raw handle을 게시하지 않는다. Android/iOS host-ready 로그를 확인했다. |
| 5 | host 생성 실패 원인이 platform layer에서 사라지는가 | JNI와 Objective-C++가 Rust error report를 플랫폼 로그로 남긴다. 성공 실행에서 host-ready와 owner TID를 확인했다. |
| 6 | null 또는 이미 없는 handle을 평가하는가 | C11 eval FFI가 `raw_host` 실패를 argument error로 반환하며 platform route는 0 handle일 때 evaluation을 건너뛴다. |
| 7 | JS 호출이 OS main/UI thread를 막는가 | Android `runtimeQueue`와 iOS 최신 task lane 경로를 확인했다. 실제 보고의 caller TID와 V8 owner TID가 다르고 draw는 별도 render lane에서 제출됐다. |
| 8 | V8 isolate thread affinity가 깨지는가 | `isolate_per_session=1`, owner TID, 정상 eval report를 두 platform log에서 확인했다. |
| 9 | 화면 pixel 크기가 CSS viewport로 잘못 해석되는가 | 두 앱은 320×240 CSS px를 전달한다. Android scale 2.8125·surface 900×675, iOS scale 3·surface 960×720을 구분했다. |
| 10 | Grid 결과가 다른 문서/style/environment revision과 섞이는가 | 고정 host의 동일 eval report에서 document/layout revision과 5 node를 확인했다. Rust adapter의 root/revision negative cases는 별도 fixture에서 검증되어 있다. |
| 11 | layout 오류가 이전 장면을 C11 success처럼 보여주는가 | C11 eval status와 scene별 `layout=ready boxes=n`을 요구한다. iOS collector가 `generation=1` metadata를 포함한 실제 draw marker를 놓친 결함을 재현해 marker matcher를 수정하고 회귀 테스트를 추가했다. |
| 12 | C11 profile이 다른 runtime CSS profile에 암묵 추가되는가 | C11 전용 constructor/eval function이 새 `RuntimeBlockFlexGridV1` profile을 호출한다. 기존 profile은 변경하지 않는다. |
| 13 | surface 생성 전 evaluation이 사라지거나 크기 0으로 제출되는가 | evaluation 결과를 보존하고 surface generation이 준비된 뒤 renderer를 만들며 draw 보고에서 texture size와 5 boxes를 확인했다. |
| 14 | 오래된 surface generation이 PixelCopy에 사용되는가 | `sourceIsCurrent`가 surface·renderer generation, handle, dimensions를 확인하고 copy 전·후에 다시 검사한다. |
| 15 | renderer가 오류를 반환해도 pixel capture가 시작되는가 | capture 호출은 draw 결과가 `status=0`인 경우에만 실행된다. runtime logs에서도 successful present 뒤 capture가 시작됐다. |
| 16 | PixelCopy 일시 오류가 무한 재시도되거나 실패가 숨겨지는가 | `ERROR_SOURCE_NO_DATA`·`ERROR_TIMEOUT`만 최대 3회 재시도하고, 그 밖의 상태는 error marker로 종료한다. |
| 17 | 잘못된 sample coordinate나 변조 PNG가 비교 대상이 되는가 | PixelCopy helper가 CSS→device coordinate 변환, 표면 경계, bitmap 크기, 저장 PNG 재해석을 검사한다. 수집기는 PNG signature·로그 SHA-256도 대조한다. |
| 18 | 실제 pixel이 기대색과 달라도 capture가 성공 표시되는가 | 초기 구현은 sample별 `exact=false`만 기록하고 aggregate failure를 내지 않았다. `SPINON_C111_GRID_SURFACE_SUMMARY`를 추가해 표본 수 또는 색이 다르면 mismatch를 내고 수집기가 실패하게 수정했다. |
| 19 | geometry 비교가 평균값으로 node 단위 오류를 숨기는가 | Android 실기기와 iOS Simulator에서 각 10 scene·40 data node를 실행하고 wrapper 10개를 별도 비교한다. 모든 fixture node의 x/y/width/height를 각각 ±0.5 CSS px로 검사한다. |
| 20 | evidence가 실기기 pixel/WPT/CSS 전체 완료를 과장하는가 | Android 실기기 10-scene geometry와 `fixed-tracks-gap` PixelCopy 7/7, iOS Simulator 10-scene geometry/Metal 제출을 분리 기록한다. C11 iOS Metal readback, C11 WPT runner, 전체 CSS 지원은 미완료다. |

## 수정과 실행 근거

- Android PixelCopy callback은 처음에 각 sample의 `exact`만 로그에 남겼다. aggregate summary와 mismatch 상태를 추가했다. 비교 runner가 `matched`, 7/7 표본, PNG hash, node별 Chrome geometry를 검사하도록 보강했다.
- [Android SM-S731N 실행](./c11-grid-android-physical-device-2026-10-12/c11-grid-android-20261011T182711821Z-ebf7f7a5/README.md): 5/5 node geometry, PixelCopy 7/7 exact, PNG hash 일치.
- [iPhone 17 Pro Simulator 실행](./c11-grid-ios-simulator-2026-10-12/README.md): V8→Grid layout→Metal surface draw, 5/5 node geometry. Direct Metal surface pixel readback은 수집하지 않았다.
- 실행 수집기는 [capture-c11-grid-android-physical-device.mjs](../../../tools/css-reference/capture-c11-grid-android-physical-device.mjs)다. 이 문서는 Android PixelCopy와 iOS screenshot의 증거 종류를 섞지 않는다.

## 10-scene mobile collector 후속 검토

| 실패 관점 | 확인 및 조치 |
| --- | --- |
| scene allowlist 밖 ID, 인자 대소문자 변경, scene index 범위 초과 | Android/iOS allowlist와 Rust FFI index 거부를 유지하고 10개 ID 집합을 fixture inventory와 대조했다. |
| 생성 fixture가 HTML·CSS·palette·inventory 변경 뒤 낡은 채 실행됨 | `--check` generator를 수집기 시작 전에 실행하고, matrix/reference 입력 hash를 comparison에 기록했다. |
| scene 수 또는 data-node 수가 기대값보다 작거나 큼 | 양 플랫폼 모두 정확히 10 scene·40 data node를 실행했고 scene별 개수와 전체 집합을 비교했다. |
| HTML preorder 또는 parent 관계가 바뀜 | 생성기에서 구조와 NodeId 순서를 검사하고, 런타임 수집기에서 scene별 preorder NodeId를 비교했다. |
| fixture adapter wrapper를 data node로 오인하거나 검사 누락 | wrapper NodeId 1과 360×800 frame을 data node와 별도로 검증했다. |
| 이전 앱 process의 scene 로그를 새 결과로 채택 | 각 scene을 새 process로 시작하고 PID로 unified log를 필터링했다. |
| 로그 조회 범위가 renderer 제출 marker를 제외 | iOS predicate에 `SPINON_C0410_DRAW`를 포함했다. |
| 로그 순서가 달라도 성공 marker를 놓치거나 잘못 결합 | scene별 summary, wrapper/data frame 수와 draw box 수를 같은 PID log에서 모두 요구한다. |
| frame 좌표 정수·소수·음수 parsing 손실 | Swift가 FFI report에서 짧은 node frame marker를 내고 별도 Node parser test가 소수 및 음수 좌표를 검사한다. |
| 중복·누락 frame이 통과 | 예상 frame 개수와 preorder NodeId를 모두 검사한다. |
| geometry 평균이 단일 축 오차를 감춤 | x/y/width/height마다 기대값과 절대 오차를 독립 비교한다. |
| wrapper가 viewport 크기와 혼동 | 360×800 adapter frame과 320×240 viewport/surface를 별도 기록한다. |
| Metal submit log에 generation prefix가 있어도 실패 판정 | `SPINON_C0410_DRAW generation=… status=0` 실제 형식을 반영해 detector를 수정하고 올바른 box 수/실패/누락 negative case 테스트를 추가했다. |
| screenshot 존재만으로 geometry 성공 처리 | screenshot은 draw+frame 검증 후 보조 산출물로 저장하며 geometry 판정은 log frame과 Chrome reference로 수행한다. |
| screenshot 또는 matrix hash가 이전 실행에서 섞임 | run별 evidence directory를 새로 만들고 app/matrix/reference hash를 comparison에 보존한다. |
| Android PixelCopy 결과를 iOS 표면 근거로 재사용 | Android 7/7 PixelCopy와 iOS Metal submit/screenshot을 서로 다른 evidence로 유지한다. |
| iOS Simulator 결과를 iPhone 실기기로 일반화 | 대상 runtime과 UDID를 고정하고 Simulator 결과로만 표시한다. |
| 한 platform 전체 성공을 다른 platform 성공으로 간주 | Android 물리 장치와 iOS Simulator comparison을 별도 생성하고 각각 10/10·40/40을 확인했다. |
| 일부 scene만 성공했는데 matrix 성공으로 표시 | 전체 scene ID 집합·node total이 고정값과 다르면 process가 비정상 종료하며 comparison을 쓰지 않는다. |
| C11.1 fixture 결과를 WPT/CSS 전체 지원으로 확대 | status/plan/evidence에 C11 WPT runner, direct iOS pixel readback, 미구현 CSS 범위를 미완료로 명시한다. |

후속 실행에서 iOS log collector가 실제 renderer 성공을 놓치는 결함이 드러났다. 로그를 넓게 다시 조회해 원인을 재현하고 matcher와 회귀 테스트를 고쳤다. 고정 Chromium reference 기준 Android 실기기·iOS Simulator 각각 10/10 scene·40/40 data node의 geometry가 통과했으며, 이 결과는 surface pixel equality나 WPT 적합성을 뜻하지 않는다. [Android 10-scene 결과](./c11-grid-android-physical-device-2026-10-12/c11-grid-android-matrix-20261011T201500690Z-be1c7d50/README.md) · [iOS 10-scene 결과](./c11-grid-ios-simulator-2026-10-12/c11-grid-ios-matrix-20261011T200317362Z-9c5ad710/README.md).

Android 재검증에서는 단일 장면 수집기가 scene adapter wrapper를 세지 않고 `boxes=5`를 기대하는 stale assertion을 발견했다. 수집기를 wrapper 1개와 data node 5개로 나눠 확인하도록 수정한 뒤, 현재 연결된 SM-S731N 실기기에서 단일 장면과 10-scene 전체 행렬을 다시 실행했다. 현재 APK SHA-256 `f1bef1b6…0964b35`에서 단일 장면은 wrapper 1/1·data node 5/5·PixelCopy 7/7 exact, 전체 행렬은 10/10 scene·40/40 data node 통과다. 이 변경은 검증 수집기와 근거 참조만 수정하며, iOS direct pixel readback이나 C11 WPT runner는 완료하지 않는다. [단일 장면 재검증](./c11-grid-android-physical-device-2026-10-12/c11-grid-android-20261011T201439778Z-44cf5612/README.md) · [최신 전체 행렬](./c11-grid-android-physical-device-2026-10-12/c11-grid-android-matrix-20261011T201500690Z-be1c7d50/README.md).

이 검토는 C11.1 전체 지원, iOS 색상 표본 일치, 전체 10-scene Spinon 비교, WPT 적합성, 성능 우위 또는 C11 완료를 승인하지 않는다.
