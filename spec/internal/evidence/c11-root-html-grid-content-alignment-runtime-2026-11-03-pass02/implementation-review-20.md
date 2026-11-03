# C11 Root HTML Grid 콘텐츠 정렬 구현·실행 실패 관점 검토 20개

계획 검토와 별도로, 구현과 실제 실행 근거를 다시 대조했다. 각 항목은 서로 다른 실패 경계다.

1. **일반 CSS profile 누수:** 새 값은 `RuntimeRootHtmlTextPaintV1`의 eligible Grid에서만 projection에 연결하고 다른 profile은 기본 거부 경로로 남겼다.
2. **비 Grid element 오허용:** Root HTML에서 non-initial `align-content`/`justify-content`를 쓰는 non-eligible element의 거부 테스트를 추가했다.
3. **부적격 Grid 오허용:** profile predicate가 거짓이면 기존 `reject_grid_alignment`에서 해당 선언을 거부한다.
4. **indefinite container 크기:** 두 축을 finite non-negative `LengthPx`로 제한한다.
5. **intrinsic/minmax track 오해:** track 목록의 모든 항목이 finite non-negative fixed px일 때만 통과한다.
6. **empty explicit track 목록:** 양 축 track이 비어 있으면 eligible set에서 빠진다.
7. **implicit track 누락:** item 수가 rows×columns보다 많으면 허용하지 않는다.
8. **named line/repeat/area 우회:** auto-repeat, fixed-repeat group, area 및 line-name metadata를 각각 검사한다.
9. **dense/column auto-flow 혼입:** sparse row flow만 허용한다.
10. **명시 item placement:** 네 line/span computed placement가 모두 Auto인지 확인하고 raw fallback 값도 확인한다.
11. **text/replaced/nested item 오허용:** 직접 item은 HTML namespace의 빈 `div`만 허용한다. whitespace text만 무시한다.
12. **외부 child/다른 namespace:** 허용한 child 구조 외 node kind는 predicate를 실패시킨다.
13. **item의 intrinsic/minmax 크기:** item 크기도 고정 px이며 min/max/aspect-ratio 제약을 금지한다.
14. **item edge·overflow:** margin/padding/border가 0이고 overflow visible인지 검사한다.
15. **writing mode/direction:** container와 item 모두 `horizontal-tb`·LTR이어야 한다.
16. **음수·비유한 수치:** 크기·track·gap helper가 NaN, infinity와 음수를 거부한다.
17. **baseline 축약:** first/last baseline은 allowlist에서 빠지고 별도 negative test가 있다.
18. **safe/unsafe 의미 혼선:** 양 축 negative free space에서 safe fallback과 unsafe overflow를 layout/projection 테스트로 확인한다.
19. **gap 분배 오류:** positive case는 기존 10px gap을 포함한 독립 산식과 Chrome 기대 frame을 1/64 CSS px tolerance로 확인한다.
20. **partial frame·플랫폼 과장:** 기본 runtime은 `layout=ready`, 7 boxes로 완료됐고, 두 화면은 visual smoke로만 기록했다. surface pixel equality나 WPT 결과로 승격하지 않았다.

검토 중 수정 사항은 없었다. 구현 단계에서 발견한 초기 shorthand cascade 오류와 AVD crash-consent 대기는 각각 fixture 수정과 emulator 실행 옵션으로 해결했고 원본 실패 자료를 보존했다.
