# C11.4 nested Grid 양 축 PR diff 적대 검토 30개

PR 제출 diff를 독립 실패 경로와 계약 경계 30개로 검토했다. 각 관점은 최종 diff에서 재확인했으며 해결되지 않은 변경 결함은 찾지 못했다.

1. **기능 범위** — 새 동작은 네 고정 two-axis scene에만 추가된다.
2. **기존 scene 호환** — 이전 inline-only ID 네 개의 스타일과 기대값을 유지한다.
3. **ID 중복** — runtime scene table과 capture script가 unique ID를 검사한다.
4. **ID 정규화** — `-two-axis` 제거는 기존 네 expected-value mapper에서만 사용한다.
5. **FFI 문자열 크기** — 생성 scene source는 기존 adapter 제한 길이 안에 있다.
6. **깊이 encoding** — depth-one은 inner 직계 배치, depth-two는 middle을 한 번 추가한다.
7. **잘못된 depth** — 새 allowlist 외 depth와 ID는 runtime source lookup에서 거부된다.
8. **text content** — leaf text와 Ahem 선언이 비교 입력과 동일하다.
9. **style 일치** — width/height와 column/row constraint가 min 또는 max 한 쌍으로 맞는다.
10. **row sizing만 활성화되는 경우** — 두 축 scene 외에는 row intrinsic sizing이 켜지지 않는다.
11. **혼합 min/max** — `(min,max)` 및 `(max,min)` 조합은 전용 projection 승인 조건이 아니다.
12. **다른 Grid style** — 전문 adapter가 허용한 트리/스타일 검사를 통과한 scene만 새 함수를 호출한다.
13. **Feature gate** — 새 projection 함수와 호출 경로는 `c11-4-grid-track-sizing-fixture-test`에 묶인다.
14. **Projection option defaults** — 기존 helper 인자들은 모두 새 option을 false로 명시한다.
15. **Error atomicity** — intrinsic input validation 실패는 layout output 생성 전에 반환된다.
16. **Track pass lookup** — Rust test는 Columns와 Rows의 마지막 sizing pass contribution을 각각 확인한다.
17. **Contribution kind** — min/max expected kind가 두 축과 parent Grid마다 검사된다.
18. **Contribution span** — 각 고정 direct item은 span 1로 확인된다.
19. **Contribution magnitude** — width/height contribution value가 해당 축 기대 크기와 비교된다.
20. **Text leaf frames** — min/max 각 결과의 text frame과 모든 ancestor frame을 따로 검사한다.
21. **Fixture count** — FFI adapter는 기존 네 개를 포함해 정확히 여덟 scene을 내보낸다.
22. **Android allowlist** — 신규 ID를 앱 fixture 범위에 추가했고 화면 adapter의 box count 기대와 일치한다.
23. **iOS allowlist** — Android와 동일한 신규 ID 집합이며 Simulator 검증이 전부 통과했다.
24. **Capture scene matrix** — 캡처 도구의 scene 수는 188에서 192로 바뀌고 네 expected trace를 포함한다.
25. **Runtime trace verifier** — 새 scene만 Rows contribution을 추가 확인하며 기존 scene은 Columns 조건을 유지한다.
26. **Simulator readiness** — `layout=ready`, 전 노드 frame, 두 축 tracks, trace, `drawPresented=true`를 확인한다.
27. **Retry 경로** — 두 플랫폼 네 scene에서 superseded 재시도가 0회였고 부분 결과를 성공으로 기록하지 않는다.
28. **기본 artifact 경계** — 기본 `spinon-ffi` archive의 전체 string scan에서 새 projection 함수명과 네 scene ID가 0건이고 fixture-test archive에는 함수명과 네 ID가 모두 있다. archive hash와 검사 결과를 근거 파일에 남겼다.
29. **문서 상태** — 계획은 구현 중으로 갱신하되 공식 C11.4 check와 `72/213`은 유지한다.
30. **스크린샷 해석** — PR 이미지는 앱 화면 증거로 제시하고 layout/track 판정은 상세 JSON/log로 연결한다. 최종 합성 픽셀을 주장하지 않는다.

**검토 결과:** diff에서 수정이 필요한 코드 결함은 없었다. feature archive의 positive symbol 확인에는 Apple `nm`이 Rust 1.96 LLVM object 일부에서 attribute-version 오류를 출력했으므로, 기본/fixture archive의 전체 `strings` scan과 feature build 결과도 함께 기록했다. PR required checks와 merge state는 GitHub에서 별도로 확인한다.
