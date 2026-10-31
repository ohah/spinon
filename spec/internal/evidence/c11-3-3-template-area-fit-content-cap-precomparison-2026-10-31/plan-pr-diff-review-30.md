# C11.3.3 area-backed fit-content cap 계획 PR diff 실패 관점 검토 30개

계획 자체의 실패 관점 20개와 구분해, 실제 계획 PR의 상태 문구·fixture·관찰·링크·범위 경계를 다시 읽었다. 검토 대상은 [계획](../../../../plan/c11-3-3-template-area-fit-content-cap.md), [STATUS 대장](../../../STATUS.md), 이 디렉터리의 고정 fixture와 evidence다.

1. **계획 상태 문구** — Chrome-only이며 runtime/Simulator 실행 전이라고 명확히 기록한다.
2. **official parent state** — C11.3.3, C11, C11.4를 완료로 표시하지 않는다.
3. **공식 집계** — plan과 STATUS entry가 `72/213`을 유지한다.
4. **upstream dependency** — typed area-backed `grid-auto-*`의 근거가 병합된 PR #911임을 기존 C11.3.3 기록과 맞춘다.
5. **scope** — 한 area-generated column의 fit-content cap만 다루며 전체 Grid sizing matrix라고 말하지 않는다.
6. **CSS authority link** — 2025 CSS Grid Level 1 CRD 원문 URL이 유효한 기준이다.
7. **track formula** — `max(auto minimum, min(limit, max-content))` 순서와 40px limit을 계획·기대값에서 일치시킨다.
8. **font advance math** — Ahem 10px에서 `AA`는 20px, 네 단어와 세 공백은 110px max-content다.
9. **min-content measurement** — 가장 긴 공백 없는 단어가 독립 probe에서 20px로 관측된다.
10. **max-content measurement** — 한 줄 probe가 110px로 관측되고 fallback font 측정이 아니다.
11. **font identity** — Ahem asset SHA-256이 evidence JSON에 고정된다.
12. **font readiness** — measurement promise가 `document.fonts.ready` 뒤 resolve되고 check/status가 observation에 남는다.
13. **area geometry** — explicit 20px column 하나와 2-column area template가 두 번째 track을 요구한다.
14. **auto track grammar** — `grid-auto-columns` 값은 이 fixture에서만 `fit-content(40px)`이다.
15. **container definiteness** — 200×60px root가 definite sizing basis를 제공한다.
16. **item placement** — `grid-area:fit`이 area 생성 track과 두 번째 item을 연결한다.
17. **item width source** — fit item에 별도 width를 고정하지 않아 cap이 다른 constraint에서 오지 않는다.
18. **item height** — 60px row area에서 stretch frame 60px를 예상값으로 기록한다.
19. **gap/edge reset** — root, body, item의 margin/padding/border/gap reset이 fixture에 있다.
20. **CSS selector binding** — `#fixed-area`와 `#fit-content-item` selector가 실제 element ID와 일치한다.
21. **root remainder** — 20px+40px track 뒤 root background가 남아 cap 경계를 보일 수 있다.
22. **screenshot pixel centers** — `(10,30)`, `(59,30)`, `(60,30)` 표본은 서로 다른 fixed/fit/outside 영역을 검사한다.
23. **pixel values** — 직접 PNG에서 red `[229,72,77,255]`, green `[48,164,108,255]`, root `[16,24,32,255]`를 확인했다.
24. **coordinate units** — 1280×633 viewport와 DPR 1을 고정해 CSS px와 screenshot px가 일치한다.
25. **screenshot integrity** — 1280×633 PNG SHA-256이 observation과 실제 파일에서 일치한다.
26. **oracle provenance** — Chrome 156.0.8078.76와 Agent Browser 0.31.1을 기록하고 Terminal Browser를 사용하지 않았다.
27. **evidence boundary** — screenshot과 observation을 Chrome oracle로만 취급하고 Spinon/WPT 증거로 부르지 않는다.
28. **profile integration rationale** — area expansion과 text intrinsic 경로가 서로 다른 profile임을 계획에 적고 fixture-only bridge로 제한한다.
29. **platform boundary** — 다음 단계의 AVD SwiftShader와 iOS pre-compositor Metal readback을 실기기/hardware/compositor 이후 결과와 분리한다.
30. **relative file links** — plan, STATUS, evidence README, HTML, expected JSON, observation, screenshot, 두 review 문서 상대경로가 실제 파일로 해석된다.

## 결과

초기 시각 검토에서 HTML 색상 선택자가 실제 element ID와 어긋난 문제를 찾았다. selector를 고치고 Agent Browser 캡처와 measurement를 반복했다. 최종 화면의 geometry는 그대로였고 세 경계 pixel은 기대 RGBA와 일치했다. 최종 fixture·PNG hash와 JSON 값을 다시 대조했다. plan 및 STATUS는 그 최종 evidence를 참조한다. 구현 전 단계이며 PR merge 이후에도 이 자료만으로 기능을 완료 처리하지 않는다.
