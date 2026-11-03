# PR #1124 변경분 적대 검토 30개

**대상:** [`feat/c11-root-html-grid-item-alignment`](https://github.com/ohah/spinon/pull/1124), 구현 commit `12319dc` 및 계획·계약·테스트·evidence diff. 계획 실패 관점 20개와 구현/runtime 실패 관점 20개를 재사용하지 않았다. 아래 검사는 서로 다른 failure path와 claim boundary를 대상으로 한다.

1. **Profile 진입:** 새 alignment parsing은 `RuntimeRootHtmlTextPaintV1`와 기존 eligible-node 집합에 한정되고 다른 profile은 기본 동작을 유지한다 — 통과.
2. **Grid shape:** definite fixed-px size/tracks/gaps, sparse row, auto implicit tracks, no repeat/area/named lines 조건은 기존 validator 그대로다 — 통과.
3. **Item ownership:** empty direct HTML `div`, static fixed-size, auto placement 및 explicit cell capacity 경계를 유지한다 — 통과.
4. **Author admission:** Root HTML author-property 목록에 item longhand와 `place-items`만 추가했고 general CSS admission이나 public API는 만들지 않았다 — 통과.
5. **Computed cascade:** `align-items`, `align-self`, `justify-items`, `justify-self` longhand를 실제 Stylo computed property 목록에서 가져온다 — 통과.
6. **Shorthand expansion:** `place-items:center`와 `place-items:end start`가 computed longhand 두 개를 거쳐 projection에 도달하며 fixture runtime이 두 case를 포함한다 — 통과.
7. **Non-Grid scope:** Root HTML block의 non-initial `align-items`와 `justify-items`가 각각 정확한 property error로 거부된다 — 통과.
8. **Ineligible Grid scope:** `1fr` column이 있는 Grid에서 두 item-alignment property를 각각 거부하고 정확한 diagnostic을 보존한다 — 통과.
9. **Content alignment scope:** 기존 non-eligible `align-content`·`justify-content` rejection과 eligible Grid 동작을 이름 변경 뒤에도 유지한다 — 통과.
10. **Self override:** Grid item의 non-initial `align-self`와 `justify-self`를 계속 거부하고 longhand 이름을 보존한다 — 통과.
11. **`align-items` parsing:** 공통 positional parser extraction이 기존 `allow_start` 설정과 C11.4 baseline value 처리를 유지한다 — 통과.
12. **`justify-items` diagnostics:** unsupported value의 error node/property/value가 `justify-items`를 보존한다 — 통과.
13. **C11.4 physical values:** C11.4 parser call은 physical left/right flag를 끄고 기존 track-sizing 회귀 14/14가 통과했다 — 통과.
14. **Physical left/right:** `left/right`는 `justify-items`에서만 허용되고 Root HTML eligible LTR `horizontal-tb` route에서 inline start/end가 된다 — 통과.
15. **Direction boundary:** container와 item의 writing-mode/direction 조건 밖에서 physical keyword나 self-axis semantics를 확장하지 않는다 — 통과.
16. **Flex aliases:** Grid에서 `flex-start/flex-end`의 start/end frame 결과를 독립 기대값과 비교한다 — 통과.
17. **Self-axis alignment:** `self-start/self-end`는 writing-mode 조건을 유지하고 item frame으로 대조한다 — 통과.
18. **Safe center:** positive free space와 overflow 상황에서 safe center의 start fallback을 각각 검사한다 — 통과.
19. **Unsafe center:** positive free space 중심과 overflow 음수 offset을 안전 fallback과 구분해 확인한다 — 통과.
20. **Safe/unsafe end:** 두 축과 overflow case에서 safe fallback 및 unsafe overflow offset을 별도 frame으로 대조한다 — 통과.
21. **Unsupported values:** `baseline`, `first/last baseline`, `legacy`, `safe start`는 typed layout 전에 fail-closed한다 — 통과.
22. **Taffy data path:** eligible Grid만 `GridContainerInput.justify_items`를 채우며 container default와 item sizing을 유지한다 — 통과.
23. **Normal/stretch behavior:** fixed-size item에서 `normal`·`stretch`의 existing sizing을 깨뜨리지 않는 expected frames가 있다 — 통과.
24. **Content + item composition:** combined scene은 `align-content`·`justify-content` track placement에 item center offsets를 합성해 확인한다 — 통과.
25. **Oracle independence:** Chrome 156 관찰 JSON과 별도 계산 expected JSON을 분리하고 39 scene 모두 대조한다 — 통과.
26. **Matrix cardinality:** 전체 runtime test가 39 scene·42 item을 확인하고 item별 유일 색으로 frame을 찾는다 — 통과.
27. **Frame assertion:** x/y/width/height 네 성분을 1/64 CSS px 허용 오차로 비교하고 sibling scene block offset을 포함한다 — 통과.
28. **Failure atomicity:** baseline rejection control은 failed layout/error 유지와 partial completed scene 부재를 검사한다 — 통과.
29. **Platform artifact identity:** 최종 Android APK와 iOS Simulator executable/root script hash가 metadata에 있고 최신 source build/run log와 연결된다. 두 앱은 `layout=ready boxes=83`과 draw/present를 기록한다 — 통과.
30. **Claim and evidence boundary:** screenshots는 first-viewport visual evidence로만 설명하고 PixelCopy/Metal readback·실기기·WPT·전체 C11 완료를 주장하지 않는다. STATUS와 PR 본문은 공식 `72/213`을 유지한다 — 통과.

검토 중 발견한 non-Grid `align-items` scope 누락은 fail-closed guard와 두 scope regression assertions로 수정한 뒤 관련 projection tests, runtime tests, Clippy, formatter 및 최신 Android/iOS build/run을 다시 확인했다. Agent Browser에서 PR 본문을 열어 두 raw screenshot이 모두 로드되고 natural dimensions가 Android 1080×2400, iOS 1206×2622임을 확인했다. 현재 PR status check rollup은 비어 있고 main required checks endpoint는 "Required status checks not enabled"를 반환했다. 이를 CI 성공으로 기록하지 않는다.
