# C11.4 depth-three 계획 PR diff 실패 관점 검토 30개

계획 PR에는 로드맵 계획·oracle·상태 문구만 포함한다. 아래 검토는 계획 내부의 20개 실패 관점과 구분하여 실제 PR 파일/링크/판정 경계를 다시 읽었다.

1. **depth 용어** — depth 3이 Grid container 수 네 개와 nesting 경계 세 개를 뜻하는지 확인했다. 계획과 HTML이 root → middle → branch → inner로 일치한다.
2. **새 scene ID** — 계획의 두 allowlisted ID가 HTML·독립 JSON·Chrome JSON에서 철자와 순서까지 같은지 확인했다. 일치한다.
3. **기존 controls** — plan과 parent roadmap은 depth-one/two 양 축 여덟 장면을 회귀 controls로 유지한다. 새 fixture oracle는 두 신규 scene만 담고 그 범위를 분명히 한다.
4. **HTML fixture 링크** — plan의 상대경로가 실제 c11 fixture 경로를 가리키는지 확인했다. 파일이 존재한다.
5. **plan 상위 링크** — Grid track sizing parent와 depth-two plan 상대링크가 실제 파일에 연결된다.
6. **내부 계약 링크** — internal 0059 경로가 유효하며 숫자 버전 변경을 계획하지 않는다.
7. **상태 anchor** — STATUS의 C11 CSS Grid section anchor와 새 계획 링크의 경로를 확인했다.
8. **parent roadmap 링크** — 새 계획, 사전 비교 README, plan review 20개 링크가 같은 저장소 파일을 가리킨다.
9. **evidence README 링크** — plan에서 README를 통해 Chrome observation, 기대값, screenshot에 도달한다.
10. **plan diff review 미래 링크** — 현재 PR review 근거 파일을 이 계획에 추가한 뒤 링크가 해소되도록 변경 파일 목록에 포함했다.
11. **Ahem URL** — HTML의 상대 font URL이 저장소 Ahem asset에 연결되는지 확인했다. Agent Browser capture에서 ahemReady=true였다.
12. **font race** — 측정 스크립트가 document.fonts.ready 이후 결과를 기록한다.
13. **font identity 오인** — fallback sans-serif가 oracle로 저장되지 않도록 font check 값을 관측 JSON에 기록한다.
14. **min-content 기대값** — 공백 없는 최대 word의 Ahem 70px advance로 계산하고 CSS Grid 규범 계산과 분리했다.
15. **max-content 기대값** — collapsible whitespace advance를 포함한 100px total advance를 독립 기대값 JSON에 고정했다.
16. **row 기대값** — 70px에서 두 10px line, 100px에서 한 10px line으로 20px/10px block size를 유도한다.
17. **재귀 기대 가정** — 각 parent에 direct child가 하나뿐인 fixed case임을 써서 값을 depth 임의 지원으로 일반화하지 않는다.
18. **CSS 규범 source** — primary CSS Grid 2025 CRD의 container sizing, algorithm order, item contribution sections에 직접 연결한다.
19. **Chrome 관측 독립성** — browser observation과 spec-derived expectations가 별개 JSON이며 plan도 Chrome을 규범 근거로 쓰지 않는다.
20. **Chrome build/version** — HeadlessChrome 156 user agent, macOS ARM, DPR 1과 viewport를 observation과 README에 기록했다.
21. **track serialization** — computed columns/rows 문자열과 used width/height가 JSON의 numeric expectation과 비교 가능하다.
22. **page origin** — local absolute x/y는 저장되지만 page layout이 normative geometry로 취급되지 않고 root-relative 비교를 계획한다.
23. **tolerance** — 1/64 CSS px 허용 오차가 expected JSON, README와 plan에서 일치한다.
24. **screen evidence** — screenshot은 fixture를 보는 시각 보조 자료로 두고 runtime conformance나 glyph pixel 증거라고 하지 않는다.
25. **screenshot contents** — 캡처에서 min-content/max-content label, Ahem glyph bar와 두 장면을 확인했다.
26. **screenshot hash** — README manifest의 HTML, expected JSON, observation, screenshot SHA-256 네 항목이 모두 shasum -c를 통과했다.
27. **product scope** — opt-in fixture-only 경계와 공개 API 비변경을 명시해 style admission 일반화로 읽히지 않는다.
28. **negative controls** — 미등록 scene, depth-four, 추가 child와 잘못된 intrinsic 조합을 fail-closed 검사로 포함한다.
29. **platform claims** — Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator만 다음 실행에 지정하고 실기기, hardware GPU, WPT를 제외한다.
30. **official status** — PR #453 진행 중으로 남아 있던 문구를 병합 상태로 고치고 기준 날짜를 2026-10-22로 맞췄다. C11.4 미완료, 72/213과 interface version 0.1.0은 유지한다.

## 결과

초기 링크 검사에서 plan → fixture 경로를 확인했고, 최종 상대경로는 c11 디렉터리를 포함한다. 독립 기대값은 browser observation과 track/frame 크기별로 대조됐고, manifest 네 항목은 모두 SHA-256 검증을 통과했다. 계획/상태/parent roadmap의 새 링크를 별도로 검사해 이 PR 범위 안의 끊어진 경로를 남기지 않았다.
