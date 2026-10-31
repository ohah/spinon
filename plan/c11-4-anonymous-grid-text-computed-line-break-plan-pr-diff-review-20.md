# C11.4 computed `line-break` 계획 PR 변경분 적대 검토 20개

**검토 범위:** `plan/c11-4-grid-track-sizing.md`, `plan/c11-4-anonymous-grid-text-computed-line-break.md`, 두 개의 계획 검토 문서, 그리고 [Chrome 사전 비교 근거와 이미지](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-precomparison-2026-11-01/README.md). 이는 계획 PR 후보 diff 검토이며 구현/runtime 검토가 아니다. PR 생성 뒤 diff가 이 검토 대상과 정확히 같은지 확인하고, 달라진 파일이 있으면 영향을 받는 관점을 다시 연다.

1. **C11.4 부모 문서 범위:** 신규 기록이 부모 계획의 Grid/text-sizing 범위 안에 있는가? **PASS** — 새로 추가된 section만 C11.4 anonymous direct-text sizing을 다룬다.
2. **Latest-first 순서:** #942 병합 이후 다음 계획 후보가 올바른 위치에 있는가? **PASS** — 계획 후보가 #942 기록 위에, #942가 #938 이하 이력 위에 놓였다.
3. **#942 병합 상태:** PR 번호, UTC 시간, merge SHA가 원본 상태와 같은가? **PASS** — `spec/STATUS.md`와 GitHub PR #942의 merged SHA `845f59ed378e7b3bbbabee3ed2eb0926b81f5359`와 일치한다.
4. **#942 범위:** word-break 결과에 locale별 한국어 line-break 지원을 덧붙였는가? **PASS** — `lang=ko`는 Parley metadata였고 locale별 줄분할을 보장하지 않았다는 원본 제한을 보존한다.
5. **#942 검증 표현:** 빈 PR check rollup을 CI pass라고 기록했는가? **PASS** — main required check 미설정과 미보고 check를 명시했다.
6. **오래된 tab-size 항목:** 이미 구현된 #893을 계속 “다음 후보”라고 두었는가? **PASS** — #892 plan과 #893 implementation을 병합 이력으로 바꾸고 실제 140 positive-scene 범위를 기록했다.
7. **tab-size의 evidence 경계:** whitespace-only controls를 positive scene으로 부풀렸는가? **PASS** — 14 controls를 양성 scene에서 제외한다고 명시한다.
8. **후보 status:** `line-break` 계획 초안을 지원 기능처럼 표시하는가? **PASS** — 계획·사전 비교 상태와 미완료 parent/72/213을 구분한다.
9. **Oracle 도구:** Chrome 비교가 사용자 선호인 Agent Browser에서 수행됐는가? **PASS** — evidence README와 Chrome image가 Agent Browser 실행을 기록한다; terminal-browser 증거를 사용하지 않는다.
10. **Oracle input 재현성:** browser 결과를 입력, 브라우저, 언어, 글꼴, 크기, Grid model과 연결했는가? **PASS** — README와 `chrome-observation.json`이 환경과 4개 장면 값을 고정한다.
11. **Font integrity:** 고정 font path/hash와 실제 weight-400 glyph advance가 맞는가? **PASS** — SHA와 variable-font HVAR instancing, cmap glyph, advance가 적혔고 저장소 hash와 일치한다.
12. **Image integrity:** PR에서 inline 이미지로 열 수 있는 브라우저 화면이 실제 존재하는가? **PASS** — runtime-style과 shrink-to-fit PNG가 각각 1280×633이며 README가 상대 경로로 연결한다.
13. **CSS Text 규범:** W3C §5.2 링크가 해당 속성 정의를 가리키는가? **PASS** — direct spec fragment를 사용하고 중국어/일본어 U+301C 조건을 계획에 인용한다.
14. **Intrinsic sizing 규범:** CSS Sizing와 Grid links가 track 기대를 뒷받침하는가? **PASS** — intrinsic min-content와 Grid track-sizing 절에 각각 연결한다.
15. **Scene arithmetic:** 각 expected column/row/root size가 10px glyph 및 line-height model과 맞는가? **PASS** — Japanese normal 10/30, strict와 English normal 20/20, max-content 30/10으로 일치한다.
16. **Computed-style source:** scene이 authored CSS 문자열, scene 이름 또는 오래된 snapshot에서 값을 추론하게 했는가? **PASS** — 같은 revision의 computed style 및 HostDocument owner를 확인하도록 계획한다.
17. **Fail-closed 범위:** unsupported values, locale, owner, text, revision이 fail-open되는가? **PASS** — `auto` baseline 유지와 새 scene exact allowlist, negative mutation matrix를 분리한다.
18. **Platform evidence:** 계획이 simulator 결과를 실제 기기·hardware GPU·compositor 증거로 부풀리는가? **PASS** — Android software AVD와 iOS pre-compositor Metal 범위를 따로 기록하도록 고정한다.
19. **Artifact isolation:** 새 fixture/font가 기본 product artifact에 섞일 수 있는가? **PASS** — 기본 archive와 fixture-test positive control을 분리해 검사한다.
20. **Status와 review 분리:** 계획 review, PR diff review, 구현 review, 공식 완료 수가 한 결과로 합쳐졌는가? **PASS** — plan의 서로 다른 20관점 검토와 향후 runtime review 및 C11.4/C11 promotion을 별도로 유지한다.

## 판정

20개 계획 PR 후보 diff 관점에서 수정이 필요한 결함을 찾지 못했다. PR을 연 뒤 final head diff 동일성을 다시 확인해야 하며, plan merge 뒤에도 구현 검증이 남는다.
