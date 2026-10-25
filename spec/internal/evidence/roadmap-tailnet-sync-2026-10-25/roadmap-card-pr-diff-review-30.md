# C11.4 행 축 로드맵 카드 PR 변경 검토 30개

검토 대상은 `spec/public/roadmap.html`에 추가한 PR #620 카드와 연결된 공개 근거 경로다. 계획 검토와 runtime 구현 검토를 재사용하지 않는다.

1. **섹션 순서가 오래된 작업을 최신처럼 보일 수 있음** — 카드가 #615 항목 뒤에 놓여 날짜와 흐름이 맞는다.
2. **PR 링크가 선행 계획 PR을 가리킬 수 있음** — 구현 링크는 #620을 가리킨다.
3. **병합 상태가 근거 없이 표기될 수 있음** — GitHub PR state가 `MERGED`다.
4. **병합 시각이 로컬 시간으로 혼동될 수 있음** — UTC `2026-10-25T02:29:00Z`를 썼다.
5. **merge commit을 source commit으로 오해할 수 있음** — merge SHA와 최종 검증 source SHA를 각각 표기했다.
6. **행 축과 열 축이 뒤바뀔 수 있음** — 제목, 설명, 링크 대상이 모두 row-axis 후보를 가리킨다.
7. **text/image 순서가 명시되지 않을 수 있음** — forward와 reversed source-order scene을 기록했다.
8. **min-content만 시험한 것으로 오해할 수 있음** — min-content와 max-content 두 keyword를 기록했다.
9. **keyword별 출력이 구분된다고 오해할 수 있음** — 현재 fixture에서 sizing 차이를 구별하지 못한다고 밝혔다.
10. **scene 수가 런마다 달라 보일 수 있음** — 고정 네 opt-in scene으로 한정했다.
11. **Android 기기 구성이 모호할 수 있음** — API 37 ARM64 16 KB AVD를 명시했다.
12. **Android 4/4의 분모가 불명확할 수 있음** — 네 scene matrix라고 문맥에서 명시했다.
13. **iOS 대상이 실기기로 오해될 수 있음** — iPhone 17 Pro/iOS 26.2 Simulator라고 명시했다.
14. **iOS 4/4의 분모가 불명확할 수 있음** — 같은 네 scene을 플랫폼별로 따로 세었다.
15. **column regression이 row 통과에 섞일 수 있음** — 별도의 #611 회귀 결과로 분리했다.
16. **공유 trace matcher 확인을 전체 layout 보증으로 읽을 수 있음** — 기존 column regression 확인으로 한정했다.
17. **화면 링크가 잘못된 파일을 가리킬 수 있음** — Android PNG 경로가 현재 runtime evidence 디렉터리에 존재한다.
18. **iOS 화면 링크가 잘못된 파일을 가리킬 수 있음** — iOS PNG 경로가 현재 runtime evidence 디렉터리에 존재한다.
19. **화면 설명이 픽셀 일치를 암시할 수 있음** — box 배치 근거라고 밝혀 pixel equality를 주장하지 않는다.
20. **simulator evidence가 실기기 증거로 확대될 수 있음** — 실기기 결과가 아니라고 적었다.
21. **AVD 결과가 hardware GPU 증거로 확대될 수 있음** — hardware GPU proof가 아니라고 적었다.
22. **fixture 결과가 일반 CSS 지원으로 읽힐 수 있음** — 일반 CSS admission 제한을 명시했다.
23. **고정 scene 결과가 WPT conformance로 확대될 수 있음** — WPT를 검증하지 않았다고 적었다.
24. **후보 성공이 상위 완료 상태를 바꿀 수 있음** — C11.4 미완료 문구를 유지했다.
25. **표시 숫자가 상태 대장과 어긋날 수 있음** — 공식 `72/213`을 그대로 유지했다.
26. **집중 계획 경로가 열리지 않을 수 있음** — 계획 markdown에서 생성된 HTML route가 선택 build에 포함됐다.
27. **Chrome oracle 경로가 누락될 수 있음** — precomparison README HTML과 raw evidence directory가 선택 build에 포함됐다.
28. **실행 세부 근거 경로가 누락될 수 있음** — runtime README, 구현 검토, PR 검토의 HTML route를 선택했다.
29. **상태 페이지가 예전 상태를 보여줄 수 있음** — 현재 `spec/STATUS.md`와 그 HTML route를 선택 목록에 포함했다.
30. **HTML 추가가 바깥 section을 닫거나 기존 문서를 잘라낼 수 있음** — 삽입 위치가 `</section>` 전의 독립 `<p>`이며 `git diff --check`와 RSPress build가 통과했다.

확인 결과: 변경 diff에서 별도 결함을 찾지 못했다. 소스의 사실·링크 경로는 선택 build에서 확인했고, 실제 Tailnet HTTP/hash와 Agent Browser 화면 확인은 병합 뒤 기록한다.
