# C11.4 cyclic percentage `fit-content()` PR diff 실패 관점 검토 30개

PR [#474](https://github.com/ohah/spinon/pull/474)의 main 대비 최종 diff를 implementation review와 별도로 살폈다. 범위는 Rust fixture/projection, Android/iOS validator, capture runner, plan/status/roadmap, 실행 근거, PR 본문 screenshot 링크다.

1. **Rust module 경계:** 새 scene adapter가 product runtime 경로에 직접 섞이는가? C11.4 fixture module 아래에만 두고 opt-in fixture-test dispatch에서 연결했다. PASS.
2. **fixture source 중복:** 이전 위치에 orphan copy가 남아 두 구현이 갈라지는가? canonical `runtime_gpu/fixtures/c11_4/` 경로만 `#[path]`로 참조한다. PASS.
3. **고정 source 변이:** scene ID 외 입력으로 arbitrary HTML이 만들어지는가? adapter는 고정 세 tuple만 lookup하고 미등록 ID는 오류다. PASS.
4. **CSS 지원 승격:** PR이 일반 author CSS support를 열거나 주장하는가? public API/admission은 변경하지 않고 fixed runtime source만 추가했다. PASS.
5. **fixture matrix 오염:** 신규 ID가 기존 211개 runner scene count 및 unique guard를 깨뜨리는가? inventory는 211이고 three IDs가 unique하다. PASS.
6. **scene run-group 충돌:** 세 장면이 기존 cyclic-percent `run-kind`에 오분류되는가? 요청 ID allowlist를 확장했고 명시 `cyclic-fit-content` kind/evidence group으로만 capture했다. PASS.
7. **column oracle 혼합:** cyclic column 25px 기대가 이전 `minmax` 또는 12px percentage baseline에서 유래하는가? CSS Grid 2025 CRD/Ahem 독립 계산과 Chrome 156 관측을 분리했다. PASS.
8. **row oracle 혼합:** row height 40px root와 20px final row가 같은 값을 뜻한다고 혼동하는가? trace, root frame, item frame을 각각 기대값으로 둔다. PASS.
9. **definite control 누락:** cyclic cases만 있어 definite percentage basis 경로를 비교할 수 없는가? width 40px control은 Columns `[20]`과 root/item frame을 별도로 고정한다. PASS.
10. **typed value erasure:** computed `fit-content(50%)` projection이 percentage 대신 px/calc로 바뀌는가? typed `FitContent(Percentage(0.5))` regression이 있다. PASS.
11. **scene topology mismatch:** Chrome/Rust/native tree가 root/item/text 수나 순서에서 달라지는가? root와 item ID 및 ordered two-node frame을 검사한다. PASS.
12. **Ahem fallback:** 폰트 누락 시 system font metric으로 test가 조용히 바뀌는가? fixture runtime이 고정 Ahem font bytes를 등록하고 사용한다. PASS.
13. **기존 text fixture 회귀:** `TextCase` 구조 확장으로 기존 case의 축·크기 기본값이 달라지는가? 기존 열 개 항목은 값을 명시하고 신규 세 항목과 함께 13개 고유 case matrix에 유지한다. PASS.
14. **normal alignment 예외 확산:** 새 computed `normal` 처리로 기존 `start` cases가 느슨해지는가? 예외 predicate는 `FitContentPercentage` track에만 적용한다. PASS.
15. **dimension category broadening:** Auto 또는 MaxContent를 예상하는 case가 arbitrary length를 받아들이는가? `dimension_matches`는 세 expected variants와 같은 category일 때만 성공한다. PASS.
16. **min-height auto minimum:** `min-height:0` source와 computed style assertion이 서로 불일치하는가? fixed cases에 `min_height=0`을 넣어 정확히 비교한다. PASS.
17. **track list under-validation:** column scene의 row 또는 row scene의 column 오류를 놓치는가? runtime adapter, Rust test, runner는 두 축의 single explicit track을 모두 검사한다. PASS.
18. **initial sizing pass leak:** 첫 provisional trace를 final track으로 오인하는가? direct layout helper는 축별 마지막 matching pass를 고르고 final stage base size를 비교한다. PASS.
19. **partial frame acceptance:** expected track만 맞고 box frame은 달라도 통과하는가? Rust와 두 host는 root/item frame 및 node order를 각각 assert한다. PASS.
20. **box count drift:** 추가 anonymous box가 생겨도 scene이 성공하는가? Android expectation·iOS expectation·runner가 2개 box를 고정한다. PASS.
21. **Android argument path:** Android C11.4 scene allowlist가 신규 ID를 받아들이지 않거나 다른 case로 dispatch하는가? 세 ID가 allowlist와 root/item expectation에 들어갔고 AVD 3/3 launch가 통과했다. PASS.
22. **Android sample target:** PixelCopy request가 잘못된 node ID/좌표를 검사하는가? scene별 inside/outside 좌표와 item node ID를 정확히 고정한다. PASS.
23. **Android color default:** 새 sample ID RGB가 누락돼 fallback 색이 정답이 되는가? 안쪽 red·바깥 background의 여섯 기대 RGB를 명시했고 PixelCopy 6/6 exact다. PASS.
24. **iOS request identity:** Metal request의 scene/sample IDs가 PixelCopy IDs와 섞이거나 기본 sample set으로 허용되는가? `hasFixedC114TrackSizingSamples`의 세 tuple을 각 scene별로 추가했다. PASS.
25. **iOS sample boundary:** UIKit screenshot을 Metal drawable readback으로 오인하는가? 실행 evidence는 drawable 값을 보고하며 compositor 제출 전임을 명시한다. PASS.
26. **draw completion:** runtime layout-ready만으로 화면 present까지 통과시키는가? 결과 JSON은 각 scene `drawPresented=true`와 retry 0을 확인한다. PASS.
27. **Android binary identity:** AVD 설치본이 local fixture APK와 달라질 수 있는가? local/installed SHA-256이 동일하다. PASS.
28. **simulator source binding:** untracked module이 metadata dirty path에서 빠질 수 있는가? 최종 capture는 clean commit `9af5ee8f`/tree `3f21957d`를 가리키며 빈 diff와 dirty-path 목록을 기록한다. PASS.
29. **artifact claim expansion:** static `.rlib` scan을 APK/IPA release exclusion으로 과장하는가? evidence는 기본/positive `.rlib` 두 hash를 구분하고 앱 release artifact 검증이 아니라고 제한한다. PASS.
30. **roadmap completion drift:** 작은 fixture slice가 C11.4 완료 수나 공개 지원 범위를 올리는가? `spec/STATUS.md` checkbox와 `72/213`을 유지하고 roadmap/plan에 미완료 범위 및 실행 링크를 둔다. PASS.

PR 본문 Android/iOS raw image 링크는 Agent Browser `0.31.1`에서 실제로 렌더링됐다. 각각 `naturalWidth` 1080/1206이며 전체 PR 화면과 판정 JSON을 같은 evidence directory에 보존했다. 최종 PR diff 기준 판정은 PASS다. GitHub required-check와 merge state는 이 문서 검토와 별도로 merge 직전에 live 조회한다.
