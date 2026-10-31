# C11.3.3 template-area implicit track의 typed `grid-auto-*` 크기

**상태:** 구현·Chrome 비교·Android API 37 AVD/iPhone 17 Pro iOS 26.2 Simulator 행렬·static FFI archive 경계·구현/runtime 검토 20개·PR diff 적대 검토 30개를 마쳤다. area 생성 typed track은 양 플랫폼에서 각 7/7 surface sample exact다. trailing positive named-line typed phase는 Rust layout test로 확인했다. PR 생성·필수 check 확인·병합은 아직 남았다. 이 후보는 제한 후속이며 C11.3.3 전체, C11, 공개 CSS 지원 및 공식 완료 수 `72/213`을 완료로 바꾸지 않는다.

## 결손과 목표

현재 `RuntimeBlockFlexGridTemplateV1`의 area 경로는 `grid-auto-rows`·`grid-auto-columns` 목록을 px만 허용한다. 제한 layout 입력도 `TemplateGrid`/`TemplateGridImplicitTracks`의 area-backed track list에서 `LengthPx`만 허용하고, positive implicit expansion의 `TemplateAreaImplicit` 모드도 같은 제약을 다시 검사한다. 그 결과 이미 C11.4 `TrackSizing`에서 처리하는 `minmax()`, percentage, `fr`, intrinsic keyword, `fit-content()` 및 `calc()`가 template area가 부족한 explicit track을 채우거나 그 뒤의 positive implicit named line을 확장할 때 거부된다.

기존 기능을 조합해 다음만 허용한다.

- `RuntimeBlockFlexGridTemplateV1`에서 computed `grid-template-areas`가 있고 `grid-auto-*` 값이 비어 있지 않은 supported typed track list일 때 area가 요구하는 누락 track을 그 목록 순서로 생성한다.
- `TemplateGrid` layout 경로에서 이미 생성한 area track의 typed sizing을 Taffy에 전달한다.
- `TemplateGridImplicitTracks`의 area-backed trailing positive named-line expansion이 같은 typed list를 순환하고, area 생성에 소비된 목록 phase를 보존한다.
- 허용되는 track 문법과 CSS math projection은 기존 C11.4 `TrackSizing` subset을 재사용한다. 새 track grammar나 sizing 알고리즘은 추가하지 않는다.

영역 없는 template/implicit 경로, 빈 목록, malformed·unsupported 값, allocation/quota 실패는 계속 거부한다. `TemplateGridSignedImplicitTracks`의 leading/negative extent, independent numeric implicit placement, 일반 author admission, auto-repeat + area 조합, writing mode/RTL, WPT 전체 적합성은 이 slice 범위에 넣지 않는다. Signed 경로는 현재 fixed-px 계약을 유지한다.

## 비교 기준과 고정 입력

- 규범: [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.3, §7.5–7.6, §8.5, §11.4–11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/). CSS Grid의 implicit track 생성·sizing과 track sizing 단계를 기준으로 사용한다.
- 독립 oracle: [Chrome 156 fixture](../spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-precomparison-2026-10-31/grid-auto-area-sizing.html), [관찰 JSON](../spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-precomparison-2026-10-31/chrome/observation.json), [screenshot](../spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-precomparison-2026-10-31/chrome/fixture.png), [캡처 이력과 한계](../spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-precomparison-2026-10-31/chrome/README.md). Headless Chrome 156, 1280×633 CSS px, DPR 1의 관찰이다. Agent Browser CLI 0.31.1을 사용했으며 Terminal Browser는 쓰지 않았다. Spinon 결과나 WPT 적합성으로 간주하지 않는다.
- 고정 비교는 fixed-list cycling, `minmax(20px, 1fr)`, percentage cycling, min-content, max-content, `calc()`, `auto`의 used track과 item frame을 포함한다. 현재 `fit-content(40px)` 장면의 fixed 60px child는 min-content contribution 자체가 60px일 수 있어 cap을 증명하지 않는다. 새 cap 판정에는 min-content가 40px 이하이고 max-content가 40px보다 큰 별도 content 입력을 사용한다.
- 직접 기대값은 definite root 크기·explicit track 크기·gap 0과 CSS sizing 단계로 산출한다. 예를 들어 root 높이 90px, explicit row 10px, implicit rows 두 개, `minmax(20px, 1fr)`이면 각 implicit row는 40px이다. Chrome은 이를 교차 확인한다.

## 구현과 검증 계획

1. Stylo extraction은 이미 cascade된 유효 `grid-template-areas`와 `RuntimeBlockFlexGridTemplateV1` 조건을 함께 만족할 때만 auto-track 값에 기존 C11.4 `track_size(..., true)` 변환을 사용한다. 명시 `grid-template-*` track grammar와 area 없는 profile은 그대로 둔다. Style projection도 같은 area/profile gate로 기존 nonempty `project_tracks` grammar를 열고 CSS math 값을 checked projector로 보낸다.
2. Area 확장은 authored track을 보존하고 부족한 track만 기존 `GridAutoTrackSize` pattern에서 복사한다. 목록 phase는 생성 수만큼 정확히 이동해 후속 positive named-line implicit track이 올바른 다음 항목을 쓴다.
3. Layout validation은 area-backed `TemplateGrid`/`TemplateGridImplicitTracks`에서만 기존 typed track validation을 사용한다. Area 없는 경로와 signed leading/negative 경로에는 새 sizing을 열지 않는다. `TemplateAreaImplicit`의 추가 trailing expansion도 nonempty typed list를 허용한다.
4. Style-to-layout 테스트에서 rows·columns 양 축의 missing-area track과 `grid-auto-*` typed list 전달, percentage basis와 `calc()` projection, 기존 list phase 및 fixed-px 회귀를 확인한다. Layout 테스트에서 `minmax/fr`, intrinsic track 전달, trailing named-line expansion과 consumed phase를 확인한다. 새 positive named-line test는 양축 `[10px,25%,8px]` 중 area가 두 항목을 소비한 뒤 `8px,10px` 순환과 frame `(70,60,18,18)`을 확인한다. `fit-content(40px)` cap의 별도 min/max-content fixture는 현재 simulator matrix에 넣지 않았으며 cap을 검증했다고 주장하지 않는다.
5. Negative controls는 area 없는 TemplateGrid/TemplateGridImplicitTracks, signed/leading mode, 빈 목록, unsupported/malformed 값, 음수·NaN·infinite component, track quota 초과, allocation 오류를 확인한다. 실패 시 partial frame/track output이 없어야 한다.
6. C11.3.3 fixture matrix는 typed row/column area scenes를 더해 39 scene·126 node가 됐다. 최종 Rust 수정 뒤 앱을 다시 빌드해 Android API 37 ARM64 AVD/SwiftShader 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 전체 geometry를 비교했다. typed area visible controls도 PixelCopy/Metal drawable에서 각 7/7 exact다. iOS 표본은 compositor 제출 전이다.
7. `cargo fmt --all -- --check`, layout 9/9, style cascade 1/1, style-to-layout 1/1, opt-in FFI 1/1, Node C11.3.3/fixture-boundary 50/50, workspace all-target Clippy를 통과했다. 기본 `spinon-ffi` Release archive에서 신규 scene marker와 C11.3.3 fixture export가 없고, opt-in positive control에는 두 scene ID·source marker·`new`/`eval` export가 있음을 확인했다. [실행·한계](../spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-runtime-2026-10-31/README.md).
8. 이 계획의 [사전 실패 관점 검토 20개](../spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-plan-review-20.md), 구현 뒤의 [별도 구현/runtime 검토 20개](../spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-runtime-2026-10-31/implementation-runtime-review-20.md), 실제 변경의 [PR diff 검토 30개](../spec/internal/evidence/c11-3-3-template-area-auto-track-sizing-runtime-2026-10-31/pr-diff-review-30.md)를 마쳤다. 다음은 Rebase and merge 전 필수 check/merge state 확인이다.

## 완료 조건

- Chrome fixed input의 supported computed track list, used rows/columns, item frame이 independent CSS 계산과 일치한다.
- `TemplateGrid` area extension과 `TemplateGridImplicitTracks` positive trailing expansion 모두 typed pattern과 pattern phase를 보존한다.
- 두 축 및 typed sizing 분류를 확인하고, 영역 없는 경로·signed 경계·invalid input이 계속 거부된다.
- 최신 소스 simulator geometry와 적용 가능한 표면 capture, Rust/fixture/artifact gates, 별도 구현/runtime 20관점 검토가 통과한다.
- PR diff 30관점 검토와 PR 병합이 끝난 뒤에도 제한 범위·실행 한계·공식 상태가 서로 일치한다.
- 내부 계약 0059와 C11 계획의 제한 결과·제외 범위를 업데이트한다. 이 slice만으로 C11.3.3, C11 또는 공식 완료 수를 갱신하지 않는다.

## 계획 단계에서 주장하지 않는 것

Chrome observation은 Chrome 156에 한정되며 Spinon 동작·WPT 적합성의 증거가 아니다. 기존 fit-content screenshot은 cap 근거가 아니다. Simulator는 실제 기기/hardware GPU나 iOS compositor 뒤 pixel을 검증하지 않는다. C11.3.3 candidate와 전체 C11은 미완료다.
