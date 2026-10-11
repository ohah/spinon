# C12.4 Android AVD matrix 구현 검토

**대상:** Android AVD 26-scene runtime matrix, Rust render-failure diagnostic mapping, Android case runner, Chromium frame comparator 및 이번 변경 문서
**실행 기기:** Android 17 / API 37.1 / 16 KB page size AVD `emulator-5554`만 사용. 이번 검토에서 Android 실기기는 연결되지 않았다.
**최종 실행:** 2026-10-11 16:20 KST · APK Debug build 성공 · 26/26 기대 결과 일치 · 64 fixture node 최대 frame 오차 0 CSS px

## 별도 실패 관점

| # | 공격 관점 | 확인 결과와 처리 |
| ---: | --- | --- |
| 1 | 실기기 입력이 simulator 결과에 섞이는가 | 실행 전후 `adb devices -l`에 AVD `emulator-5554`만 있었다. 실기기 측정으로 표시하지 않는다. |
| 2 | AVD를 하드웨어 GPU 근거로 오인하는가 | WGPU 로그에서 ANGLE/Vulkan SwiftShader adapter를 확인했다. 소프트웨어 그래픽 경로 및 성능 비대표성을 문서에 적었다. |
| 3 | inventory 변경이 조용히 matrix에 반영되는가 | 생성기는 fixture ID, 26 case, 총 77 node를 먼저 확인하고 inventory SHA-256을 출력한다. |
| 4 | case ID 중복이 누락을 가리는가 | generator 테스트와 comparator가 고유 case 수 및 중복 런타임 ID를 거부한다. |
| 5 | source node 수만 맞고 장면이 다른가 | comparator가 고정 inventory case/node 순서와 Chromium reference ID를 각각 대조한다. |
| 6 | class selector 부분 문자열이 엉뚱한 CSS rule을 선택하는가 | class 토큰 경계를 사용하고 생성기 테스트에서 rule 포함 범위를 확인한다. |
| 7 | 외부 stylesheet 누락으로 author cascade 결과가 달라지는가 | 각 장면에 해당 규칙을 V8 `style` 요소로 삽입한다. CSS가 없는 장면은 style 요소를 만들지 않는다. |
| 8 | cascade layer 순서가 장면별 추출 중 사라지는가 | 해당 layer rule이 있을 때 선언 순서를 함께 추출하도록 하고 test가 `@layer` 순서 source를 고정한다. |
| 9 | 등록 사용자 지정 속성이 정의 없이 계산되는가 | 등록 속성 장면에서 `@property` 규칙을 먼저 포함한다. 생성기 test가 이를 확인한다. |
| 10 | 초기 viewport 크기나 stage offset 차이가 비교를 오염시키는가 | 매 host를 320×240 CSS px로 설정하고 Chromium 기준과 같은 stage origin을 geometry 기대값에 더한다. |
| 11 | 한 장면의 문서/cache 상태가 다음 장면으로 누출되는가 | 매 case마다 새 native host를 만들고 `finally`에서 해제한다. |
| 12 | host 생성·환경 설정 실패가 CSS의 예상 거부로 잘못 집계되는가 | `op=eval` 실패가 명시된 경우만 거부로 인정한다. host 생성 실패는 `matched=false`로 남고 전체 요약도 실패한다. |
| 13 | JavaScript 평가 실패가 layout 지원 성공으로 분류되는가 | `ready`는 `status=0`과 `layout=ready`를 모두 요구한다. |
| 14 | `visibility` 거부가 노드와 속성을 보고하는가 | 실제 로그에서 `node=Some(...) property=Some("visibility")`를 두 장면 모두 확인했고 Rust mapping unit test를 추가했다. |
| 15 | `opacity` 거부가 다른 속성 진단으로 통과하는가 | comparator는 `node=Some(...)`와 정확한 `property=Some("opacity")`를 요구한다. |
| 16 | `showModal` 미지원이 임의의 TypeError로 통과하는가 | 기대 API가 `showModal`인지 확인하고 report의 TypeError 및 `showModal is not a function`을 모두 요구한다. 이 결과는 top-layer paint 검증이 아니라 API 미지원 거부 확인이다. |
| 17 | pseudo-element 거부 문구가 로케일 가정 때문에 흔들리는가 | 첫 재실행에서 영어 진단 문자열 가정이 실제 한글 CSS parser 오류를 잘못 거부해 25/26이 됐다. 검사를 실제 `지원하지 않는 CSS 선언 content` 진단에 맞춰 수정하고 APK를 재빌드했다. |
| 18 | render frame에 wrapper/style 노드가 더 생겨도 누락을 놓치는가 | 첫 비교에서 style 요소가 있는 case의 frame 1개를 누락 예상해 비교기가 실패했다. frame 수를 root·stage·필요한 style 요소·fixture node로 계산하고 고유 ID도 확인하도록 고쳤다. |
| 19 | 평균 오차가 개별 frame 오차를 숨기는가 | node별 x/y/width/height 절대 오차를 모두 검사한다. 이번 지원 fixture 64 node의 최대 오차는 0이며 허용치는 0.5 CSS px다. |
| 20 | paint-rank와 hit-test 결과 차이를 픽셀 결과로 오해하는가 | `row-reverse` 한 표본에서 순서 후보가 다르다. `elementsFromPoint`는 이 계약의 paint oracle이 아니고 scene별 Android pixel readback이 없으므로 일치나 불일치로 승격하지 않고 미해결로 기록했다. |

## 결과와 미완료 범위

수정된 Android Debug APK에서 26개 case가 끝났고 21개 지원 case 및 5개 거부 case가 기대 분류·진단과 일치했다. 21개 지원 장면의 fixture node 64개 geometry frame은 고정 Chromium reference와 0 CSS px 차이였다. 캡처에는 기본 10-node GPU scene과 26/26 matrix 상태가 보이지만 26개 scene별 GPU surface 출력은 아니다.

이번 작업에서 확인하지 않은 항목은 scene별 Android surface 제출·pixel readback, iOS 전체 matrix와 Metal surface pixel readback, allocator failure injection, 깊은 graph stress, Android 하드웨어 GPU 경로다. Chrome 154 및 고정 WPT 실행은 reference 관찰이지 Spinon WPT 적합성 증거가 아니다. C12.4는 이 검토로 완료 처리하지 않는다.

## 후속 검증 정정

이 목록은 Android matrix 구현 검토 당시의 미실행 항목이다. 이후 Android `PixelCopy` 21 scene·22 sample, iOS Metal surface reverse Flex 8 sample, painter `Vec::try_reserve` 실패 주입, 50,000단계·폭 합성 stress, 실제 HostDocument 깊이 제한, iOS 26-scene geometry matrix를 각각 실행했다. 남은 경계는 Android 하드웨어 GPU·전체 iOS scene surface pixel, Spinon WPT runner 적합성, 전역 allocator OOM recovery 및 실기기/thread별 memory 귀속이다. [최신 종합 evidence](./c12-4-stacking-runtime-2026-10-11/README.md)와 [후속 검증 표](./c12-4-follow-up-verification-2026-10-11/README.md)를 기준으로 한다.

## 실행 근거

- [Android AVD matrix logcat](./c12-4-stacking-runtime-2026-10-11/android-api37-emulator-matrix-2026-10-11.log)
- [frame 비교 JSON](./c12-4-stacking-runtime-2026-10-11/android-api37-emulator-matrix-2026-10-11.json)
- [Android AVD 캡처](./c12-4-stacking-runtime-2026-10-11/android-api37-emulator-matrix-2026-10-11.png)
- [runtime diagnostic unit test](../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs)
- [matrix generator test](../../../tools/css-reference/c12-4-runtime-matrix.test.mjs)
