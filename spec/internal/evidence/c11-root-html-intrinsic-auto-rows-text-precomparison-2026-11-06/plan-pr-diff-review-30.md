# C11.4 implicit min-content row text 계획 PR diff 검토 30개

대상 diff는 text child 하나를 가진 Root HTML Grid item을 기존 min-content implicit-row 제한 경로에 추가하기 위한 계획, Chrome 사전 비교 fixture·근거와 계획 검토 20개다. 제품 코드나 공식 완료 상태는 변경하지 않는다.

1. **범위 경계:** 계획은 기존 기본 Root HTML 경로만 확장하며 새 profile, FFI selector, public API를 제안하지 않는다.
2. **계획 계보:** 기존 empty-div `grid-auto-rows:min-content` 계획을 연결해 상위 범위와 후속 범위를 구분한다.
3. **Track function 정체성:** 새 값은 singleton `min-content`이며 `max-content`, `auto`, `minmax()`를 성공 범위로 오인하지 않는다.
4. **Item 정체성:** text는 direct `<div>` item에 귀속되고 Grid owner의 anonymous text는 계속 제외된다.
5. **Text run 한계:** text node 하나만 허용하며 inline element, 여러 run, replaced content를 지원한다고 넓히지 않는다.
6. **줄바꿈 조건:** 40px explicit column이 줄바꿈 가능한 inline-size 기준을 제공한다.
7. **Font 입력:** font family, size, line-height와 white-space를 fixture에서 확인할 수 있다.
8. **Font 이식성:** 일반 monospace advance를 모든 OS에서 같은 값으로 주장하지 않는다.
9. **기준 pinning:** observation은 HeadlessChrome 156.0.0.0과 DPR 1을 기록한다.
10. **Fixture 정체성:** expected JSON의 source hash가 저장된 HTML fixture와 일치한다.
11. **줄 수:** Chrome text range가 두 줄이며 계획도 이 결과를 기록한다.
12. **Block contribution:** 두 줄 × 10px line-height의 기대 row contribution은 20px이다.
13. **Explicit row:** 첫 explicit row 10px을 implicit contribution과 구분한다.
14. **Sibling contribution:** 마지막 item의 고정 34px 높이가 별도 implicit row를 만든다.
15. **Track 순서:** 기대 row `[10,20,34]`는 source order와 sparse row placement에 맞는다.
16. **Frame 순서:** 기대 item y 좌표 0, 10, 30은 track 시작 위치와 일치한다.
17. **Overflow:** visible content bottom y=64는 40px container보다 24px 아래다.
18. **고정값 대조:** 임시 10px track pattern 대조에서 `[10,10,10]` geometry를 기록한다.
19. **대조 해석:** 고정값 대조의 text overflow를 clipping이나 glyph pixel 결과라고 확대하지 않는다.
20. **Computed track 검사:** item frame뿐 아니라 Taffy track trace base와 row line도 acceptance 기준으로 확인한다.
21. **Typed cascade:** CSSOM 재파싱을 금지하고 computed typed style과 revision identity를 요구한다.
22. **원자적 실패:** 지원하지 않는 topology가 partial frame/paint scene을 게시하지 않는지 검증한다.
23. **Text negative 경로:** anonymous text, nested markup, 여러 run, stale revision과 wrong owner를 범위 밖에 둔다.
24. **숨김 content:** 빈/hidden item이 text contribution을 만들지 않는 회귀 대조를 요구한다.
25. **Resource cap:** 기존 10,000-track 및 checked-count 제한을 변경하지 않는다.
26. **플랫폼 행렬:** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 지정한다.
27. **증거 종류:** screenshot, layout state, 앱이 출력한 NodeId frame, PixelCopy/Metal readback을 서로 구분한다.
28. **검사 보고:** broad suite와 실행하지 않은 gate를 green이라고 보고하지 않도록 했다.
29. **상태 집계:** C11.4/C11 checkbox와 공식 `78/214`를 유지한다.
30. **Browser 수명:** Agent Browser는 observation screenshot/JSON 뒤 종료했고 Terminal Browser는 사용하지 않았다.
