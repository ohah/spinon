# C11.3.3 fit-content cap roadmap sync PR diff 실패 관점 검토 30개

검토 대상은 `spec/public/roadmap.html`의 C11.3.3 area-backed typed `grid-auto-*` 카드에 계획·Chrome 비교 링크를 추가한 변경과 그 경로를 선택 빌드·Tailnet에 전달하는 보조 스크립트다. 각 항목은 서로 다른 내용·경로·게시 실패를 확인한다.

1. **카드 식별** — 새 링크가 PR #911을 설명하는 C11.3.3 카드에만 들어갔다. 통과: Agent Browser에서 대상 카드 텍스트와 세 링크를 확인했다.
2. **변경 범위** — public roadmap diff가 대상 카드에 한정된다. 통과: 변경 라인은 카드 한 줄이다.
3. **기존 설명** — PR #911 동작 설명과 실행 근거 문장이 보존된다. 통과: diff는 두 링크만 추가한다.
4. **cap 한계** — `<code>fit-content()</code> cap`이 미검증이라는 기존 경계를 유지한다. 통과: 카드에 그대로 남아 있다.
5. **공식 집계** — C11 미완료와 `72/213`이 유지된다. 통과: public 카드와 공식 상태 대장 모두 변경하지 않았다.
6. **계획 route** — `/spinon/plan/c11-3-3-template-area-fit-content-cap.html`이 계획 문서로 이어진다. 통과: 선택 RSPress 출력에서 생성되고 Agent Browser로 열었다.
7. **비교 근거 route** — `/spinon/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/README.html`이 사전 비교 문서로 이어진다. 통과: 선택 출력에서 생성되고 Agent Browser로 열었다.
8. **링크 이름** — 계획 링크는 cap 후속 계획임을 나타낸다. 통과: `fit-content cap 후속 계획`이다.
9. **oracle 표시** — 비교 링크는 Chrome 156 사전 비교임을 나타낸다. 통과: 링크 이름과 목적지 모두 Chrome oracle 페이지다.
10. **oracle 경계** — Chrome 관찰을 Spinon 구현 결과로 오해할 표현이 없다. 통과: 페이지와 계획이 Chrome-only라고 명시한다.
11. **Android 링크 회귀** — 기존 Android AVD 화면 링크가 유지된다. 통과: 카드 diff 밖이고 렌더된 카드 링크 목록에 남아 있다.
12. **iOS 링크 회귀** — 기존 iOS Simulator 화면 링크가 유지된다. 통과: 카드 diff 밖이고 렌더된 카드 링크 목록에 남아 있다.
13. **계약 링크 회귀** — 내부 계약 0059 링크가 유지된다. 통과: 기존 링크와 route가 보존된다.
14. **기존 로드맵 검토 링크** — PR #911의 로드맵 diff 검토 링크가 유지된다. 통과: 기존 경로가 보존된다.
15. **공식 상태 anchor** — `/spinon/STATUS.html#css-구현-체크리스트`가 유지된다. 통과: 기존 href를 보존했다.
16. **base prefix** — 추가한 두 href가 `/spinon/` 아래에 있다. 통과: Agent Browser의 mobile JSON에서 두 정확한 경로를 기록했다.
17. **HTML escaping** — inline `<code>` 표기가 태그를 깨뜨리지 않는다. 통과: 기존 escaped markup은 변경하지 않았고 카드가 정상 렌더됐다.
18. **href와 anchor 구조** — 속성 따옴표와 anchor 종료가 유효하다. 통과: HTML을 렌더하고 두 링크를 DOM에서 읽었다.
19. **paragraph 경계** — 새 링크가 기존 `<p>` 카드 안에 있다. 통과: 선택 출처 검사에서 같은 paragraph 경계를 확인했다.
20. **separator** — 링크 사이의 가운데점 구분자가 유지된다. 통과: 렌더된 링크가 기존 목록의 순서와 구분자로 이어진다.
21. **중복 route** — plan/evidence 링크가 대상 카드에서 각각 한 번만 나온다. 통과: DOM 링크 목록과 원본 카드 범위를 대조했다.
22. **다른 카드** — C11.4 및 다른 Grid 카드에는 새 링크가 섞이지 않는다. 통과: diff에 대상 카드 한 줄만 있다.
23. **WPT 주장** — 변경으로 Spinon WPT 적합성을 주장하지 않는다. 통과: 원래의 미검증 문구를 유지했다.
24. **플랫폼 주장** — AVD SwiftShader와 iOS 제출 전 Metal 한계를 넓히지 않는다. 통과: 기존 문장과 수치는 변경하지 않았다.
25. **지원 범위** — 일반 author CSS나 전체 Grid 지원으로 확대해 표현하지 않는다. 통과: public diff에 지원 주장이 없다.
26. **문서 권위** — 완료 판정은 `spec/STATUS.md`가 담당한다. 통과: 완료 체크나 공식 수를 수정하지 않았다.
27. **선택 빌드** — 전체 문서 빌드 오류가 새 링크 경로를 가리지 않도록 선택 root를 따로 빌드했다. 통과: RSPress 출력에서 17개 HTML route를 생성했다. 전체 docs link validation은 실행하지 않았다고 manifest에 기록했다.
28. **responsive 화면** — Agent Browser 390×844 CSS px에서 문서 가로 넘침이 없다. 통과: `scrollWidth=390`, 카드 폭 362px이며 mobile screenshot을 보관했다.
29. **선택 게시와 열린 파일** — 게시 목록은 manifest의 57개 route이며 삭제는 없다. 통과: Tailnet preflight에서 35 create·7 replace·15 same, 열린 변경 대상 0개를 확인했고 helper는 복사 직전 열린 파일을 다시 검사한다. 실제 복사는 PR 병합 뒤 실행한다.
30. **사후 무결성 확인** — 게시 helper는 선택 파일의 원격 HTTP 응답·SHA-256을 확인하고 교체 전 백업한다. 복사 직전에 `origin/main` roadmap이 선택 source와 같은지도 확인한다. 통과: 보호 경계를 검토했다. 실제 복사·원격 검증은 PR 병합 뒤 별도 결과로만 보고한다.

## 결과

30개 변경 검토를 마쳤다. PR image는 [Agent Browser desktop preview](roadmap-card-agent-browser-pass05.png)이며 viewport 1280×900 CSS px다. 390×844 결과는 [mobile layout JSON](mobile-layout-pass03.json)과 [mobile screenshot](roadmap-card-mobile-agent-browser-pass03.png)에 있다. Tailnet preflight는 통과했지만 복사·외부 URL 검증은 아직 실행하지 않았다.
