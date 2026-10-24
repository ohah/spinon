# C11.4 · fixed-repeat line names와 auto-repeat 결합

**상태:** 구현·runtime 및 PR diff 검토를 마친 미병합 후보. [PR #600](https://github.com/ohah/spinon/pull/600)은 열려 있고, fixed/outer/auto line-name metadata와 named placement를 연결했다. GitHub CI/check 및 merge/post-merge main 검증은 남아 있다. C11 checkbox와 공식 `72/213`은 그대로다.

## 현재 동작과 결손

[PR #597](https://github.com/ohah/spinon/pull/597)은 한 track list에서 하나의 `auto-fill` 또는 `auto-fit`과 최대 10,000 authored descriptor 내의 임의 positive integer fixed-repeat group을 제한 지원한다. 각 fixed group은 computed finite nonnegative `LengthPx` track 1–2개다. 현재 `computed_auto_repeat_tracks`는 fixed-repeat이 있으면 `TrackList.line_names`, fixed-repeat 안의 `line_names`, auto-repeat 안의 `line_names`가 모두 비어 있지 않은 입력을 거부한다. 따라서 named line이 있는 repeat 조합은 fail-closed된다.

이 계획은 기존 breadth·count·descriptor 제한을 바꾸지 않고 해당 metadata를 보존하는 단일 slice다. C11.4 intrinsic track sizing 전체, 일반 CSS admission 또는 공개 지원 완료를 선언하지 않는다.

## 목표와 제외

- 한 축의 한 `TrackList`에서 하나의 `auto-fill` 또는 `auto-fit`과 authored 순서의 positive integer fixed-repeat group을 함께 허용한다. fixed group count와 pattern은 기존 #597 제한을 유지한다: 양의 정수 count, computed finite nonnegative `LengthPx` track 1–2개, 총 authored descriptor 10,000 이하.
- ordinary track의 바깥 line-name list, fixed-repeat pattern의 `N+1` line-name groups, auto-repeat의 `N+1` line-name groups를 cascade snapshot부터 layout adapter까지 보존한다.
- 동일 물리 line에서 만나는 인접 이름 목록을 authored 순서대로 합친다. 동일한 토큰이 합쳐진 목록에 두 번 있어도 named occurrence는 같은 물리 line을 두 번 세지 않는다. 문자열 목록 자체는 임의로 제거·재정렬하지 않는다.
- CSS line-based placement의 named line positive/negative occurrence, named span, auto-fill과 auto-fit을 columns·rows 각각에서 검증한다. 한 장면에서 양 축 repeat를 동시에 확장하지 않는다.
- runtime 검증은 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture boundary에만 추가한다. 제품 CSS admission 또는 공개 CSS 지원으로 확대하지 않는다.
- 두 번째 auto-repeat, nested/other repeat shape, invalid/zero count, pattern 2개 초과, percentage·unresolved `Calc`, flex/intrinsic/`minmax()` fixed-repeat breadth, template areas 결합, implicit named-line fallback, WPT 적합성, 실기기/hardware GPU, iOS compositor 이후 pixels와 전체 C11.4는 범위 밖이다.
- 내부 계약 0059의 숫자 버전 `0.1.0`, C11 공식 checkbox와 전체 `72/213` 집계는 바꾸지 않는다.

## 규범·독립 기대값·Chrome 사전 비교

- 규범은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1 repeat syntax](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax), [§7.2.2 named lines](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#common-uses-named-lines), [§8.3 line-based placement](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)다. auto-repeat은 한 번만 나오지만 fixed-repeat group과 함께 쓸 수 있고, 인접한 line-name lists는 하나의 line에 합쳐진다.
- [Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-4-fixed-repeat-line-name-merge-precomparison-2026-10-24/README.md)는 네 장면, 전체 observation JSON, 독립 sizing/geometry 기대값, screenshot과 재실행 validator를 보존한다.
- Agent Browser 0.31.1 / HeadlessChrome 156.0.0.0, macOS ARM64, viewport 1280×633, DPR 1이다. Chrome은 교차 관찰이며 규범이나 independent expectation을 대체하지 않는다.
- 세 fixed groups의 breadth 합은 84px, auto-repeat minimum은 24px, gap은 4px, definite axis는 300px다. `84 + 24n + 4(9+n) = 120 + 28n`이므로 6회는 288px, 7회는 316px이며 기대 반복 수는 6이다. 양 축 기대 track list는 `[12,8,12,8,6,6,6,6,24,24,24,24,24,24,10,10]`이다.
- fixture는 fixed-repeat 내부 반복 경계, 두 fixed group 사이, fixed/auto 경계, auto/fixed 경계에서 이름을 겹치게 한다. Chrome CSSOM은 해당 경계의 merged groups, 같은 physical line의 duplicate token, 양·음수 occurrence, span, auto-fit에서 named placement 후 collapse를 보인다. Independent item rectangles는 validator로 observation JSON과 대조한다.

## 데이터 경계와 구현 순서

1. **AST 대조:** Stylo `TrackList`에서 ordinary `list.line_names`, integer `TrackRepeat.line_names`, auto-repeat `line_names`가 각각 어느 source component 경계에 붙는지 fixture와 작은 cascade assertion으로 확정한다. 이 구조가 사전 비교 가정과 다르면 코드를 바꾸기 전에 plan과 oracle을 수정한다.
2. **Cascade metadata 생성:** positive fixed repeats의 `line_names.len() == track_sizes.len() + 1`을 확인한다. fixed repeats는 이미 expanded `tracks`로 나가고 layout converter도 non-auto tracks를 `Single` component로 내보내므로, 그 track 경계에 해당하는 fixed-repeat 이름을 authored order로 확장해 `ComputedGridTracks.line_names`에 놓는다. auto-repeat의 `N+1` inner groups는 `ComputedCssGridAutoRepeat.line_names`에 분리 보존한다. fixed repeat을 새 runtime component로 압축하거나 `GridContainerInput`에 새 field를 추가하지 않는다.
3. **Seam merge:** 한 physical line에 모이는 이름 group을 `list.line_names`와 repeat inner group 순서대로 이어 붙인다. fixed-repeat 각 iteration의 이전 끝과 다음 시작, component 바깥 이름, auto-repeat 시작·끝에서 기존 Taffy named-line resolver가 보는 component boundary 순서와 일치해야 한다. 동일 이름이 한 line에 반복돼도 source token을 조기에 deduplicate하지 않고, Taffy line map의 physical-line dedup이 occurrence를 한 번만 세는지 pinned vendored source와 assertion으로 확인한다.
4. **크기·할당 제한:** track count, expanded fixed line-name group 수, group/vector/name byte 합계를 만들기 전에 checked arithmetic으로 계산한다. computed line metadata는 기존 1 MiB cascade 제한을 넘지 않아야 한다. auto-repeat pattern은 기존 layout source 1 MiB·expanded 16 MiB 제한을 유지한다. Spinon-owned nested vector/string reservation은 fallible해야 하며 실패 시 node/property가 붙은 오류와 no-partial-output을 보장한다. vendored Taffy named resolver의 기존 infallible working-vector allocation은 이 보장에서 제외하고 process-wide OOM recovery를 주장하지 않는다. 기존 10,000 track limit와 overflow 경계를 바꾸지 않는다.
5. **Adapter 회귀:** `GridContainerInput.columns_line_names`/`rows_line_names`는 이미 auto-repeat Taffy component를 기준으로 경계 그룹을 받는다. 생성 vector 길이가 `non-auto expanded tracks + 2`인지 검사하고, named placement가 auto-repeat 있는 축에서 Taffy로 위임되는 기존 경로가 동일 group sequence를 읽는지 검증한다. layout API나 internal contract version 변경이 필요해지면 이 계획을 중단하고 contract 영향을 먼저 재검토한다.
6. **Cascade/layout 검사:** 기존 무명 single/multiple fixed-repeat·auto-repeat 회귀와 auto-repeat named-line 회귀, `grid-template`/`grid` shorthand cascade를 보존한다. named positive·negative occurrence, named span, repeated seam duplicates, fixed/fixed·fixed/auto·auto/fixed seam, rows/columns, auto-fill/auto-fit collapse, invalid count·second auto-repeat·unsupported breadth·areas 거부, 10,000/10,001 track, metadata budget exact/over, overflow·allocation failure·다음 호출 복구를 각각 검사한다.
7. **Runtime 및 artifact 경계:** opt-in fixture scene에 columns auto-fill, columns auto-fit, rows auto-fill을 추가한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 frame/track/trace/draw/present를 확인한다. screenshot 또는 pixel matrix는 capture source와 device/build hash에 연결하며 simulator 근거를 실기기·hardware GPU·iOS compositor 후 pixel로 확대하지 않는다. 기본 static archive에는 fixture marker/ABI가 없고 fixture-test positive control에는 들어가는지 확인한다.
8. **문서·PR:** 제한 slice만 계약 0059와 `spec/STATUS.md` C11.4 진행 기록에 추가한다. C11 checkbox·`72/213`은 유지한다. 구현/runtime 새 실패 관점 20개, PR diff 별도 30개를 기록하고 CI·실제 merge 상태를 확인한 뒤 Rebase and merge한다. merged docs와 roadmap Tailnet preview를 backup/hash/Agent Browser로 확인하고 브라우저 프로세스를 종료한다.

## 통과 기준

- Chrome 사전 비교 validator가 CSSOM track 순서, auto-repeat count, merged line group, unique physical-line occurrence와 expected rectangles를 통과한다.
- Cascade/adapter 결과는 Chrome과 named line placement·track order·auto-fit collapse에서 일치하고, geometry 오차는 0.5 CSS px 이하이며 exact pixel 주장은 캡처된 표본에만 한정한다.
- 모든 malformed/unsupported/limit 입력과 Spinon-owned allocation fault injection은 올바른 node/property 오류로 fail-closed하고 부분 Grid/style/layout/draw 결과를 내지 않는다. vendored Taffy 내부 infallible allocation에 대해 전역 OOM 복구를 주장하지 않는다.
- 기존 named line metadata가 없는 repeat profile의 결과와 artifact boundary를 보존한다. Android/iOS Simulator run을 모두 source revision·app hash에 묶는다.
- 이 slice 병합 후에도 C11은 미완료다. named line slice만으로 template areas, intrinsic track sizing, WPT, 전체 CSS Grid 적합성을 완료 처리하지 않는다.

## 계획 실패 관점 검토

계획 자체의 독립 20관점 검토는 [`c11-4-fixed-repeat-line-name-merge-plan-review-20.md`](./c11-4-fixed-repeat-line-name-merge-plan-review-20.md)에, 계획 PR 변경분의 별도 30관점 검토는 [PR diff review](../spec/internal/evidence/c11-4-fixed-repeat-line-name-merge-precomparison-2026-10-24/pr-diff-review-30.md)에 기록한다.

## 2026-10-24 구현·runtime 후보 결과

- Cascade fixed-repeat line groups를 authored order로 확장하고 ordinary/fixed/auto-repeat seam의 이름을 병합한다. Auto-repeat inner groups는 별도로 유지한다. Positive/negative named occurrence, named span, rows/columns와 auto-fit placement/collapse는 style-to-layout 4/4를 포함해 통과했다.
- `spinon-style` 16/16, layout 4/4, FFI scene 1/1, fixture boundary 11/11, formatter와 diff checks가 통과했다. 실행 로그·app hashes·화면은 [runtime evidence](../spec/internal/evidence/c11-4-fixed-repeat-line-name-merge-runtime-2026-10-24/README.md)에 둔다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 3/3 scene을 실행했다. PixelCopy 및 compositor 제출 전 Metal 표본은 각 12/12 exact다. AVD 결과는 실기기/hardware GPU 근거가 아니다.
- Release FFI archive 경계는 기본 marker 0/3·fixture ABI와 candidate Rust symbol 부재, opt-in positive control marker 3/3·두 symbol 존재다. [artifact evidence](../spec/internal/evidence/c11-4-fixed-repeat-line-name-merge-runtime-2026-10-24/artifact-boundary/README.md).
- 새 구현 실패 관점 20개는 [review](../spec/internal/evidence/c11-4-fixed-repeat-line-name-merge-runtime-2026-10-24/implementation-review-20.md)에 기록했다. 이 결과는 PR diff 30개 검토나 CI/merge를 대신하지 않는다.
- C11.3.3 template areas follow-up과 C11.4 전체 intrinsic track sizing, WPT/일반 CSS 지원은 계속 미완료다. 이 slice만으로 공식 완료 수를 올리지 않는다.
- PR #600의 별도 30관점 변경분 검토는 [review record](../spec/internal/evidence/c11-4-fixed-repeat-line-name-merge-runtime-2026-10-24/pr-diff-review-30.md)에 기록했다. PR checks는 아직 보고되지 않았다.
