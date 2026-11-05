# C11.1 intrinsic Grid fixed-px gap · PR #1172 변경분 적대 검토 30개

**검토 대상:** PR #1172의 base `b76bf7daa3ecf55c8baa0364100e189fce542872` 이후 구현·테스트·명세·실행 근거·공개 roadmap 변경. 계획 검토 20개와 구현/runtime 검토 20개는 이 표에 합산하지 않았다.

| # | 공격 관점 | 확인과 판정 |
|---:|---|---|
| 1 | fixed gap 허용이 다른 Grid profile 전체에 번지는가 | `root_html_intrinsic_gap_is_supported`는 intrinsic validator 경로에서만 호출된다. **통과**. |
| 2 | 기존 one-axis non-intrinsic placement가 nonzero gap을 허용하게 되는가 | `intrinsic_track_sizing == false` 분기는 기존 fixed-zero gap predicate를 유지한다. **통과**. |
| 3 | row와 column gap이 반대로 연결되는가 | 입력 validator, Taffy track positions 검사, four-scene trace가 각 축 gap과 track 목록을 분리한다. **통과**. |
| 4 | shorthand와 longhand의 computed 값이 다른 adapter를 타는가 | Chrome/runtime fixture는 column longhand, row longhand, both-axis shorthand를 비교하고 같은 `LayoutGap` 경로를 사용한다. **통과**. |
| 5 | CSS-valid percentage가 fixed px로 잘못 평탄화되는가 | 10% oracle은 Chrome에서 used 10px로 기록되지만 runtime negative는 cascade-ready 상태에서 `column-gap`으로 거부된다. **통과**. |
| 6 | CSS-valid math-backed `calc()`가 계산되어 우연히 허용되는가 | `calc(5px + 5px)` runtime mutation이 computed CSS로 유지되고 해당 profile에서 원자 거부된다. **통과**. |
| 7 | CSS-invalid 음수 gap이 valid unsupported computed value처럼 layout을 실패시키는가 | `gap:-1px` mutation은 Stylo fallback 뒤 zero-gap control과 같은 frame으로 layout-ready다. **통과**. |
| 8 | 직접 입력의 negative·NaN·infinity가 helper를 통과하는가 | direct layout tests가 invalid typed 값 각각을 `InvalidStyle`의 정확한 field로 거부한다. **통과**. |
| 9 | gutter 개수를 track 수와 같게 계산해 마지막 이후 gutter가 생기는가 | aggregate 계산은 `explicit_track_count.saturating_sub(1)`을 쓴다. **통과**. |
| 10 | 큰 유한 gap 여러 개의 aggregate가 `f32`를 넘는가 | 세 explicit columns와 `f32::MAX` gap 입력을 Taffy 전에 `column-gap` diagnostic으로 거부하는 회귀가 있다. **통과**. |
| 11 | gutter aggregate는 finite지만 intrinsic base와 gap의 합이 overflow하는가 | 두 min-content columns에서 각각 `0.75 × f32::MAX`인 base/gap을 구성하면 기존 Taffy 결과는 `NaNpx`였다. resolved positions 검사를 추가해 frame 수집 전에 거부한다. **수정 후 통과**. |
| 12 | 사용하지 않는 빈 explicit track의 NaN이 유한한 item frame 뒤에 숨는가 | 검사는 visible frame이 아니라 `DetailedGridInfo`의 모든 track start/end를 대상으로 한다. **통과**. |
| 13 | Taffy 상세 Grid 정보가 없을 때 검사를 건너뛰고 성공하는가 | gap이 양수인데 detailed Grid info가 없으면 대응 gap property로 fail-closed한다. **통과**. |
| 14 | column overflow가 row property로 또는 반대로 보고되는가 | 실제 resolved columns/rows position과 같은 축의 typed gap/property를 묶어 진단한다. 직접 overflow test는 양축 property를 확인한다. **통과**. |
| 15 | 오류 경로가 partial frame 또는 paint snapshot을 publish하는가 | aggregate overflow, track-plus-gutter overflow, percentage, calc runtime negatives는 모두 `completed.is_none()`을 요구한다. **통과**. |
| 16 | positive result의 layout과 cascade가 서로 다른 revision을 쓰는가 | positive runtime verifier가 `completed.key == cascade_completed.key`를 확인한다. 이번 slice는 별도 asynchronous stale-race를 주장하지 않는다. **통과**. |
| 17 | gap이 intrinsic minimum contribution을 바꾸거나 두 번 빠지는가 | 고정 min-content bases와 gutter trace를 독립 expected 및 four-case direct layout regression에 대조한다. **통과**. |
| 18 | definite `fr` free space에서 gutter를 빠뜨리는가 | column·row only와 both-gap case에서 `[40,50]`, `[20,70]` track 결과를 expected와 비교한다. **통과**. |
| 19 | 지원 밖 implicit tracks나 auto-placement composition이 우연히 수용되는가 | existing implicit-track·mixed placement rejection regression은 유지되고 새 helper는 explicit track 수만 받는다. **통과**. |
| 20 | Root-relative item frame 비교가 Grid origin 또는 item 수를 무시하는가 | runtime matrix는 4 scene × 3 item의 root-relative frame을 비교한다. **통과**. |
| 21 | 독립 expected가 Spinon 출력에서 복사되어 circular oracle이 되는가 | JSON은 100px definite-axis 산술과 fixed item contribution에서 계산되며 Chrome observation은 별도 보존된다. **통과**. |
| 22 | Chrome capture가 다른 browser/viewport나 다른 입력을 측정하는가 | precomparison README는 Agent Browser HeadlessChrome 156, 1280×633, DPR 1 및 fixture/observation hash를 고정한다. **통과**. |
| 23 | 수정된 precomparison fixture가 실행 fixture와 장면 구성이 달라지는가 | 네 fixed-gap scene의 selector, item placement와 expected JSON을 대조했다. **통과**. |
| 24 | runtime mutation이 fixture marker를 찾지 못한 채 원본을 재실행하는가 | `replacen` mutation tests는 각 source 변경 직후 `assert_ne!`를 요구한다. **통과**. |
| 25 | cascade rejection을 CSS parser failure로 오분류하는가 | percentage/calc/overflow rejection에서 cascade는 `Ready`이고 layout만 실패해야 한다. **통과**. |
| 26 | 앱이 test-only fixture FFI나 fixture build flags로 성공하는가 | Android/iOS logs는 공통 C04 runtime만 켜고 C11 fixture, generic fixture flags는 0을 기록한다. **통과**. |
| 27 | Android screenshot을 실기기·hardware GPU·surface readback 근거로 확대하는가 | evidence는 API 37 ARM64 AVD의 ANGLE/Vulkan/SwiftShader와 visual smoke로 한정하고 PixelCopy를 주장하지 않는다. **통과**. |
| 28 | iOS screenshot을 실기기·compositor 이후 pixel 근거로 확대하는가 | iPhone 17 Pro iOS 26.2 Simulator의 draw/present만 주장하며 Metal drawable readback을 명시적으로 제외한다. **통과**. |
| 29 | 앱에 fixture를 주입한 뒤 원래 asset을 복구하지 않거나 hash가 다르게 남는가 | 빌드 script는 EXIT trap으로 Android/iOS asset을 되돌리고 원본 hash 두 개가 동일함을 evidence에 보존한다. **통과**. |
| 30 | status·내부 계약·공개 roadmap·PR body·evidence가 지원/완료와 빌드 범위를 서로 다르게 주장하는가 | C11.1/C11/C11.4 미완료와 공식 `78/214`를 유지하고 fixed-px-only 범위, simulator 한계, 계획·evidence·실제 PR 링크 및 screenshots를 대조한다. Full docs build의 dead-link 실패를 성공으로 세지 않고 README에 기록했으며 checksum manifest도 새 review 파일을 포함한다. **통과**. |

## 결과

서로 다른 30개 실패 관점을 확인했다. 10–14번에서 발견한 aggregate 및 track-plus-gutter overflow 경계는 property-specific fail-closed 검증과 회귀로 수정됐다. 이번 PR diff에서 추가 수정이 필요한 결함은 찾지 못했다. `spinon-layout` 전체 suite는 448개 중 447개 통과이며 변경 밖 C15 assertion 하나를 별도 기록한다. PR check rollup이나 CI 성공은 이 검토 결과에서 추론하지 않는다.
