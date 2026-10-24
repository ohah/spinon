# C11.4 fixed-auto-area 구현/runtime 실패 관점 20개

검토일: 2026-10-25

## 검토 범위와 실행 여부

최신 작업 트리의 [계획](../../../../plan/c11-4-fixed-auto-repeat-template-areas.md), [공식 상태](../../../STATUS.md), [내부 계약 0059](../../0059-c11-grid-layout.md)의 새 same-axis fixed-repeat + auto-repeat + areas 절, [runtime README](./README.md), cascade/projection/layout 코드와 테스트, Android/iOS fixture adapter, capture 및 저장된 runtime evidence를 대조했다. 계약 절은 plan·코드의 profile 범위, fixed-repeat 제한, shorthand 경계, metadata/expansion budget 및 남은 비지원 범위와 일치한다. 저장된 source capture 이후 앱 코드는 변경되지 않았고, 그 이후 추가된 관련 변경은 내부 계약 문서뿐이다. 계약 숫자 버전은 `0.1.0`이며, 이 제한 후보는 C11.4 완료로 승격되지 않는다. 별도 [PR diff 30개 리뷰](./pr-diff-review-30.md) 링크도 확인했다. CSS Grid 2025 CRD §7.4는 W3C 원문을 agent-browser로 확인한 뒤 브라우저를 종료했다. 이 리뷰에서는 테스트와 빌드를 실행하지 않았다. README와 저장된 실행 결과에는 Android API 37 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각 9/9 scene, area-larger 표면 표본 각 8/8 일치가 기록되어 있다. 아래 runtime pass 표시는 그 보관된 증거에 대한 확인이며, 이 리뷰에서 재실행했다는 뜻이 아니다.

## 실패 관점 20개

1. **[해결됨] Plan의 현재 결손 설명과 테스트 참조.** 재검토한 plan의 `이번 slice에서 해결한 결손과 남은 제한` 절은 기존 profile에서 fixed-repeat composition과 auto-repeat + areas를 별도로 지원한 상태, 이번 제한 후보가 같은 axis 조합을 허용한 상태, 교차축/두 번째 auto-repeat 등의 남은 거부 범위를 설명한다. 해당 단락은 실제 존재하는 `same_axis_fixed_auto_repeat_areas_preserve_tracks_lines_and_rectangles_on_both_axes` 및 shorthand parity test를 링크/언급하며 삭제된 테스트 이름은 남아 있지 않다. runtime README와 cascade guard의 제한 profile 설명에도 맞는다.

2. **[통과] 허용 profile 누출.** crates/spinon-style/src/stylo_dom/cascade/incremental/element.rs:177–202는 allow_auto_repeat을 RuntimeBlockFlexGridTrackSizingV1에서만 켠다. 같은 입력을 다른 profile에서 통과시키는 회귀가 생기면 fixture 전용 범위가 넓어진다.

3. **[통과] 교차축 조합 거부.** crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:120–139는 fixed-repeat과 auto-repeat이 서로 다른 축에 있을 때 계속 거부한다. cascade 테스트 fixed_repeat_mixture_is_limited_to_one_auto_repeat_axis가 양쪽 방향을 확인한다.

4. **[통과/실행 증거] cascade가 placeholder track 수와 area 차원을 혼동하는 경우.** fixed_repeat_auto_repeat.rs의 same-axis 테스트는 columns 7개 placeholder와 area 10열, rows 7개 placeholder와 area 10행을 별도 값으로 검사한다. 각 플랫폼의 9-scene 저장 결과에는 해당 track 및 rectangle trace가 있다.

5. **[통과/실행 증거] area가 확장 템플릿보다 큰 경우.** vendor/taffy/src/compute/grid/explicit_grid.rs:422–441은 area가 확장 template보다 큰 부분을 grid-auto-columns/rows 기반 implicit track으로 만든다. area-larger 기대값은 9개 확장 track 뒤 20px track을 추가하고 20px overflow를 만든다. 양 플랫폼 결과가 그 geometry와 표면 표본을 기록한다.

6. **[통과/실행 증거] area가 템플릿보다 짧은 경우.** columns-area-shorter는 8열 area가 10-track 템플릿을 줄이지 않고 tail을 x=158에 둔다. Android/iOS result.json의 observedNodeFrames와 tracks가 이 사례를 포함한다.

7. **[통과/실행 증거] 행축과 열축 전치.** cascade tests와 Android/iOS expectation은 row scene의 rectangles, frames, track trace를 column scene의 전치로 둔다. rows-area-larger도 두 플랫폼의 저장된 9-scene 실행에 포함된다.

8. **[통과/실행 증거] area-generated line과 authored line의 중복 및 occurrence.** Taffy vendor/taffy/src/compute/grid/types/named.rs:151–188, 392–455가 area aliases와 명시 이름을 병합한다. precomparison은 hero-start/end collision과 양·음 occurrence probe를 정의하며 fill-collision runtime frames가 양 플랫폼에서 일치한다.

9. **[통과/실행 증거] area가 있다는 이유만으로 auto-fit track이 점유되는 경우.** vendor/taffy/src/compute/grid/explicit_grid.rs:383–391은 실제 배치된 item 여부로 collapse를 판단한다. fit-empty는 0 크기, fit-occupied는 hero item이 span한 track 유지로 구분되고 양 플랫폼 trace가 기록돼 있다.

10. **[통과/실행 증거] auto-fit 접힘 구간의 gutter 경계.** 같은 Taffy 코드의 track과 다음 gutter 접힘, run 시작 gutter 유지 규칙은 track 크기만으로 놓치기 쉽다. 이 fixture는 collapsed flags와 item frames 모두 비교한다. 전체 surface pixel 비교까지 한 것은 아니다.

11. **[통과/범위 명시] 유효한 track-list shorthand와 area longhand 조합.** cascade 테스트 same_axis_fixed_auto_repeat_areas_match_grid_template_and_grid_shorthands는 grid-template/grid의 rows/columns track-list branch 뒤에 grid-template-areas longhand를 두고 typed metadata를 longhand 기준과 비교한다. grid shorthand가 implicit grid property를 초기값으로 reset한 뒤 다음 선언이 이를 다시 설정하는 cascade 순서도 유효한 범위다. 다만 grid-auto-columns/rows reset 조합의 별도 fixture는 없다.

12. **[§7.4 제외 정당] area-string shorthand 안의 repeat().** [CSS Grid 2025 CRD §7.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#explicit-grid-shorthand-the-grid-template-property)는 area 문자열 shorthand branch의 track listings에서 repeat()를 허용하지 않는다고 명시한다. 따라서 이 C11.4 후보에서 area-string branch의 repeat() 조합을 제외하는 계획과 README의 설명은 정확하다. 예를 들어 grid-template: "a" 20px / repeat(auto-fill, 20px)는 이 후보의 유효 입력이 아니다. 별도 parser negative fixture는 없으므로, 추후 parser admission을 넓힐 때는 adapter에 도달하지 않는지 확인할 수 있다.

13. **[통과/한계 명시] immutable document snapshot에서 area→none→area.** crates/spinon-style/src/stylo_dom/document.rs:47–50은 StyloDocumentView가 snapshot과 root에 고정된다고 설명한다. cascade 테스트는 세 문서 snapshot에 각각 새 view를 만들어 동일 node의 metadata 제거·복원을 비교한다. 이 설명은 README의 제한과 맞는다.

14. **[명시된 미검증 범위] 동일 view의 cached incremental grid restyle.** README는 같은 StyloDocumentView 내부의 cached incremental grid restyle을 이 profile에서 검증하지 않았다고 적는다. 테스트가 새 view를 사용하는 사실과 일치하며 지원을 과장하지 않는다. 동일 view 재계산이 제품 계약이면 별도 검증이 필요하지만 현재 후보의 pass 주장에는 포함되지 않는다.

15. **[통과] template-area 차원과 rectangle 범위.** grid_values.rs:478–555는 0 또는 GRID_MAX_TRACKS 초과 dimension, start/end가 역전되거나 같은 rectangle, area 범위 밖 끝값과 u16 변환 실패를 거부한다. 반례는 0×N, 10,001-track 축, column_end가 area.columns보다 큰 입력이다.

16. **[통과/경계] auto-repeat line-name group N+1 개수.** grid_values.rs:632–677은 반복 조각 범위와 fixed component 수에 따른 outer/repeated group 개수를 확인한다. group 수를 N 또는 N+2로 만든 synthetic ComputedCssGridLayout이 거부되어야 한다. 이 특정 malformed vector용 새 테스트는 이번 diff에서 확인하지 않았다.

17. **[통과] projected combined metadata 1 MiB 경계.** grid_values.rs:101–200은 line-name wrappers/payload와 area headers/names를 checked arithmetic으로 합산해 projection allocation 전에 검사한다. combined_metadata_tests.rs에는 exact/1 byte over와 양축 auto-repeat exact/over 사례가 있다. 주석과 README는 논리 wrapper/payload 크기이며 allocator 내부 비용은 제외한다고 정확히 한정한다.

18. **[통과] projection reservation 오류 문맥 및 재호출 복구.** combined_metadata_tests.rs는 Groups/Names/NameText/Areas/AreaName 다섯 failpoint가 CSS property가 붙은 GridAllocationFailure를 내고 다음 projection이 성공하는지 검사한다. 이는 주입된 예약 지점 검증이지 실제 시스템 전체 OOM 재현은 아니다.

19. **[통과/한계 명시] Taffy auto-repeat 이름 확장 상한.** crates/spinon-layout/src/grid.rs:897–994는 두 축의 line-name source byte를 1 MiB, 최대 track 수 기준 expanded estimate를 16 MiB로 제한한다. crates/spinon-layout/src/grid_tests.rs의 auto_repeat_expanded_line_metadata_above_budget_fails_before_taffy는 16 MiB 초과 추정치가 거부되는 반례다. compute.rs:54–61에서 전체 validate가 compute_tree보다 먼저 실행된다. 문서는 이를 estimate라 부르고 process peak/allocator rounding 보장으로 확대하지 않아 과장이 없다.

20. **[runtime 통과 증거/제품 산출물 한계] fixture route, pixel capture, artifact 경계.** Android parser는 정확히 Columns→Rows 두 pass를 요구하며 iOS parser와 맞는다. canonical HTML palette와 JSON/native sample 색은 정렬됐고 FFI CSS assertion은 scene/item selector에 placement를 묶어 확인한다. 저장 결과는 Android/iOS 9/9 draw/present, area-larger의 root/overflow/clear 표본 각 8/8 일치를 기록한다. fixture/product FFI static archive는 product marker 0/9·fixture symbol 0개, positive control marker 9/9·symbol 2개로 구분된다. README가 명시하듯 최종 Android APK/iOS app bundle 전체의 fixture code/data 제외와 Chrome 대비 전체 화면 색상 parity는 검증되지 않았다.

## 결론

§7.4의 area-string shorthand 제외는 규범 문구에 맞으므로 구현 결손이나 shorthand parity blocker로 보지 않는다. 새 0059 절은 same-axis 제한, 별도 fixed-repeat group 개수 상한 없음, authored component/expanded descriptor 각각 10,000 한도와 CSS parse/admission/projection/layout 오류 경계를 현재 코드와 맞게 기록한다. expansion estimate, projection budget, immutable snapshot 제한도 현재 README가 코드와 맞게 한정한다. 이전 1번의 stale plan 문구 및 삭제된 테스트 참조는 수정됐고 현재 계획·STATUS·runtime README·0059 계약의 C11.4 상태, 완료 수와 링크가 서로 맞는다. 항목 2–20은 기존 결론을 유지할 수 있다. 내부 계약 숫자 버전은 `0.1.0`이다. C11.4 checkbox는 미완료이고 공식 완료 수 `72/213`은 유지된다. 최종 Android APK/iOS app bundle 전체 fixture exclusion은 미검증이다. 이 문서는 작업 단위 구현/runtime 20개 리뷰이며 별도 PR diff 30개 리뷰를 대체하지 않는다. 이번 재검토에서는 테스트와 빌드를 실행하지 않았다.
