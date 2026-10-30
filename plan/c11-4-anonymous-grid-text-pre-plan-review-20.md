# C11.4 anonymous Grid text `white-space:pre` 계획 실패 관점 검토 20개

1. **`pre`를 `pre-wrap`로 잘못 처리할 수 있음** — 계획은 computed `preserve + nowrap`을 별도 `WhiteSpaceMode::Pre`로 모델링한다.
2. **repeated spaces가 collapse될 수 있음** — `XX  XX`의 60px `pre`와 50px normal control을 고정했다.
3. **min-content를 word boundary에서 잘못 자를 수 있음** — `XX XX`의 `pre` min-content track 기대값은 50px, normal은 20px이다.
4. **soft wrap을 막지 못할 수 있음** — 25px root에서 `pre` line 50×10px 한 줄과 overflow를 비교한다.
5. **forced LF까지 억제할 수 있음** — `XX\nXX`는 `pre`에서도 두 줄, 20×20px이어야 한다.
6. **max-content와 min-content를 같다고 선언하면서 다른 입력을 오해할 수 있음** — no-forced-break 문자열의 무분할 의미와 forced-break control을 각각 명시했다.
7. **normal control이 실패해도 `pre` 결과만 보고 통과할 수 있음** — normal max/min/fixed-width scene을 같은 source와 별도 ID로 둔다.
8. **grid root sizing과 text line sizing을 혼동할 수 있음** — track, root border box와 text range line box를 독립 assertion 대상으로 둔다.
9. **공백 두 개가 조상/anonymous-item 경계에서 바뀔 수 있음** — 브라우저 및 runtime 모두 직접 text child를 둔 anonymous Grid item을 사용한다.
10. **CSS inheritance가 fixture에서 생략될 수 있음** — computed `white-space`가 Grid parent에서 anonymous item으로 상속되는 현재 경로를 고정하고 computed longhand를 확인한다.
11. **tab을 보존된 일반 문자처럼 잘못 렌더링할 수 있음** — tab-size 범위를 제외하고 tab 입력은 구체적 오류로 거부한다.
12. **CRLF 또는 form-feed가 두 줄로 중복 변환될 수 있음** — 이 slice의 scene은 LF 하나로 제한하며 공통 normalizer의 기존 segment-break 회귀는 별도 검사한다.
13. **source-to-paragraph provenance가 보존 공백에 손실될 수 있음** — text source ranges와 line mapping을 focused test에서 확인한다.
14. **wrap-mode만 바꾸고 collapse mode를 누락할 수 있음** — mapping은 `preserve + nowrap` 쌍을 모두 검사한다.
15. **`NoWrap`과 `Pre`를 합쳐 forced newline이 사라질 수 있음** — `NoWrap` 기존 collapse 동작 및 `Pre` forced-break scene을 분리 보장한다.
16. **기존 `PreLine`, `PreWrap`, `BreakSpaces` 동작을 회귀시킬 수 있음** — 기존 26 runtime scenes와 기존 text-layout tests를 전체 회귀 행렬에 포함한다.
17. **일반 CSS 입력으로 fixture scope가 넓어질 수 있음** — 고정 source/hash/style/scene allowlist 이외 입력은 거부하며 public author admission은 제외한다.
18. **Taffy 출력으로 기대값을 역산할 수 있음** — CSS Text/Grid 규칙과 산술 및 Agent Browser 관찰을 사전에 기록하고 별도로 비교한다.
19. **simulator 결과가 실기기나 final pixel 증거로 과장될 수 있음** — AVD software renderer와 iOS drawable/compositor 경계를 결과물에 적는다.
20. **좁은 slice를 C11.4 완료로 오인할 수 있음** — 계획과 상태 대장은 C11.4/C11 checkbox와 `72/213`을 유지한다.
