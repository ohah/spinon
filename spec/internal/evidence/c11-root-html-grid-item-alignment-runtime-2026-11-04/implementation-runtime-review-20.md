# C11.1 parsed Root HTML Grid item alignment 구현/runtime 실패 관점 검토 20개

검토 대상은 현 후보의 cascade allowlist, typed projection, Taffy mapping, Root HTML runtime, simulator evidence다. 계획의 20개 검토와 계획 PR 변경 검토는 재사용하지 않았다. 아래 항목은 서로 다른 실패 조건을 확인했다.

후속 재검토에서 13번 경계에 빠진 negative case를 발견했다. non-Grid Root HTML의 non-initial `align-items`가 scope guard를 빠져나갈 수 있어 `align-items`도 `justify-items`처럼 거부하도록 수정하고, non-Grid root와 비적격 Grid 양쪽에서 두 property의 정확한 오류를 검사하도록 regression을 보강했다. 수정 뒤 관련 profile/parser/runtime 관점과 전체 Root HTML projection tests를 다시 확인했다.

1. **Profile 경계:** 새 admission은 `RuntimeRootHtmlTextPaintV1`에서 계산한 `root_html_grid_alignment_nodes`에서만 열린다. 다른 profiles에서는 해당 node set이 비어 있고 C11.4 track-sizing profile은 physical `left/right` 변환을 끈 별도 parser 호출을 유지한다.
2. **Grid shape 경계:** 기존 eligibility가 fixed non-negative definite size/tracks, sparse row flow, auto implicit tracks, no repeat/area/line names, zero edges, visible overflow, static LTR 조건을 확인한다. 새 기능은 이 validator를 넓히지 않았다.
3. **Direct item 소유권:** 대상은 empty direct HTML `div`, visible static fixed-size item, all-auto placement이며 node owner가 그 Grid와 일치해야 한다. Text, nested item, 다른 tag/namespace와 item capacity 초과는 기존 validator에서 거부된다.
4. **author CSS 경계:** Root HTML author allowlist에 item longhands, self longhands와 `place-items` shorthand가 추가됐고 computed property 목록에도 대응 longhand가 들어갔다. `align-items`·`justify-items`의 non-initial computed 값은 eligible Grid 외 Root HTML scope에서 각각 거부한다.
5. **Shorthand cascade:** `place-items:center`에서 Stylo가 산출한 computed `align-items`·`justify-items` 값이 layout에 적용된다. Root HTML style test와 39-scene runtime fixture의 두 `place-items` cases가 이를 검사한다.
6. **Block axis mapping:** Eligible container에서만 `align-items`가 runtime item-alignment parser를 거친다. `end`, `self-end`, safe/unsafe center/end, flex aliases의 positive expected frames를 Rust projection/full runtime tests에서 확인했다.
7. **Inline axis mapping:** `justify-items`는 별도 parser error property를 보존한다. Parser unit test가 `legacy` 오류의 node/property/value를 확인하고 physical value controls도 검사한다.
8. **Physical 좌우:** `left/right`를 inline start/end로 해석하는 branch는 Root HTML eligible `horizontal-tb`/LTR route에서만 활성화된다. Fixed 40px cell과 10px item의 x frame은 0px/30px다.
9. **Self position alias:** `self-start`·`self-end`는 item의 own writing-mode와 container axis에 의존한다. Eligibility가 container와 item 모두 `horizontal-tb`/LTR로 제한하고 runtime은 self-end frame을 확인한다.
10. **Overflow safety:** safe center overflow는 start로 되돌리고 unsafe center는 양·음 free space의 중앙에 둔다. Projection/runtime tests와 Chrome precomparison의 8개 overflow control이 안전성 및 geometry를 고정한다.
11. **Baseline rejection:** `align-items`와 `justify-items` baseline은 CSS computed value가 Root HTML allowlist에는 들어오지만 typed layout 전에 node/property/value error가 된다. Runtime negative control은 실패 layout이 partial scene을 publish하지 않는 것을 검사한다.
12. **값 allowlist:** `safe start`, `legacy`, `baseline`, 미등록 value는 item allowlist에서 허용되지 않는다. Allowlist unit test와 parser test가 out-of-scope value를 fail-closed로 유지한다.
13. **부적격 scope:** `1fr` track Grid와 non-Grid root에서 `align-items:center`와 `justify-items:center`를 각각 거부하며 diagnostic의 property가 입력과 일치한다.
14. **Self longhand 방어:** child `align-self:end`·`justify-self:start`는 계속 거부한다. Root HTML cascade가 두 computed longhand를 보존하고 style tests가 각 실제 property name을 검증한다.
15. **기존 content alignment:** module rename은 기존 `align-content`/`justify-content` allowlist와 Taffy values를 유지한다. Root HTML projection 회귀와 `c11_1_root_html_fixed_grid_content_alignment_reaches_default_runtime`가 통과했다.
16. **Track-sizing profile 회귀:** 공용 item-value parser extraction 뒤 C11.4 profile의 `justify-items` 변환은 start/physical branch 조건을 분리한다. Fixture-test track sizing focused regression은 14/14 통과했다.
17. **Grid container data flow:** `justify-items`는 적격 Root HTML Grid에서만 `GridContainerInput.justify_items`에 저장되고 기존 Taffy Grid mapping을 탄다. Center, end, safety, left/right frames로 실제 layout effect를 확인했다.
18. **Oracle 독립성 및 matrix completeness:** expected JSON의 39 scene·42 item은 독립 CSS 산식으로 만들어지고 Chrome 156 관찰과 39/39 비교됐다. Runtime test는 색상 identity로 각 item frame을 찾아 전체 42/42를 대조한다.
19. **앱 경로와 문서 좌표:** 같은 fixture를 기본 Root HTML V8 route로 읽어 Android/iOS 모두 `layout=ready boxes=83` 및 `presented boxes=83`을 냈다. Full item geometry는 Rust runtime frame test가 담당하고, 긴 페이지 screenshot은 보이는 첫 viewport만 증명한다.
20. **실행 재현성과 한계:** Android API 37 ARM64 AVD/SwiftShader와 iPhone 17 Pro/iOS 26.2 Simulator의 최신 source build/run 증거를 source·artifact hash에 연결했다. 최초 빈-script ESM eval 및 headless `monkey` launcher 실패를 보존했다. 앱은 explicit Activity launch로 확인했다. AVD와 전용 simulator를 종료하고 bootstrap source를 원래 SHA로 복구했다. PixelCopy, Metal drawable readback, physical device와 WPT는 미검증이다.

검토 후 남은 결함은 없다. Test logs와 app run logs는 [실행 README](README.md)에 연결되어 있다. 이 검토는 bounded candidate만 다루며 C11.1/C11 완료를 판정하지 않는다.
