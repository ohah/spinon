# 공개 로드맵 PR #1124 카드 변경 적대 검토 30개

**검토 대상:** `spec/public/roadmap.html`의 신규 C11.1 카드와 Agent Browser 화면·hash evidence. PR #1124의 기능 구현 리뷰 및 PR #1125의 병합 상태 문서 리뷰와 구별되는 이번 public page diff 검토다.

1. **PR 식별:** 신규 카드는 구현 PR #1124를 가리키며 문서 동기화 PR #1125를 기능 PR로 오인하지 않는다 — 통과.
2. **merge timestamp:** `2026-11-03T15:49:14Z`가 GitHub PR #1124의 실제 병합 시각과 일치한다 — 통과.
3. **merge SHA:** `e11df25732ef1a2c8365287b8ef475b55f6aa408`가 GitHub 및 공식 대장과 일치한다 — 통과.
4. **카드 순서:** 2026-11-04/PR #1124 카드가 더 오래된 2026-11-03/PR #1117 카드보다 앞에 있다 — 통과.
5. **상태 범위:** 카드가 bounded Root HTML implementation이라고 설명하고 C11.1/C11 전체를 완료로 표시하지 않는다 — 통과.
6. **공식 집계:** 카드가 공식 완료 `72/213`을 유지한다 — 통과.
7. **alignment 속성:** `align-items`, `justify-items`, `place-items`만 언급하고 self longhand를 지원한다고 넓히지 않는다 — 통과.
8. **eligible profile:** fixed-track Grid의 기존 eligible profile이라는 제한을 유지한다 — 통과.
9. **author CSS 범위:** 일반 author CSS 지원을 주장하지 않는다 — 통과.
10. **WPT 범위:** Chrome 비교를 Spinon WPT 적합성으로 해석하지 않는 문구가 있다 — 통과.
11. **독립 oracle:** 39 scene·42 item의 Chrome 156 관찰과 독립 기대값 범위를 유지한다 — 통과.
12. **projection 수치:** Root HTML projection 38/38 수치가 구현 evidence와 같다 — 통과.
13. **Android 기기 구분:** Android API 37 ARM64 AVD/SwiftShader를 실기기라고 부르지 않는다 — 통과.
14. **iOS 기기 구분:** iPhone 17 Pro Simulator를 물리 iPhone 결과라고 부르지 않는다 — 통과.
15. **runtime observation:** 앱 관찰은 `layout=ready boxes=83`, draw/present로 한정한다 — 통과.
16. **화면 표면 한계:** PixelCopy/Metal drawable readback이 없음을 카드에 표시한다 — 통과.
17. **CI 경계:** 비어 있는 check rollup을 CI 성공으로 표현하지 않는다 — 통과.
18. **required checks 경계:** main required status checks 미설정을 함께 표시한다 — 통과.
19. **계획 경로:** `/spinon/plan/c11-root-html-grid-item-alignment.html`이 source plan과 대응한다 — 통과.
20. **계약 경로:** `/spinon/internal/0059-c11-grid-layout.html`이 내부 계약 문서와 대응한다 — 통과.
21. **runtime evidence 경로:** `README.html` route가 기존 runtime evidence README와 대응한다 — 통과.
22. **implementation review 경로:** `implementation-runtime-review-20.html` source 파일이 존재한다 — 통과.
23. **PR review 경로:** `pr-diff-review-30.html` source 파일이 존재한다 — 통과.
24. **공식 상태 경로:** `STATUS.html` anchor 링크를 유지한다 — 통과.
25. **HTML 구조:** 새 카드가 기존 `kanban-note hero-progress` 문단 패턴을 따르고 다른 문단을 중첩하지 않는다 — 통과.
26. **escape/markup:** inline `<code>`와 anchor markup이 browser DOM에 보이고 문장 텍스트로 평탄화되지 않는다 — 통과.
27. **desktop layout:** Agent Browser 1280×1000에서 document width가 1280px이고 카드가 표시된다 — 통과.
28. **mobile layout:** Agent Browser 390×844에서 document width가 390px이고 가로 overflow가 없다 — 통과.
29. **screenshot fidelity:** README와 desktop/mobile screenshot 해시가 `SHA256SUMS.txt`와 일치한다 — 통과.
30. **change scope/process:** 최종 PR diff는 이 카드와 관련 화면 evidence로 제한되며 Agent Browser session과 전용 localhost server를 닫았다 — 통과.

화면은 source HTML을 로컬에서 제공한 visual evidence다. Tailnet publication, 전체 문서 링크 그래프, CSS/Grid 제품 동작을 증명하지 않는다.
