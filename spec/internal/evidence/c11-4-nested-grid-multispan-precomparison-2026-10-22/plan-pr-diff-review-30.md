# C11.4 nested Grid multi-span 계획 PR 변경 검토 30개

검토 대상은 계획·fixture·사전 oracle·`spec/STATUS.md`·내부 계약·C11.4 상세 계획 추적 변경이다. 이것은 계획 자체의 별도 20개 실패 관점 검토와 구분되는 PR 변경 검토다.

1. **작업 후보가 기존 fixture 중복인가** — direct multi-span은 root 단일 Grid, nested depth-three는 span 없는 leaf여서 교집합이 비어 있음을 코드에서 대조했다.
2. **C11.4를 완료로 표시하는가** — 공식 `- [ ] C11.4`는 유지했다.
3. **공식 완료 수가 변하는가** — STATUS·계획에서 `72/213`을 유지한다.
4. **미구현 상태를 숨기는가** — plan, STATUS와 0059가 구현 미시작으로 기록한다.
5. **공개 CSS/API 지원을 선언하는가** — 계획은 opt-in fixture만 대상으로 하고 지원 선언을 배제한다.
6. **내부 계약 숫자를 임의로 올리는가** — 숫자 버전 변경이 없다.
7. **범위가 재현 가능한가** — 네 Grid topology, child 종류, 텍스트, span, track과 크기가 고정됐다.
8. **scene 수가 문서마다 일치하는가** — 네 source-order case를 계획·fixture·evidence에서 대조했다.
9. **익명 item과 direct-leaf text를 혼동하는가** — 계획을 named HTML `div` direct-leaf text로 한정했다.
10. **row sizing 계산이 본래 목표를 오염하는가** — 60px 고정 height와 고정 20px row 세 개로 고정했다.
11. **Ahem font 경로가 잘못됐는가** — fixture 상대 경로가 `tests/fixtures/resources/c15/Ahem.ttf`를 가리키며 로드 성공을 관측했다.
12. **Ahem metrics의 근거가 없는가** — 50/70/180px 문자열별 min-content 값을 독립 기대 JSON에 남겼다.
13. **span-2 계산에서 overlap이 누락되는가** — 독립 track 결과 `[25,35,35,0]`을 문서와 JSON에서 교차 확인했다.
14. **span-3 계산이 fourth track을 포함하는가** — span은 첫 세 track뿐이며 fourth track은 0이다.
15. **분수 계산을 틀렸는가** — residual 85px을 균등 분배한 `[160/3,190/3,190/3]`을 다시 산술 검토했다.
16. **Chrome과 명세 기대값이 뒤섞이는가** — rational 기대값과 1/64 CSS px quantized browser 값이 별도 파일·열이다.
17. **browser scene 수치가 artifact와 안 맞는가** — 네 observation의 root·inner tracks와 item coordinates를 README 요약과 대조했다.
18. **Chrome 버전을 흐리게 쓰는가** — UA HeadlessChrome 156과 Agent Browser 0.31.1을 기록했다.
19. **환경 조건이 빠졌는가** — DPR·viewport·Ahem readiness를 JSON에 기록했다.
20. **screenshot이 다른 입력을 보여주는가** — 정적 fixture의 네 scene과 screenshot labels를 눈으로 대조했다.
21. **screenshot artifact hash가 누락됐는가** — SHA-256 manifest에 screenshot을 포함했다.
22. **입력 fixture hash가 누락됐는가** — manifest에서 precomparison HTML 경로를 기록한다.
23. **JSON 산출물 hash가 누락됐는가** — Chrome observation과 독립 기대 JSON을 모두 manifest에 포함했다.
24. **relative link가 틀렸는가** — plan→evidence, README→fixture, STATUS→plan 경로 깊이를 확인했다.
25. **C11.4 parent plan이 최신 후보를 못 가리키는가** — `c11-4-grid-track-sizing.md`에 날짜 순 후속 계획 링크를 추가했다.
26. **공식 STATUS가 진행 상태와 다르게 보이는가** — 새 항목을 C11.4 아래 계획 중으로 추가하고 C12 이전에 배치했다.
27. **내부 계약 문서가 새 runtime을 이미 보장한다고 오해되는가** — 0059에 계획/사전 oracle이며 runtime 구현은 아니라고 적었다.
28. **임의 실패를 partial layout으로 받아들이는가** — unknown scene·topology·style rejection을 계획 acceptance에 둔다.
29. **플랫폼 검증을 과장하는가** — Android API 37 AVD와 iOS Simulator만 계획하고 실기기·compositor/WPT 한계를 기록한다.
30. **계획 검토와 PR diff 검토를 한 번으로 세는가** — 별도 20개 plan review와 이 30개 PR diff review 파일로 분리했다.

**판정:** 계획 PR 범위의 사실·링크·상태 표기는 서로 맞는다. 구현 후에는 새로운 runtime 실패 관점 20개와 최종 구현 PR diff 30개를 별도로 수행해야 한다.

**후속 링크·빌드 재확인:** 첫 Rspress 실행에서 재현 HTML 상대 링크가 dead link로 잡혀 evidence README에서는 경로를 코드로 표시했고, STATUS·0059의 계획 링크는 GitHub main 문서 링크로 바꿨다. 다시 실행한 `mise exec -- bun run docs:build`에서는 이번 작업 파일의 dead link가 나오지 않았지만, tracked `spec/internal/evidence/c14-2-image-ratio-implementation-2026-10-14/README.md` 등 기존 evidence의 dead link 때문에 전체 build는 exit 1이다. 해당 타 파일은 이 PR 범위 밖이며 수정하지 않았다.
