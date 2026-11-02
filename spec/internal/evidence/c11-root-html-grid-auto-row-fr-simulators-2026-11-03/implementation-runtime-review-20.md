# C11.1 direct fractional implicit-row implementation/runtime 검토

검토 대상은 현재 후보 코드와 실행 근거다. 계획 PR의 20개 검토 및 plan PR diff 검토는 재사용하지 않았다. 각 항목에서 별도 실패 조건을 확인했다.

1. **Style profile 누수:** direct fraction row list opt-in은 `RuntimeRootHtmlTextPaintV1`에서만 켜진다. Cascade test `root_html_profile_preserves_fractional_and_mixed_auto_row_lists`가 기본 parsed Root HTML 경로를 확인한다. 다른 profile option 기본값과 호출부는 바뀌지 않았다.
2. **CSS cascade 대 computed projection 경계:** `-1fr` syntax-invalid auto-row declaration은 cascade fallback을 유지한다. `root_html_profile_falls_back_after_syntax_invalid_negative_fractional_auto_row`가 이를 고정한다.
3. **Typed projection:** Root HTML no-area projection은 direct px/%/fraction 값과 authored 순서를 유지한다. `root_html_profile_projects_mixed_px_percentage_fractional_implicit_row_track_lists`가 px/%/fr 목록을 확인한다.
4. **비 direct track:** intrinsic, `minmax()` 등 목록 요소는 새 fr 허용으로 projection admission에 들어오지 않는다. `root_html_profile_rejects_empty_invalid_or_unsupported_implicit_row_lists`와 기존 intrinsic-track 테스트를 확인했다.
5. **NaN/음수/무한대 factor:** layout 경계에서 모두 `InvalidGridTrack(grid-auto-rows)`로 실패한다. `root_html_fractional_auto_rows_reject_invalid_factors`가 각 유형을 확인한다.
6. **이전 px scalar 계약:** `GridAutoRows::LengthPx` 단일 값 표현은 바뀌지 않았고, fraction list 변환만 추가했다. 기존 fixed-px implicit-row test와 runtime regression을 다시 실행했다.
7. **빈 목록과 목록 cap:** non-empty direct list 및 `GRID_MAX_TRACKS` admission을 유지한다. Projection empty/oversized tests와 layout expanded-row ceiling tests를 확인했다.
8. **확장 전 자원 한도:** placement는 capped row 수를 계산한 뒤 exact reservation을 수행하고 expanded rows를 한도 내에서 만든다. 기존 reservation-failure recovery test와 52개 auto-placement regression이 통과했다.
9. **Explicit row 시작점:** auto-list 인덱스는 explicit row 뒤 첫 implicit row에서 시작한다. Runtime `1fr 2fr` fixture의 explicit 20px row 이후 item starts `[25, 40, 65, 80]`을 확인했다.
10. **목록 순환과 배치 순서:** 반복 factor 패턴이 각 implicit row에 적용되고 source-order placement가 유지된다. Layout exact frames 및 runtime `cycles_fractional_implicit_rows`가 확인한다.
11. **Percentage basis:** percentage는 definite root content height로 계산되며 row-gap은 basis에서 빼지 않는다. Chrome 사전 비교의 mixed/overflow scenes 및 runtime mixed-percent/fraction composition 결과를 고정했다.
12. **Mixed fixed/%/fr free space:** fixed·percentage sizing 및 gutter를 제외한 남은 definite block space를 flex factor로 나눈다. Chrome `mixed` expected rows `[20,15,30,20,15]`와 layout/runtime exact frame tests를 대조했다.
13. **100% 초과와 visible overflow:** `120%` implicit row는 clamp하지 않고 이후 fr track은 남은 공간이 없을 때 0이 된다. Chrome overflow oracle와 runtime item starts `[25,150,165,290]`이 일치한다.
14. **Definite zero axis:** fixed zero-height Root HTML Grid도 유효하며 fractional rows가 0으로 계산된다. Layout `root_html_fractional_implicit_row_resolves_zero_height_to_zero`를 확인했다.
15. **`0fr` automatic minimum:** zero factor track은 automatic minimum을 유지하고 뒤 flex track이 남은 share를 받는다. `root_html_zero_fractional_track_keeps_automatic_minimum_and_remaining_fr_share`를 확인했다.
16. **Subunit factor:** total flex factor가 1보다 작은 경우 남는 공간을 억지로 채우지 않는다. `root_html_subunit_fractional_implicit_rows_preserve_trailing_space`의 17.5px rows와 trailing space를 확인했다.
17. **Item minimum과 overflow:** fixed-size child의 automatic minimum이 fr track을 키울 수 있고 root 경계를 넘어도 clamp되지 않는다. `root_html_fractional_implicit_row_preserves_fixed_item_minimum_overflow`와 Chrome `minimum` scene을 비교했다.
18. **Definite height 전제:** indefinite height, min/max override 및 aspect-ratio를 RootHtml implicit-row validator와 expanded-row validator에서 거부한다. Layout·runtime rejection matrices에서 partial scene이 나오지 않는다.
19. **Placement/profile 경계:** named-area, authored placement, dense/column flow, implicit columns, nonzero order와 unsupported item shapes는 기존 실패 경계를 유지한다. Projection tests와 runtime `rejects_unsupported_implicit_row_combinations_atomically`를 확인했다.
20. **Revision·app route·증거 한계:** runtime helper가 completed layout/cascade revision key를 비교하고 13-box scene을 확인한다. Android API 37 AVD와 iOS 26.2 Simulator 로그는 같은 fixture SHA의 `layout=ready`와 draw/present를 보인다. 화면 캡처는 visual smoke이며 PixelCopy/Metal readback 또는 실기기 검증이 아니다.

범위 외 넓은 style-to-layout Root HTML 검사는 36/37이다. 유일한 실패는 변경하지 않은 `grid-auto-columns: -1fr` fallback assertion이다. 이 실패를 green 근거로 바꾸거나 C11 completion으로 확대하지 않았다.
