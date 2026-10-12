# C11.2.1 · 명시 Grid line 배치 구현 계획

**상태:** 제한 profile 구현 · Android API 37 AVD, Android 16/API 36 SM-S731N 실기기, iPhone 17 Pro/iOS 26.2 Simulator의 10/10 scene 근거 보존 · 2026-10-13 Android 초기 eval 대체 응답을 좁게 분류하고 비교기 상태·marker 검사를 보강 · 수정 뒤 Android 실기기/iOS Simulator 각 10 scene 재실행 통과 · 성공 scene 표면 표본 각 20/20 exact · 기대 거부 4/4 no-draw · 고정 WPT 후보 Chrome 기준 4/4(Spinon 적합성 아님) · 코드는 아직 미통합 · superseded 분기는 수정 후 실기기 rerun에서 재발생하지 않아 직접 재현은 미확정 · Spinon WPT runner는 X16에서 별도 추적하며 이 단계의 완료 관문이 아님
**상위 원본:** [구현 상태 대장](../spec/STATUS.md) · [C11 Grid 전체 계획](c11-css-grid.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)
**구현 전 기준:** 고정 Chromium 154 · 현재 C11.1 typed grid 입력 · Taffy `0.14.0`
**계획 실패 관점 검토:** [보완 검토 결과](../spec/internal/evidence/c11-2-1-explicit-grid-placement-plan-review-v2-2026-10-12.md)
**구현 실패 관점 검토:** [20개 실패 경계와 수정 결과](../spec/internal/evidence/c11-2-1-grid-placement-implementation-review-2026-10-12.md)
**후속 Android 환경 대체 응답:** [실패 재현·수정 후 플랫폼 실행](../spec/internal/evidence/c11-2-1-android-environment-supersede-2026-10-13/README.md) · [새 구현 실패 관점 검토](../spec/internal/evidence/c11-2-1-android-environment-supersede-implementation-review-2026-10-13.md)
**Chromium 사전 비교:** [고정 reference](../tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json) · [fixture inventory](../tests/fixtures/css/c11/grid-placement-inventory.json)

## 목표와 경계

C11.1의 자동 row placement만 있는 Grid에서 다음으로, CSS numeric line과 positive span을 사용해 item을 명시 배치한다. Stylo가 계산한 line 값을 엔진 중립 입력으로 옮기고 기존 Taffy custom tree에 적용한다. Android·iOS V8→Rust layout→WGPU 경로에도 별도 fixture를 연결한다.

이번 단계는 명시 track 안에 배치가 닫히는 제한 profile이다. Grid item의 `grid-row-start/end`, `grid-column-start/end` computed longhand를 입력으로 삼는다. CSS shorthand는 Stylo가 전개한 longhand가 두 축 모두 지원 가능한 definite line/span이 된 경우에만 지원한다. 따라서 `grid-row: 2`나 `grid-area: 2 / 4`처럼 한 축의 끝이 `auto`인 shorthand는 아직 지원하지 않는다. 자체 CSS shorthand parser는 만들지 않는다.

한 grid container의 보이는 in-flow child는 다음 중 한 가지 형태만 허용한다. container별로 두 형태를 독립 판정하므로 중첩·형제 Grid container가 서로 다른 형태를 쓰는 것은 허용한다.

- 모든 item placement가 `auto`: C11.1 row auto-placement 경로를 그대로 사용한다.
- 모든 item의 row·column 축이 각각 명시 track 안의 definite 범위로 정규화됨: C11.2.1 explicit placement 경로를 사용한다.

두 형태가 섞이거나 한 축만 명시된 item은 자동 배치 알고리즘과 implicit-grid 생성에 의존하므로 이 단계에서 전체 계산을 구체적인 오류로 거부한다. 빈 container는 C11.1 경로를 유지한다. `display:none` subtree는 item 분류와 배치에서 제외한다.

## 지원할 배치 값

- 양수 line은 explicit grid의 start edge부터 세며, 음수 line은 explicit grid의 end edge에서 역산한다. `0`은 허용하지 않는다.
- `span N`은 정수 `N > 0`만 허용한다. 한 축에 line과 span 하나가 있으면 양 끝 line으로 정규화하고, span만 있거나 두 span이 충돌하는 조합은 거부한다.
- 양 끝이 numeric line이면 CSS conflict handling에 따라 역순은 교환한다. 같은 line은 CSS에서 끝 값을 제거해 auto-placement에 의존하므로 이 단계에서는 거부한다.
- 정규화한 line 범위는 `1..=track_count + 1` 안이어야 한다. 범위를 벗어나 implicit track이 필요한 입력은 성공으로 clamp하지 않고 거부한다.
- 이름 line, named span, `auto`와 숫자 조합, implicit track, `grid-auto-rows`·`grid-auto-columns`, dense/column flow, mixed auto/explicit placement는 다음 세부 단계로 남긴다.
- CSS 선언 우선순위와 shorthand→longhand 전개는 Stylo cascade 결과를 따른다. 사용하지 않는 자식의 비지원 값 때문에 무관한 Grid container를 거부하지 않도록, item 값은 실제 보이는 Grid child에서만 검증한다.

## 구현 계약

1. 구현 전 Chromium 비교 입력을 추가한다. fixture inventory에는 node ID, authored declaration, computed placement longhand, rect, DPR/viewport, Chrome 버전·revision·실행 파일 hash·fixture hash·capture tool hash를 고정한다. 기존 C11.1 reference는 덮어쓰지 않는다.
2. 새 typed snapshot은 line/span/auto를 보존하고, named 값·정수 변환 overflow·할당 실패를 property와 node가 포함된 오류로 낸다. Stylo가 계산한 shorthand 결과와 author cascade 우선순위를 별도 assertion으로 고정한다.
3. `RuntimeBlockFlexGridV1`에 새 성공 의미를 몰래 추가하지 않는다. 새 profile 이름은 `RuntimeBlockFlexGridPlacementV1`로 고정하고 computed-style profile·FFI fixture entry를 분리한다. 기존 C11.1 사용자·fixture 동작은 바뀌지 않아야 한다. profile 추가 시 exhaustive match·paint 선택·모바일 runner가 빠지지 않았는지 확인한다.
4. layout adapter가 line을 Taffy에 넘기기 전에 CSS line 번호를 명시 track 개수에 대해 checked arithmetic으로 검증·정규화한다. Taffy의 0 처리, `i16` 변환, out-of-range clamp가 CSS 오류를 숨기지 못하게 한다.
5. 하나의 Grid subtree에서 placement 분류·검증·Taffy 계산이 모두 성공한 뒤에만 결과를 publish한다. 일부 item frame, 이전 revision frame, 부분 트랙 결과를 반환하지 않는다.
6. 중복 배치는 CSS상 허용한다. geometry, source/DOM order, paint order, hit-test를 각각 관찰한다. 이번 단계는 C12 stacking semantics나 `order`를 추가하지 않으며 기존 source order와 paint contract를 보존한다.
7. 명시 배치 item의 width·height는 `auto` 또는 fixed length만 허용한다. `auto`는 Taffy의 grid stretch를 사용하고, min/max·intrinsic content·text 측정은 후속 범위다. C11.1 auto-placement의 기존 fixed-size gate는 그대로 둔다.
8. 구현 전 고정 WPT revision에서 line·span·shorthand 의미에 맞는 후보를 골라 source SHA-256과 Chromium 실행 결과를 inventory에 저장한다. 적합한 단일 후보가 없으면 후보를 억지로 끌어오지 않고 자체 fixture와 미선정 사유를 기록한다. WPT 결과는 후보별로 관리하며 전체 Grid 적합성으로 일반화하지 않는다.
9. Rust 고정 fixture에서 line·span 변환과 오류 경계를 먼저 검증한다. 그 뒤 Android 연결 실기기와 iOS Simulator에서 동일 bundle fixture를 실행해 V8→Rust→WGPU 제출을 확인한다. 플랫폼별 실행 불가·surface 미수집은 별도 미완료로 기록한다.

## 사전 비교와 완료 관문

| 관문 | 통과 기준 | 실패 처리 |
| --- | --- | --- |
| Chromium oracle | positive/negative line, span, 역순, 같은 line, shorthand, cascade override, overlap, explicit-grid 경계의 computed value와 rect를 저장한다. 각 node frame의 최대 절대 오차 `0.5 CSS px` 이하. | 기준 변경·누락 node·중복 node·computed value 또는 rect 누락이면 fixture 자체 실패. |
| Stylo typed cascade | shorthand가 네 longhand에 올바르게 전개되고 author precedence가 유지된다. named/zero/overflow 값과 비 Grid child 입력이 성공으로 오인되지 않는다. | 지원 밖 declaration은 원인·node·property·값을 보존해 거부한다. |
| layout 변환 | 양·음수 line, 양의 span과 조합 규칙이 explicit track 범위 안에서만 변환된다. 결과 line이 Taffy의 origin-zero 의미와 일치한다. | line 0, 0 span, 음수 span, 넘친 정수, implicit boundary 요구, 혼합 auto/explicit은 계산 전체를 거부한다. |
| 실패 원자성 | invalid item이 앞·뒤에 있거나 여러 item이 겹쳐도 partial frame, stale revision, 잘린 track이 나오지 않는다. reservation/error injection은 실패 후 같은 worker에서 정상 후속 계산이 가능하다. | 입력 revision별 전체 결과 부재 및 기존 화면 snapshot 유지 여부를 기록한다. |
| 모바일 runtime | Android 실기기와 iOS Simulator에서 고정 scene 전부가 실행되고 node ID·frame·revision·WGPU present marker가 수집된다. Android와 iOS 결과를 동일 Chrome oracle과 비교한다. | 시뮬레이터를 실기기 근거로, present marker를 pixel parity로 승격하지 않는다. surface pixel 표본은 별도 명시된 실제 capture만 완료 처리한다. |

WPT 후보 원문과 Chrome 실행은 oracle 입력으로만 사용한다. 이번 단계는 Spinon의 전체 WPT runner·적합성 구현을 제외하며, 공통 runner는 X16의 소유 범위다. 이를 C11.2.1 완료의 선행 조건으로 사용하지 않는다.

구현 후 별도의 20개 실패 관점으로 실제 diff·테스트·런타임 로그를 검토했다. 누락된 nested container mode, hidden child, overlap paint snapshot 회귀를 추가했다. 남은 관문이 통과하기 전에는 C11.2.1을 완료 체크하지 않는다. C11, S02, X09, X16 또는 전체 CSS 지원도 완료로 표시하지 않는다.

## 이번 단계에서 하지 않는 일

Implicit track sizing과 CSS auto-placement 혼합, sparse/dense/column flow, named lines/areas, repeat/auto-repeat, `minmax()`·intrinsic sizing, track alignment, text/replaced item, absolute-positioned Grid child, WPT 전체 runner, 성능 우위 주장은 포함하지 않는다. 해당 항목은 C11.2.2 이후 및 C11.3–C11.6 계획의 소유 범위다.

## 현재 확인 결과

- 제한 profile `RuntimeBlockFlexGridPlacementV1` 및 별도 V8/FFI entry point를 구현했다. 양수·음수 numeric line, positive span, reversed line, shorthand cascade, overlap을 처리하고 zero/equal/named/implicit/mixed auto-explicit 입력은 지원 밖으로 거부한다.
- pinned Chrome 154와 비교한 style-to-layout fixture 9/9, runtime profile 3/3, FFI 인수 경계 1/1, layout-core normalization 4/4, nested/hidden 경계 2/2를 통과했다. C11.2.1 표면 실행 당시 저장소 suite는 CSS reference 336/336, Android touch tools 29/29, Rust workspace 618 passed·4 ignored, release depth 1/1이었다. 이후 Grid 테스트 모듈 분리 뒤 현재 작업 트리 전체를 재실행한 결과는 CSS reference 340/340이며, 상세는 [run-08](../spec/internal/evidence/c12-4-follow-up-verification-2026-10-12/current-source-rerun-20261012-08/README.md)에 있다. C11.2.1 당시 실행 로그는 [여기](../spec/internal/evidence/c11-2-1-mobile-surface-pixels-2026-10-12/full-test.log) · [Clippy](../spec/internal/evidence/c11-2-1-mobile-surface-pixels-2026-10-12/clippy.log), 별도 [구현 실패 관점 검토 20개](../spec/internal/evidence/c11-2-1-grid-placement-pixel-matrix-implementation-review-2026-10-12.md).
- Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 10-scene 행렬을 실행했다. 플랫폼별 성공 6/6·기대 거부 4/4, 21 fixture node, 최대 geometry 오차 0 CSS px였다. 성공 scene의 표면 sample은 두 플랫폼 각각 20/20 exact·최대 channel 오차 0이다. AVD는 ANGLE/Vulkan/SwiftShader 소프트웨어 renderer이고 iOS 값은 compositor 제출 전 drawable texture readback이다. [통합 표면 행렬·화면 캡처](../spec/internal/evidence/c11-2-1-mobile-surface-pixels-2026-10-12/README.md) · [Android 실행](../spec/internal/evidence/c11-2-1-grid-placement-android-runtime-matrix-2026-10-12/c1121-android-20261012T093216817Z-43dfc994/README.md) · [iOS 실행](../spec/internal/evidence/c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-12/c1121-iossim-20261012T093253773Z-8df46c33/README.md).
- Grid 테스트 모듈 분리 뒤 2026-10-13 현재 소스로 Android API 37.1 AVD, Android 16/API 36 SM-S731N 실기기, iPhone 17 Pro/iOS 26.2 Simulator 행렬을 각각 다시 실행했다. 세 대상 모두 10/10 scene, 성공 6/6·기대 거부 4/4, 21 node, 최대 geometry 오차 0 CSS px, 성공 scene PixelCopy/Metal 표본 20/20 exact·최대 channel 오차 0이다. [현재 소스 통합 결과](../spec/internal/evidence/c11-2-1-current-source-rerun-2026-10-13/README.md) · [Android 실기기 실행](../spec/internal/evidence/c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T194810238Z-5c74d6b6/README.md) · [iOS 실행](../spec/internal/evidence/c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T194925776Z-70f51679/README.md). AVD는 SwiftShader software renderer이며 iOS readback은 compositor 제출 전이다.
- 고정 WPT 후보 4개의 원문 hash를 재확인하고 Chrome `154.0.8037.98` 기준 WPT runner에서 4/4 통과, negative-line 7/7 subtest를 확인했다. 후보에는 이번 profile 바깥 기능이 포함되어 있어 Spinon WPT 적합성으로 해석하지 않는다. [실행 근거](../spec/internal/evidence/c11-2-1-wpt-chrome-2026-10-12.md).
- Android 실기기 full-scene PixelCopy 행렬은 [SM-S731N에서 10/10 scene으로 통과](../spec/internal/evidence/c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T194810238Z-5c74d6b6/README.md)했다. Spinon WPT runner·적합성은 이 단계 바깥에서 X16으로 추적한다. C11.2.1 코드는 아직 미통합이므로 C11.2.1·C11.2 및 C11 상위 완료 표시는 유지하지 않는다.
- 후속 Android 실행에서 초기 eval `status=-12`와 최신 환경 갱신 사이의 경합을 재현했다. Android는 `status=-12`와 내부 `op=eval status=0`이 함께 있을 때만 대체된 초기 장면으로 기록하고, 최종 summary는 현재 ready 환경 결과에서 한 번만 내도록 수정했다. 비교기는 초기 eval 오류와 최종 summary의 불일치, superseded marker 누락·순서 오류를 거부한다. 집중 검사 5/5, 전체 CSS reference 342/342 통과. 수정 후 Android 실기기와 iOS Simulator 10-scene 재실행은 통과했으나 superseded 분기는 재발생하지 않아 수정 후 실기기 직접 재현은 미확정이다. [실행 근거](../spec/internal/evidence/c11-2-1-android-environment-supersede-2026-10-13/README.md) · [구현 검토](../spec/internal/evidence/c11-2-1-android-environment-supersede-implementation-review-2026-10-13.md).


## 정확한 인덱스 통합 후보 재검증 · 2026-10-13

- 검증한 코드는 기준 커밋 `975d3356c9e8837834d4dd633fb3699084fad71a`의 staged tree `fb9411cb7cc7f767699447d29a202a8fadca1ece`다. C12.4 작업 트리 변경은 검증 복사본에 섞지 않았다. `ffi/c12.rs`에는 새 readback 인수 형식과의 단일 호출 호환 수정만 있다.
- 기존 600줄 이상 신규 테스트 파일 두 개를 cascade, 성공 배치, 숨김 문맥, 거부 경계 모듈로 나눴다. 동작 코드는 바뀌지 않았으며 전체 테스트를 정확한 인덱스 복사본에서 다시 통과했다.
- JS 2/2, CSS reference 304/304, Android 계측 도구 29/29, Rust 623 통과·4 ignored, Release depth 1/1, Clippy와 rustfmt를 통과했다. 상세는 [통합 후보 검증](../spec/internal/evidence/c11-2-1-index-candidate-2026-10-13/README.md)을 따른다.
- Android 16/API 36 SM-S731N 실기기와 iPhone 17 Pro/iOS 26.2 Simulator에서 10 scene 전체, 성공 6/6·기대 거부 4/4, 21-node frame, geometry 오차 0 CSS px, 20/20 surface 표본 exact를 확인했다. [Android run](../spec/internal/evidence/c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T203140900Z-69e4ca98/README.md) · [iOS run](../spec/internal/evidence/c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T203042604Z-f7983af0/README.md).
- C11.2.1 구현 검토 20개 관점은 [인덱스 후보 구현 검토](../spec/internal/evidence/c11-2-1-index-candidate-implementation-review-2026-10-13.md)에 기록했다. 아직 PR 전이며 통합 전 완료 표시는 하지 않는다. Android/iOS 실기기 외 기기군, compositor 이후 화면, 전체 WPT·Grid·CSS 지원은 미완료다.
