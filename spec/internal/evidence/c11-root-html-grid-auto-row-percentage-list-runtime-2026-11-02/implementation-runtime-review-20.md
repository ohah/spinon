# C11.1 percentage implicit row 목록 구현/runtime 실패 관점 검토 · 20개

**대상:** Root HTML typed cascade, style-to-layout validation/projection, Root HTML Grid placement/layout, parsed-document Taffy 사용 geometry 및 Android/iOS Simulator 실행.
**기준:** [계획](../../../../plan/c11-root-html-grid-auto-row-percentage-list.md), [Chrome/독립 oracle](../c11-root-html-grid-auto-row-percentage-list-precomparison-2026-11-02/README.md), 이 디렉터리의 source/build/runtime hash 및 Rust 로그.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 기능 허용 플래그가 다른 computed style profile까지 번짐 | percentage 허용은 `RuntimeRootHtmlTextPaintV1` cascade 옵션에서만 켠다. 계산 함수는 기본적으로 `false`; 기존 fixed-list 옵션은 별도로 유지한다. |
| 2 | `grid-auto-columns` percentage도 함께 허용됨 | columns 호출은 `allow_percentage_track_lists: false`로 고정한다. Root HTML projection/layout은 implicit columns 전체를 거부한다. |
| 3 | 기존 단일 px scalar가 목록으로 바뀌어 ABI/계약 표현이 깨짐 | 직접 단일 px는 `LengthPx` 경로를 유지한다. cascade 7개 회귀와 기존 fixed-list layout test가 통과한다. |
| 4 | 단일 percentage가 scalar로 잘못 표현되어 cyclic list semantics를 잃음 | single percentage만 `TrackList([Percentage])`로 보존한다. parsed Root HTML computed value test에서 typed representation을 assert한다. |
| 5 | 두 개 이상의 mixed px/% entry에서 순서나 단위가 사라짐 | typed list의 entry 순서와 enum 단위를 보존한다. projection과 frame tests가 `[15px, 25%]` 및 `[10%,20%]`를 확인한다. |
| 6 | explicit rows 다음 목록 index를 0으로 다시 시작하지 않음 | 첫 implicit row에서 pattern index 0을 쓴다. explicit row를 포함한 수치 frame test에서 첫 implicit top `25px`를 확인한다. |
| 7 | percentage basis에서 row-gap을 빼거나 track 끝에 gap을 누락 | basis는 definite `height` 그대로 사용하고 row-gap은 배치 간격으로 별도 적용한다. mixed frame의 `120px` basis와 `5px` gap 결과가 독립 oracle과 일치한다. |
| 8 | definite 0px height가 indefinite로 오인되거나 division 오류 발생 | 0px basis에서 50% implicit track은 0px가 된다. fixed item의 `y=5,height=7` overflow도 보존한다. |
| 9 | 100% 초과 percentage를 100%로 clamp | `[0%,120%]`에서 두 번째 row와 마지막 item의 50px visible overflow를 유지한다. Rust layout/parsed integration tests가 확인한다. |
| 10 | row gap을 percentage basis에서 빼서 120%보다 작게 resolve | `15px 25%`와 120px height에서 25%는 30px이고, gap은 별도 5px다. row starts `[0,25,45,80,100]` 및 bottom 130px을 검사한다. |
| 11 | percentage row-gap을 implicit-row feature와 묵시적으로 합성 | parsed Root HTML integration에서 `row-gap:5%`는 정확한 `row-gap` property 오류로 거부한다. |
| 12 | `height:auto`, math, min/max가 definite basis를 바꾸는데도 허용 | parsed cascade→projection path에서 `height:auto`, `calc()`, non-auto min/max를 각 property로 거부한다. layout-level typed-input guard도 재검사한다. |
| 13 | aspect ratio가 height basis를 바꿀 수 있는데 무시 | non-auto computed `aspect-ratio`는 Root HTML percentage track 경계에서 거부한다. `auto` default만 허용된다. |
| 14 | syntax-invalid 음수 선언과 syntax-valid-but-unsupported 목록을 혼동 | `grid-auto-rows:-10%`는 cascade fallback을 타고, valid calc/fr/intrinsic 목록은 Unsupported 결과로 거부한다. cascade test에서 별개 사례로 확인했다. |
| 15 | NaN/Infinity/음수 또는 중첩 calc가 projection을 통과 | 각 direct px/% 값은 finite non-negative를 요구한다. negative/NaN/infinity, fraction/mixed unsupported entries, empty list가 projection 및 layout tests에 포함된다. |
| 16 | 목록이나 확장 rows가 resource ceiling을 우회 | 입력 목록과 최종 rows 모두 `GRID_MAX_TRACKS` 기준을 검사한다. implicit row가 없어도 입력 목록 크기 제한을 적용한다. |
| 17 | 단일 percentage resolve는 유한해도 전체 tracks/gap 누적이 f32 overflow | 각 resolved percentage와 explicit+implicit row/gap aggregate를 배치 전에 f64로 합산해 f32 상한을 검사한다. 개별·누적 overflow test가 오류를 assert한다. |
| 18 | `grid-template-areas` 또는 다른 profile이 이 확장으로 우연히 통과 | Root HTML named-area profile은 percentage implicit list를 거부하고, named-area tests가 fixed/% 모두 경계를 확인한다. |
| 19 | auto-placement가 DOM source order나 revision owner를 변경 | 한 개 root/single-grid 기본 route에서 역순 semantic IDs와 class를 JS probe로 확인한다. Android/iOS 모두 document/render-tree revision 2 및 13-box draw/present를 기록한다. |
| 20 | smoke screenshot을 geometry/pixel conformance 증거로 과장 | numeric frame tests와 simulator logs를 나눈다. screenshot은 visual capture이며 PixelCopy/Metal readback, hardware GPU, WPT 및 전체 CSS conformance를 수행하지 않았다. |

## 수정 및 재확인

- 첫 scoped Clippy에서 cascade helper 인자 수가 기준을 넘는 것을 발견해 `ComputedGridAutoTrackListOptions`로 묶었다. 같은 세 crate all-target Clippy를 `-D warnings`로 재실행해 통과했다.
- 소스 수정 뒤 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 앱을 pass03으로 다시 빌드·실행했다. APK와 iOS executable의 설치본 SHA-256이 각각 빌드 산출물과 일치한다.
- 이 검토는 계획 단계의 20개 관점과 별도이며 외부 reviewer나 GitHub CI run을 뜻하지 않는다. 전체 구현/공개 CSS 지원 완료는 주장하지 않는다.
