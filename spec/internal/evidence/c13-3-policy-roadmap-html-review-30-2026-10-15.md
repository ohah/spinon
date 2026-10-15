# C13.3 Tailnet public roadmap HTML 변경 검토 30개

대상은 `spec/public/roadmap.html`의 C13.3 simulator matrix 추가 문단이다. 구현 PR #185에 있는 코드·실행 결과를 Tailnet 정적 로드맵에 요약하는 문서 전용 변경이다.

1. **PR 식별자** — #185는 현재 main에 병합된 simulator matrix PR을 가리킨다.
2. **roadmap의 기존 연결** — 이전 computed-policy runtime evidence 링크를 보존한다.
3. **계획 링크** — 새 계획은 `main/plan/c13-3-policy-simulator-matrix.md`에 존재한다.
4. **실행 근거 링크** — 새 evidence README 경로와 공개 저장소 상대 경로를 확인했다.
5. **시나리오 수** — 각 platform 7개 fresh run, 총 14개 결과와 요약이 맞다.
6. **플랫폼 명칭** — Android는 API 37 ARM64 AVD, Apple은 iPhone 17 Pro/iOS 26.2 Simulator다.
7. **장치 한계** — simulator 실행을 실기기 검증처럼 부르지 않는다.
8. **seed와 첫 frame** — first-draw revision/seed 일치가 matrix gate에 포함된다.
9. **X policy 해석** — containment는 ancestor X offset을 막는 축별 결과다.
10. **Y policy 해석** — `auto` Y remainder는 outer로 전달되는 결과다.
11. **pixel 경계** — root 밖 changed pixel 0은 고정 screenshot 비교 범위에 한정된다.
12. **horizontal no-op** — contain-horizontal changed pixel 0은 입력이 처리됐으나 offset이 안 바뀐 결과를 정확히 적었다.
13. **재빌드 구분** — 구조 분리 후 최신 소스 확인은 containment horizontal smoke로만 표현한다.
14. **초기 matrix 시점** — 14-case capture를 latest-source full rerun으로 오해하지 않도록 evidence README의 구분을 따른다.
15. **archive 경계** — default 0/opt-in positive control 설명이 static archive 범위와 맞다.
16. **artifact 과장** — archive 결과를 최종 앱 번들 격리나 제품 ABI 지원으로 확대하지 않는다.
17. **부모 상태** — simulator 후속의 완료가 C13.3 전체 완료를 뜻하지 않는 문장을 유지한다.
18. **남은 C13 범위** — 관련성 없는 전체 CSS 지원 또는 상위 C13 완료를 암시하지 않는다.
19. **중복 roadmap 정보** — 이전 computed policy 결과는 이전 paragraph에 남고 새 matrix는 별도 paragraph로 구분된다.
20. **변경 국소성** — top nav·기존 evidence 문단·다른 roadmap 항목은 바뀌지 않았다.
21. **HTML 구조** — 새 문단은 기존 `<p>` 요소 안에서 inline `<strong>`·`<a>`만 사용한다.
22. **속성 표기** — 새 href는 큰따옴표로 감싸고 path slash/fragment를 임의로 바꾸지 않았다.
23. **문자열 종결** — 새 문장은 기존 C13.3 link period 뒤에 삽입되어 이전 텍스트를 덮어쓰지 않는다.
24. **상대/절대 URL** — Tailnet page는 main 저장소의 stable link를 쓰고, preview 내부에서 없는 `.html` 페이지를 약속하지 않는다.
25. **PR body와 일관성** — summary의 두 플랫폼·native swipe·축별 containment 설명이 PR #185와 맞다.
26. **상태 대장과 일관성** — 공식 `spec/STATUS.md`는 C13.3 전체 미완료 상태를 유지한다.
27. **Plan과 일관성** — plan의 matrix 및 scope limits와 roadmap 요약이 모순되지 않는다.
28. **성능 표현** — 측정하지 않은 지연/속도/효율 수치를 추가하지 않았다.
29. **scanout 표현** — OS compositor 뒤 표시 완료나 hardware GPU를 주장하지 않는다.
30. **교정 가능성** — evidence README가 원본 offset, pixel JSON, logs와 screenshots로 세부 결과를 추적한다.

## 판정

30개 관점에서 수정할 결함을 찾지 못했다. 이 요약은 공식 계획·상태 대장·PR #185 evidence로 연결되며, 사용자-facing web compatibility claim을 만들지 않는다.
