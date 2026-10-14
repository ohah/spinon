# C11.3.3 병합 후 증거 보강 구현 검토 · 실패 관점 20개

검토 대상은 direct typed-range regression, area 없는 bare-ident fallback fixture, Android `revision-reuse` scene와 capture runner다. 고정 26-scene matrix 계약과 C11.3.3 공개 지원 범위는 그대로 둔다.

| # | 실패 관점 | 판정 및 근거 |
| ---: | --- | --- |
| 1 | 잘못된 `scene_id` 문자열이 custom runtime source로 전달된다 | **PASS.** C ABI는 exact `revision-reuse`만 고르고 다른 ID는 기존 26-case allowlist lookup을 계속 쓴다. |
| 2 | custom scene이 26-case matrix count/summary에 들어가 이전 결과를 바꾼다 | **PASS.** custom source return은 고정 matrix parse 전에 분리돼 있고 matrix는 26/65 계약을 유지한다. |
| 3 | `MainActivity`가 후속 scene을 C11.1 검사로 잘못 보낸다 | **PASS.** `revision-reuse`만 C11.3.3 test route로 통과하며 AVD run에서 C11.3.3 host marker가 찍혔다. |
| 4 | Android demo allowlist가 기존 runtime matrix와 다른 scene을 허용한다 | **PASS.** 배열 자체는 26 IDs 그대로고 `revision-reuse`는 별도 exact special case다. 기존 JS test는 배열과 matrix ID 일치를 확인한다. |
| 5 | 두 eval이 새 host를 써서 stale snapshot을 관찰하지 않는다 | **PASS.** Java의 두 native eval 호출은 동일한 `host` 지역 변수와 pointer를 사용했다. AVD marker는 같은 host ID와 `same_host=true`를 기록한다. |
| 6 | V8 eval 사이 global state가 사라져 mutation 대신 재생성이 된다 | **PASS.** 두 번째 AVD 평가가 새 node를 만들지 않고 document node count를 유지하며 같은 node ID 2를 반환했다. |
| 7 | 첫 단계가 잘못된 초기 area를 만들어 이동 검증이 무의미해진다 | **PASS.** 첫 item frame이 Chrome/Rust layout contract와 별개로 명시한 `(0,0,80,40)`이었다. |
| 8 | mutation이 item `grid-area`도 바꿔 이동 원인이 혼합된다 | **PASS.** item style의 `grid-area:item`은 그대로 두고 root `grid-template-areas` 셀만 반전했다. |
| 9 | mutation이 layout profile에서 지원되지 않는 positioned root에 의존한다 | **PASS.** AVD fixture 시도에서 root absolute positioning의 unsupported profile 오류를 재현했다. <a href="./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-revision-20261014T032021358Z-5f461b1d/revision-reuse.log">실패 로그</a>를 확인해 position을 제거한 뒤 <a href="./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-revision-20261014T032109347Z-0ee50c0d/result.json">같은 테스트</a>가 통과했다. |
| 10 | 두 번째 eval이 실패해도 첫 결과만으로 통과한다 | **PASS.** Java는 두 단계 각각 `status=0`과 `layout=ready`를 요구하고, 실패하면 synthetic failure status로 정상 성공 경로를 막는다. |
| 11 | document revision만 증가하고 render-tree revision은 stale하다 | **PASS.** AVD 결과에서 document `10→11`, render-tree `2→3`을 별도로 비교했다. |
| 12 | 동일한 NodeId가 아니라 새 child frame을 잘못 비교한다 | **PASS.** 두 frame의 node ID 2가 같음을 Java marker와 capture runner가 각각 검사한다. |
| 13 | item frame이 실제 named area에 따라 이동하지 않는다 | **PASS.** x 좌표가 `0→80`, y·width·height가 `0·80·40`으로 유지되는지 AVD runner가 단언한다. |
| 14 | 앱 결과가 성공해도 실제 GPU presentation은 이루어지지 않는다 | **PASS.** 후속 run은 `SPINON_C0410_DRAW status=0 presented boxes=2`를 기다린 뒤 결과를 저장한다. |
| 15 | revision capture가 physical device나 둘 이상의 장치를 자동 선택한다 | **PASS.** runner는 `android-avd`만 허용하고 연결된 device serial이 지정 emulator 하나인지 확인하며 QEMU/API 37을 검사한다. |
| 16 | 다른 software renderer 결과를 SwiftShader AVD로 잘못 표시한다 | **PASS.** capture 단계에서 `dumpsys SurfaceFlinger`의 SwiftShader renderer를 요구하고 metadata를 보존한다. |
| 17 | malformed range 테스트가 CSS parser가 만든 정상 값만 검사한다 | **PASS.** Stylo typed `TemplateAreas` 값을 직접 구성해 실제 `computed_grid_template_areas` 경계에 전달한다. |
| 18 | row만 검증하고 column의 0·끝 초과·역전·empty를 누락한다 | **PASS.** 직접 주입 test의 8개 입력이 row/column 조건을 따로 포함한다. |
| 19 | bare ident fallback이 area alias 또는 `1 ident` 경로를 재사용한다 | **PASS.** pinned Chrome 154에서 `grid-template-areas:none`과 양 축 반복 `[panel]` line을 확인하고 item에는 bare `grid-area:panel`만 둔다. |
| 20 | 세 후속 결과가 전체 CSS Grid/WPT 또는 hardware 지원 완료로 오해된다 | **PASS.** evidence와 <a href="/spinon/spec/STATUS.html">상태 대장</a>은 API 37 AVD, 단일 Chrome case 및 제한 Rust fixture로 한정하고 전체 기능을 미완료로 유지한다. |

## 결과

20개 관점에서 추가 차단 결함을 찾지 못했다. 첫 simulator 실행의 failure는 product defect가 아니라 테스트 fixture가 선택한 unsupported positioned root였다. 이 속성을 제거한 후 동일 host mutation run이 통과했고, ADB에는 `emulator-5554`만 연결됐다. 사용자 요청에 따라 물리 Android 기기는 연결하거나 사용하지 않았다.

검증 기록은 [후속 evidence README](./c11-3-3-post-merge-followup-2026-10-14/README.md), <a href="./c11-3-3-post-merge-followup-2026-10-14/chrome-154-bare-ident-no-area.json">Chrome 154 관찰</a>, <a href="./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-revision-20261014T032109347Z-0ee50c0d/result.json">Android AVD 결과</a>에 연결돼 있다.
