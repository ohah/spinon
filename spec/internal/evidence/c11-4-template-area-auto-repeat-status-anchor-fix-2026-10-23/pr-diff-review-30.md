# C11.4 로드맵 상태 링크 수정 PR diff 검토 30개

대상 PR은 `spec/public/roadmap.html`의 #539 실행 기록에서 상태 페이지 fragment 한 곳을 바꾸고, PR #540 검토 기록에 pass65 브라우저가 찾은 anchor 누락과 수정 사실을 남긴다. 구현 코드, 상태 대장, 계약 버전은 바꾸지 않는다.

1. **원인 식별** — 이전 fragment가 렌더 DOM에 실제로 없었는가? Agent Browser는 `#c11-css-grid`의 target을 찾지 못했다.
2. **수정된 target** — 새 fragment가 실제 heading과 일치하는가? `#css-구현-체크리스트`가 렌더된 h2다.
3. **렌더 산출물 검증** — Markdown source에만 의존하지 않고 생성된 pass65 HTML에서도 target을 확인했는가? Agent Browser가 `getElementById` 성공을 확인했다.
4. **percent encoding** — URL 내 한글 fragment가 브라우저에서 올바르게 해석되는가? URL fragment를 percent encoded route로 열고 decoded ID 존재를 확인했다.
5. **base prefix** — 정적 문서가 `/spinon/` base 아래에서 열리는가? 전체 Tailnet route가 해당 prefix를 유지한다.
6. **페이지 route** — `STATUS.html` 파일 경로가 실제로 열리는가? Tailnet에서 문서 제목과 내용을 확인했다.
7. **fragment 목적** — target이 CSS 체크리스트 섹션에 도착하는가? 화면에서 “CSS 구현 체크리스트” 제목을 확인했다.
8. **주변 문구** — 링크 문구가 세부 C11.4 항목으로 직접 이동한다고 과장하는가? 기존의 일반 “공식 상태” 라벨을 유지한다.
9. **변경 범위** — unrelated older roadmap anchors까지 일괄 변경되는가? #539 기록의 한 링크만 바꾼다.
10. **PR 연결** — 링크가 병합된 runtime PR #539 기록 안에 남는가? 같은 paragraph와 같은 PR 근거에 둔다.
11. **개수 정확성** — 새 링크나 중복 링크를 만들지 않았는가? 기존 status link 하나의 href만 바뀐다.
12. **HTML 구조** — paragraph 경계와 나머지 inline tags가 보존되는가? 변경은 href attribute 하나다.
13. **HTML 인코딩** — 한글이 저장소 UTF-8과 브라우저에서 일치하는가? source UTF-8 텍스트와 decoded browser fragment를 대조했다.
14. **검토 기록 이력** — PR #540의 원래 검토가 완전 통과였다고 계속 주장하는가? pass65에서 누락된 30번과 후속 correction을 기록한다.
15. **수정 관점 경계** — 새 PR의 30개 리뷰가 이전 30개 표를 재사용하는가? 이 문서는 anchor 변경 고유의 새 실패 경로로 작성한다.
16. **상태 승격** — 문서 교정이 C11.4를 완료로 표시하는가? 공식 완료 표시는 바꾸지 않는다.
17. **완료 수** — `77/213`에 영향을 주는가? 상태 대장을 수정하지 않는다.
18. **계약 버전** — 내부 계약 0059를 올리는가? 계약 문서와 `0.1.0`을 변경하지 않는다.
19. **실행 수치** — 13/13 또는 66/66 설명이 바뀌는가? 이 수정은 숫자·실행 주장을 건드리지 않는다.
20. **시뮬레이터 근거** — anchor 화면을 앱 동작 증거로 과장하는가? PR 본문 캡처는 상태 문서 heading 위치를 보여준다.
21. **실기기 주장** — CSS checklist 브라우저 화면이 실기기 결과로 해석될 수 있는가? 캡처에는 플랫폼 실행 주장을 추가하지 않는다.
22. **Tailnet 호스트** — 캡처 URL이 다른 사이트나 임시 localhost를 가리키는가? `macstudio.tailed42f2.ts.net`을 사용한다.
23. **화면 내용** — screenshot이 수정 대상의 CSS checklist heading을 실제로 보여주는가? pass65 Agent Browser capture를 함께 둔다.
24. **기존 링크** — 계획·계약·실행 README·PNG 링크가 바뀌는가? 해당 href는 그대로 둔다.
25. **JS 의존성** — fragment 이동에 새 script나 client-side action이 필요한가? 기본 HTML anchor만 사용한다.
26. **접근성** — 링크 텍스트가 기존 “공식 상태”로 의미를 설명하는가? 보이는 label을 유지한다.
27. **배포 순서** — 변경한 소스가 live preview에도 반영되어야 하는가? PR 병합 뒤 roadmap 한 파일만 pass66 delta sync하고 hash를 확인한다. 이는 아직 실행 전 단계다.
28. **삭제 위험** — sync가 무관한 preview 파일을 지우는가? pass66은 roadmap 한 파일만 원자 교체하고 삭제하지 않는다.
29. **검증 경계** — Tailnet status page anchor 확인을 공식 상태의 앱 검증으로 읽는가? 이 결과는 문서 탐색만 검증한다.
30. **최종 결과** — 수정 후 이전 30번 review 항목을 해결했다고 판정할 기준이 있는가? pass66 브라우저가 `document.getElementById("css-구현-체크리스트")`와 화면 heading을 확인해야 통과다.

## 결과

source diff와 pass65에서 이미 가능한 target 검증을 기준으로 30개 관점을 검토했다. Tailnet road map은 아직 이전 href를 제공하므로 이 PR merge 뒤 pass66 sync와 브라우저 확인이 남아 있다. 그 마지막 확인 전까지 anchor fix의 Tailnet 배포 완료로 판정하지 않는다.
