# C11.4 · direct PNG `minmax(auto, max-content)` mixed tracks

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** Stage B image-only direct item/fixed-intrinsic mixed-column slice의 local implementation candidate가 추가됐다. Stage expectation correction은 [독립 derivation review](./c11-4-image-auto-max-mixed-track-trace-stage-derivation-review-2026-10-26.md)에서 PASS했고, focused tests 및 Android API 37 ARM64 AVD / iPhone 17 Pro iOS 26.2 Simulator의 corrected trace/frame matcher도 통과했다. [Runtime evidence matrix](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/README.md)는 build/artifact/source hash, screenshots와 `image_draw_count=0` 한계를 구분한다. pass01 `12 PASS / 8 PARTIAL / 0 FAIL`, pass02 `17 PASS / 3 PARTIAL / 0 FAIL`, pass03 `20 PASS / 0 PARTIAL / 0 FAIL`, pass04 `18 PASS / 2 PARTIAL / 0 FAIL`, pass05 pre-fix `14 PASS / 4 PARTIAL / 2 FAIL`은 각각 당시 payload의 historical review다. [Pass06](./c11-4-image-auto-max-mixed-track-plan-review-pass06-20-2026-10-26.md)는 README 문구 후속 확인을 반영해 `17 PASS / 3 PARTIAL / 0 FAIL`이며, [pass06 follow-up](./c11-4-image-auto-max-mixed-track-plan-review-pass06-follow-up-2026-10-26.md)이 row 17의 PASS 변경과 남은 범위를 기록한다. [PR diff-30 검토](./c11-4-image-auto-max-mixed-track-plan-pr-diff-review-30-2026-10-26.md)의 `27 PASS / 3 PARTIAL / 0 FAIL`은 trace contract correction 전 payload에 대한 historical score이고 current implementation review score가 아니다. 그 review의 남은 PARTIAL은 원격 PR diff/CI payload 부재, CSS Images raw response preimage 부재, Agent Browser cleanup 원문 log 부재다. PR/payload 부재는 remote PR 판정을 제한하며 나머지 두 항목은 provenance 한계다. Current branch base는 `040a5360be92f73e141797aa7dcd4ef00ab24942`; Chrome capture source는 별도로 `a33b245983fae050b66efb9ff259b3fe1132a051`이다. [상위 계획](./c11-4-grid-track-sizing.md)에도 review 및 runtime evidence를 연결한다. Overflow는 sizing slice의 runtime acceptance에서 제외한다. C11.4 공식 상태와 `72/213`은 그대로다.

## 목적과 기존 범위와의 관계

현재 C11.4에는 `minmax(auto,max-content)` 및 fixed/intrinsic mixed tracks 조합이 상위 [Grid sizing 계획](./c11-4-grid-track-sizing.md)에서 추가 implementation control로 남아 있다. direct image adapter는 고정 PNG natural-size 입력을 사용하지만 각 case를 별도의 root ID와 제한된 Track enum으로 허용한다. 이 계획은 direct PNG 하나가 자동 배치되는 column에 `minmax(auto,max-content)`를 두고 이웃에 fixed `17px` track 하나를 두어, 해당 intrinsic track contribution을 다른 fixed track 옆에서 검사한다.

이 slice는 image-only span-1 column case다. `min-content`/`max-content` direct text 조합을 일반화한 PR #611, row-axis mixed text/image PR #620, orthogonal text PR #630, cyclic auto-repeat PR #635를 되풀이하지 않는다. image의 fixed-gutter ratio, multi-span, cyclic percentage image slice는 각각 이미 별도 계획/구현 기록에 있다. 현행 C11.4 image adapter와 runtime source는 재사용하며 새 일반 CSS admission이나 새 이미지 로더를 목표로 삼지 않는다.

최초 precomparison capture는 `b58de5df93204ba211a025f20b5bfe3790c1d385`에서 했으나 PR #641 merge 뒤 Agent Browser fixture를 `a33b245983fae050b66efb9ff259b3fe1132a051`에서 다시 캡처했다. 현재 fixture/observation/metadata는 계속 그 capture source를 사용한다. 계획 branch는 이후 `040a5360be92f73e141797aa7dcd4ef00ab24942`로 rebase했다. pass03–pass06은 각자 당시 payload에 대한 별도 review이며 [pass06 follow-up](./c11-4-image-auto-max-mixed-track-plan-review-pass06-follow-up-2026-10-26.md)은 README 언어 판정을 갱신했다. 상위 계획은 각 review 결과와 overflow scope를 연결한다. 현재 local candidate는 이 계획의 단일 image case로 제한되며 일반 CSS admission이나 layout algorithm 변경을 포함하지 않는다. 직접 image/track의 기존 병합 후보 #357, #411, #520, #611 및 #620은 참고 이력이다.

## 입력과 독립 기대값

Chrome 재현 HTML은 [`grid-image-auto-max-mixed-track-c114-precomparison.html`](../tests/fixtures/css/c11/grid-image-auto-max-mixed-track-c114-precomparison.html)이다. 기존 PNG [`c14-1-png-basic.png`](../tests/fixtures/resources/c14-1-png-basic.png)는 97 bytes이며 natural size가 `24×16`이다. CSS 입력은 root border box `100×40px`, columns `minmax(auto,max-content) 17px`, rows `auto`, column gap `5px`, row gap `0`, `justify-content:start`, `align-content:start`, `align-items:start`다. Root와 image의 padding/border/margin은 0이고 CSS width/height는 `auto`다. Image에는 authored `overflow` 선언이 없으며, Chrome computed overflow는 별도 observation에 기록한다. item은 direct child로 첫 column에 자동 배치된다.

규범 산술과 asset/fixture hash는 [precomparison evidence](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/README.md) 및 [spec-derived-expectations.json](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/spec-derived-expectations.json)에 고정했다.

1. CSS Grid §6.6은 computed overflow가 non-scrollable이고 item이 min sizing function `auto`인 track을 span할 때 content-based automatic minimum을 허용한다. Positive HTML fixture와 V8 scene은 item overflow를 author하지 않는다. Chrome 156은 `<img>`의 computed `overflow-x/y=clip/clip`을 관측했으며 이는 scrollable overflow가 아니다. 이 값은 규범 계산의 applicability context일 뿐, Spinon의 computed overflow나 clipping/paint 동작을 검증하는 acceptance가 아니다. 이 slice는 overflow allowlist를 확장하거나 image adapter overflow guard를 구현하지 않는다. Computed `height:auto`는 반대 축 definite preferred size가 아니므로 transferred size suggestion은 없다. Specified width도 없고 min/max는 `auto/none`이어서 replaced image content-based minimum은 natural width의 `24px`다.
2. PNG는 CSS Images §4.1에서 말하는 natural dimensions가 있는 raster image다. width/height 지정 constraint가 없는 경우 §4.3.1은 natural dimensions를 specified size로 사용한다. 이에 따라 inline-axis min-content와 max-content contribution은 모두 `24px`, block-axis size/contribution은 `16px`다.
3. CSS Grid §11.4에서 intrinsic auto-min track은 base `0px`, intrinsic max track은 growth limit `+∞`로 시작한다. Fixed track은 base/growth limit 모두 `17px`다. definite `100px` container이므로 §11.5의 min/max-content constraint 특별 분기는 적용되지 않고, auto-min track base는 item의 minimum contribution으로 정해진다. width가 `auto`인 이 image는 §6.6의 natural-size automatic minimum `24px`가 그 contribution이 된다. Max-content growth limit도 같은 span-1 item의 `24px` max-content contribution으로 정해진다. §11.6 maximize는 더 키우지 않는다.
4. Final non-gutter columns는 `[24,17]px`; 사이 gutter는 `5px`다. 합계 `46px` 뒤에 남는 `54px`는 `justify-content:start`로 끝에 남는다. Auto row는 `16px`이고 `align-content:start`로 root의 남은 높이 `24px`는 row에 더해지지 않는다.

### §6.6 source와 trace acceptance

Image item에는 authored overflow declaration이 없다. Chrome CSSOM은 `clip/clip`을 기록하지만, 이 slice는 C11.4 overflow extraction/projection, clipping, paint 또는 scrolling behavior를 acceptance로 삼지 않는다. Image `Minimum` contribution source는 Columns와 Rows 각각 다음 전 필드와 일치해야 한다. `spans_auto_min_track=true`, `spans_multiple_tracks=false`, `spans_flexible_max_track=false`, `content_based=true`, `is_compressible_replaced=true`, `fixed_track_sum=null`, `fixed_gutter_sum=null`, `fixed_track_limit=null`. Contribution은 Columns `24px`, Rows `16px`; min-content/max-content도 두 축 각각 같은 natural size다. 다른 contribution kind의 `minimum_source`는 `null`이다.

CSS Grid §11.4에서 초기 intrinsic max track growth limit은 `+∞`다. Taffy trace는 `+∞`, `−∞`, `NaN` 등 모든 non-finite 값을 `None`/JSON `null`로 매핑하므로 `Initialize`의 trace `null`은 non-finite projection만 확인하며 내부 값이 `+∞`였음을 증명하지 않는다. CSS Grid §11.5는 이 span-1 max-content maximum의 growth limit을 item의 max-content contribution까지 resolve한다. Taffy는 해당 intrinsic item batch의 growth-limit update 뒤에 `IntrinsicItemBatch` trace를 기록하므로 이 stage부터 column `24px`, row `16px` growth limit은 finite `Some(value)`여야 한다. 이 stage boundary는 `vendor/taffy/src/compute/grid/track_sizing.rs`의 item-batch update와 trace emission, expectation JSON의 `/independent_derivation/expected_trace/stage_emission_contract`에 고정한다. [독립 derivation review](./c11-4-image-auto-max-mixed-track-trace-stage-derivation-review-2026-10-26.md)는 CSS spec와 Taffy source를 먼저 검토해 모든 slot을 확정하고 preserved run을 corroborating evidence로만 사용했다. Finite trace 값은 새 native matcher의 `0.0001px` tolerance 안에서 비교한다.

Taffy `GridTrackVec`는 CSS track뿐 아니라 grid line slot도 교차 저장하므로 CSS track `N`개는 raw trace에 `2N+1` internal entries를 만든다. 따라서 raw array를 CSS track 수와 혼동하거나 collapsed sentinel을 버리지 않는다. `gutter` 및 `collapsed` flags까지 모든 slot에서 검사한다. 모든 slot은 `infinitely_growable=false`여야 한다. Boundary sentinel은 `gutter=true, collapsed=true`, zero base/growth다. 실제 5px inter-column gutter는 `gutter=true, collapsed=false`다.

| Axis | raw internal slot mapping | Initialize | IntrinsicItemBatch (`span=1`, `crosses_flexible_track=false`) | IntrinsicSizingComplete | Maximize / ExpandFlexible / StretchAuto |
| --- | --- | --- | --- | --- | --- |
| Columns, 5 slots | `[leading sentinel, col0, gutter, col1, trailing sentinel]` | `[0/0, 0/null, 5/5, 17/17, 0/0]` | `[0/0, 24/24, 5/5, 17/17, 0/0]` | `[0/0, 24/24, 5/5, 17/17, 0/0]` | 각 단계 같은 배열; `StretchAutoTracks.applied=false` |
| Rows, 3 slots | `[leading sentinel, row0, trailing sentinel]` | `[0/0, 0/null, 0/0]` | `[0/0, 16/16, 0/0]` | `[0/0, 16/16, 0/0]` | 각 단계 같은 배열; `StretchAutoTracks.applied=false` |

각 pair는 `baseSize/growthLimit`이다. `Initialize`에서 intrinsic CSS track의 `null` growth projection만 기대하고, non-finite 내부 값이 `+∞`인지 trace로 주장하지 않는다. Batch stage에서는 contribution에 따라 finite growth가 기록돼야 한다. Columns contributions는 `Minimum=24`, `MinContent=24`, `MaxContent=24`; Rows는 각각 `16`이다. `MaximizeTracks`는 definite free space를 finite growth limit까지만 분배해 변동이 없고, `ExpandFlexibleTracks`는 flex track이 없어 변동이 없다. `StretchAutoTracks`는 해당 축 정렬이 `start`라 적용하지 않는다.

| 관찰 | 규범 기대 |
| --- | ---: |
| PNG natural size | `24×16px` |
| Column used sizes | `[24,17]px` |
| Gutter | `5px` |
| Row used size | `[16]px` |
| Root border-box frame | `(0,0,100,40)` CSS px |
| Image frame, root-relative | `(0,0,24,16)` CSS px |

이 수치가 Taffy 출력에서 유도된 것은 아니다. CSSOM 결과는 별개의 Chrome 비교 관측으로 보존한다. 규범 자료는 [CSS Grid Layout Module Level 1 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) §6.6, §7.2.1, §10.5, §11.3–§11.6; [CSS Box Sizing Module Level 3 2026-09-04 WD](https://www.w3.org/TR/2026/WD-css-sizing-3-20260904/) §2.2, §5.1–§5.2; [CSS Images Module Level 3 18 December 2023 CRD](https://www.w3.org/TR/2023/CRD-css-images-3-20231218/) §4.1, §4.3.1이다. CSS Images 원문 응답 bytes/hash와 snapshot revision은 evidence JSON에 기록한다.

## Chrome 사전 관측

Chrome은 Agent Browser 전용 session `spinon-c114-img-stageb-review04-20261026`에서만 실행했다. `navigator.userAgentData`의 full version은 Chrome `156.0.8078.48`, macOS `26.5.1` ARM64, DPR 1이다. 브라우저 viewport는 `320×80` CSS px다.

고정 HTML input에서 Chrome은 item `overflowX/Y=clip`, `width/height=24px/16px`, `min-width/height=auto`, `max-width/height=none`, `display:block`, `position:static`, grid placement `auto/auto`를 반환했다. Root computed track은 columns `24px 17px`, rows `16px`; root `(0,0,100,40)`, image `(0,0,24,16)`, natural size `24×16`이다. Computed/authored item/root styles와 frames는 [raw observation](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/chrome-agent-browser-observation.json)에 보존했다. 이 CSSOM은 Chrome oracle이고 Stylo C11.4 결과가 아니다. Image box가 PNG natural size와 같으므로 이 plan의 관측 범위는 sizing/geometry다. Overflow extraction, projection, clipping, scrolling, paint behavior는 검증하지 않는다. Agent Browser 전용 session은 `close` 후 `active:false,pid:null` 및 dedicated Chrome/crashpad 종료를 확인한다. 사용자/HWPJS 브라우저는 보존한다. local HTTP server는 실행하지 않았다.

## 소스 경로와 제한된 구현 slice

현재 source에서 재사용할 경로와 이 조합의 빈 곳은 다음과 같다.

| 역할 | 현재 경계 | 계획된 변경 |
| --- | --- | --- |
| Image fixture case 목록과 sizing | [`cases.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image/cases.rs#L1): `Track` enum, `IMAGE_CASES` 20개 | `Length17`, `MinMaxAutoMaxContent` track expectation과 root/item ID allowlist가 있는 image case 1개 추가; expected array size와 bounded-geometry test 갱신 |
| image subtree/source/style gate | [`c11_4_image.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs#L53) | 기존 1 direct HTML `img`, fixed PNG source, computed style, intrinsic snapshot 흐름을 유지. [`track_matches`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs#L556)에 정확한 `MinMax { min: Auto, max: MaxContent }` 및 fixed `17px` match 추가. Image adapter에 overflow-specific guard는 추가하지 않음 |
| Overflow behavior boundary | [`runtime_grid.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L126), [`style_values.rs`](../crates/spinon-style-to-layout/src/projection/style_values.rs#L878) | Fixture는 item overflow를 author하지 않는다. Overflow allowlist/projection/clip behavior와 image-adapter guard는 이 slice의 scope와 acceptance에서 제외한다. Chrome의 UA computed value는 별도 CSSOM observation일 뿐이다. |
| 앱 V8 scene source | [`c11_4.rs`](../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs#L826) `C11_4_IMAGE_TRACK_SCENES` 18개와 count assertion | scene ID `image-auto-max-mixed-tracks`, 동일 root/item ID, `100×40`, `minmax(auto,max-content) 17px`, auto row, 5px gap, image intrinsic sizing이며 overflow declaration이 없는 scene 하나 추가하고 list length/count 갱신 |
| Android/iOS scene·result·frame allowlist | Android [`C0410RuntimeGpuDemo.java`](../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java#L130) scene ID list, `#L2616` image intrinsic result, `#L2967` trace, `#L3445` root/item frame; iOS [`C0410RuntimeGpuDemo.swift`](../platforms/ios/Sources/C0410RuntimeGpuDemo.swift#L82) scene ID set, `#L2028` image scene set, `#L3389` trace, `#L3731` frame | `image-auto-max-mixed-tracks`를 양 플랫폼의 scene ID 목록, intrinsic input/result allowlist, trace matcher와 root/item frame matcher에 각각 등록. Frame은 root `(0,0,100,40)`, image `(0,0,24,16)` |
| feature 경계 | [`fixtures.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures.rs#L42)의 `c11-4-grid-track-sizing-fixture-test` | 이 opt-in fixture feature 안에서만 추가. 기본/product profile이나 일반 cascade admission에 추가하지 않음 |
| image resource snapshot | [`snapshot.rs`](../crates/spinon-image/src/snapshot.rs#L158) | 기존 24×16 decoded PNG → revision-bound intrinsic-size snapshot을 그대로 재사용; loader/API 변경 없음 |
| runtime layout handoff | [`runtime_layout.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L460), [`runtime_layout.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L599) | 이미 image input이 있는 경우의 `compute_runtime_style_layout_with_intrinsic_size_snapshot` 경로를 재사용. 새 text dispatch나 일반 image branch를 만들지 않음 |
| generic layout projection | [`projection.rs`](../crates/spinon-style-to-layout/src/projection.rs#L827), [`style_values.rs`](../crates/spinon-style-to-layout/src/projection/style_values.rs#L878) | public projection API와 `supports_overflow_clip` gate는 바꾸지 않는다. Overflow projection behavior는 이 slice의 acceptance 범위가 아니다. Candidate track trace가 expected source/stages와 다르면 algorithm correction은 별도 근거를 요구 |

Image adapter가 고정 `IMAGE_URL`을 `ImageRequest::from_document(...).decode(PNG)`에 넘기고 snapshot을 만든다. 이 앱 fixture는 live HTTP image loading을 검증하지 않는다. 새로운 root ID가 adapter allowlist에서 발견되지 않거나 CSS typed grid style이 정확히 일치하지 않으면 error로 닫힌다. Overflow declaration/admission과 image adapter overflow behavior는 이 slice에서 다루지 않으며, CSS feature 지원을 광고하지 않는다.

## Fail-closed negatives

구현 PR에는 positive scene 외 image adapter rejection controls를 포함한다. Overflow author declarations와 overflow-specific errors는 이 slice의 fail-closed matrix에서 제외한다. 나머지 controls는 existing adapter root/item/source/grid gates의 error boundary를 사용한다.

| 변이 입력 | 예상 error boundary 및 code/detail | atomicity 기대 |
| --- | --- | --- |
| Columns를 `minmax(min-content,max-content) 17px`로 교체 | `validate_grid_style`; `unsupported_c114_image_case`, `track functions` | image decode/snapshot 이전 거부; frame·trace 없음 |
| Root width/height, border, gap, row list 또는 alignment 변경 | `validate_grid_style`; `unsupported_c114_image_case`, `container box geometry` 또는 `track functions` | image decode/snapshot 이전 거부; frame·trace 없음 |
| Item placement 명시 또는 span 추가 | `validate_item_style`; `unsupported_c114_image_case`, `replaced item box or placement` | image decode/snapshot 이전 거부; frame·trace 없음 |
| 다른 root/item ID, extra direct child, nested/second/non-leaf image | `prepare` topology/source gate; `unsupported_c114_image_fixture_root` 또는 `unsupported_c114_image_case` (`one direct HTML img required`, `image tag or id mismatch`, `replaced leaf required`) | style validation와 image request 이전 거부; snapshot·frame·trace 없음 |
| Fixed PNG `src` 불일치 | `prepare` source gate; `unsupported_c114_image_case`, `fixed PNG source required` | decode와 snapshot 이전 거부; frame·trace 없음 |
| PNG bytes decode 실패 | `ImageRequest::decode`; `c114_image_decode` | resource snapshot이 만들어지지 않음; frame·trace 없음 |
| Decode 완료 이미지 natural size가 `24×16`이 아님 | resource dimension gate; `c114_image_metadata_mismatch` | 임시 `ImageResourceSnapshot`은 내부에서 만들 수 있지만 intrinsic-size snapshot 또는 `C11_4ImageInput`을 반환하지 않음; frame·trace 없음 |
| Stale/foreign completion revision 또는 document generation | `ImageResourceSnapshot::new`; `c114_image_resource_snapshot` | 생성된 snapshot을 게시하지 않음; intrinsic-size snapshot·frame·trace 없음 |
| C11.4 typed track profile 외 profile | profile gate; `c114_image_profile_mismatch` | subtree scan/resource 단계 전에 거부; 모든 snapshot·frame·trace 없음 |

## 구현 전·후 검증 게이트

1. Plan review 기록: pass01 `12 PASS / 8 PARTIAL / 0 FAIL`, pass02 `17 PASS / 3 PARTIAL / 0 FAIL`, historical pass03 `20 PASS / 0 PARTIAL / 0 FAIL`, pass04 `18 PASS / 2 PARTIAL / 0 FAIL`, pre-fix pass05 `14 PASS / 4 PARTIAL / 2 FAIL`, pass06 및 README follow-up 완료 후 `17 PASS / 3 PARTIAL / 0 FAIL`. Pre-correction plan diff-30은 `27 PASS / 3 PARTIAL / 0 FAIL`; current implementation trace contract는 [독립 derivation review](./c11-4-image-auto-max-mixed-track-trace-stage-derivation-review-2026-10-26.md)에서 targeted PASS를 받았다. 이는 full implementation diff review나 remote PR/CI verdict가 아니다. Historical plan-review PARTIAL은 원격 PR diff/CI payload 부재, CSS Images raw response preimage 부재, Agent Browser cleanup 원문 log 부재다.
2. Positive fixture/app scene에는 `overflow` declaration을 추가하지 않는다. Agent Browser CSSOM의 Chrome `clip/clip`은 observed metadata로 기록하지만 Spinon overflow extraction/projection, clipping, paint 또는 scrolling behavior는 이 slice에서 검사하지 않는다. Runtime acceptance는 image natural-size contributions, final track sizes, frame과 revision provenance로 제한한다.
3. `spinon-runtime` image fixture에서 새 case 및 fail-closed controls를 검사한다. `spinon-layout` trace에서 Columns/Rows span-1 `Minimum` source의 모든 flags/`None`, min/min-content/max-content 값, JSON에 고정한 stage arrays, gutter `is_gutter`, `infinitely_growable`, frame과 revision provenance를 전부 대조한다. Candidate 결과가 이 기대와 다르면 implementation output을 oracle로 역산하지 않고 규범 입력/계산을 다시 확인한 뒤 별도 수정한다. `cargo fmt --all -- --check`, focused tests, workspace test/Clippy는 구현 시 실행하고 실제 결과만 보고한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 등록한 `image-auto-max-mixed-tracks` scene을 실행한다. Source commit과 app artifact hash를 고정하고 root/item frame, full stage/contribution JSON, intrinsic revision, draw/present를 확인한다. Positive scene의 representative screenshots `c11-4-image-auto-max-mixed-track-android-api37-arm64.png`와 `c11-4-image-auto-max-mixed-track-ios-26.2-simulator.png`를 각각 저장해 PR 본문에 Markdown image로 첨부한다. PR 페이지가 준비되면 dedicated Agent Browser로 양쪽 image link가 실제 렌더/로드되는지 확인하고 Agent Browser `close` 뒤 session `active:false,pid:null`, 해당 Chrome/crashpad 프로세스 종료를 확인한다. 남의 사용자/HWPJS 브라우저는 보존한다.
5. 시각 근거 수준을 분리한다. JSON frame/stage는 layout evidence, draw/present는 submission evidence, simulator screenshot은 해당 simulator window의 대표 화면이다. Android PixelCopy와 iOS Metal texture/readback sample이 있으면 별도 paint/readback evidence로 기록하되 screenshot이나 submission과 동등하다고 취급하지 않는다. 어떤 결과도 실기기, hardware GPU 또는 iOS compositor 이후의 색을 증명한다고 확대하지 않는다.
6. Default/product FFI archive에서 새 scene marker, fixture payload, fixture-only ABI가 제외되고 `c04-runtime-gpu-fixture-test` positive control에서는 포함되는지 확인한다. Archive 결과를 APK/IPA 전체 배제 증거로 확대하지 않는다.
7. 구현 뒤에는 계획 검토와 다른 실패 관점 20개로 runtime/code를 검토하고, PR diff도 별도 실패 관점 30개로 검토한다. 발견 수정은 영향을 받는 trace, negative controls, simulator screenshot/link와 artifact gate를 다시 확인한다.
8. 근거가 모두 고정되면 내부 계약 0059, 상위 C11.4 계획, STATUS와 roadmap의 해당 제한 case를 필요한 만큼 동기화한다. 이 slice만으로 C11.4 checkbox나 공식 완료 수를 변경하지 않는다.

## 성공 및 미지원 범위

- Success: item overflow를 author하지 않은 single-PNG image scene에서 독립 규범 expectation과 Chrome CSSOM natural size/geometry가 재현되고, Spinon runtime의 fixed/intrinsic mixed tracks, minimum provenance, stage arrays, frame/revision이 JSON contract와 일치한다. Overflow clipping, scrolling, projection과 paint behavior는 별도 scope다. Android/iOS simulator screenshots가 PR 본문에 나타나며 Agent Browser link check가 통과하고, fixture-test opt-in 경계가 positive/negative artifact controls로 검증된다.
- 최초 candidate run의 runtime layout/frame은 `status=0`이었지만 기존 app matcher가 raw sentinel 슬롯과 finite batch growth를 기대하지 않아 양 플랫폼 candidate check가 실패했다. 원 로그를 보존했다. Stage derivation review PASS 후 corrected run에서 양 플랫폼의 trace/frame matcher, intrinsic source, draw/present 및 screenshot capture가 통과했다. `runtime_image_count=0,image_draw_count=0`이므로 PNG paint/readback은 주장하지 않으며 screenshots는 simulator UI evidence다. 구현 도중 trace가 독립적으로 유도한 기대와 다르면 기존 Taffy 결과를 기대값으로 역산하지 않는다.
- 제외: author CSS를 통한 일반 C11.4/overflow admission, direct text 또는 mixed text/image, min-content/max-content cross-case, row-axis intrinsic sizing, multi-span, nested/anonymous/out-of-flow items, percentage/calc/cyclic input, auto-repeat, another image format or loading behavior, overflow/border-pixel matrix, WPT completeness, public API, 전체 C11.4/C11 완료.

## 근거

- [Chrome 사전 비교와 독립 기대값](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/README.md)
- [규범 입력/수치 JSON](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/spec-derived-expectations.json)
- [Chrome input fixture](../tests/fixtures/css/c11/grid-image-auto-max-mixed-track-c114-precomparison.html)
- [상위 C11.4 계획](./c11-4-grid-track-sizing.md)
