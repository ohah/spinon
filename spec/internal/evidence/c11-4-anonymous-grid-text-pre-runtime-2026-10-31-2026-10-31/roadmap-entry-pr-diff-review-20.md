# C11.4 `white-space:pre` 공개 로드맵 항목 실패 관점 검토 20개

1. **병합 전 PR을 완료로 표시 — PASS.** 항목은 구현 PR #880의 병합 시각과 SHA를 기준으로 기록한다.
2. **PR 번호·merge SHA 오기 — PASS.** #880과 `c592212bbf858cda0f310bb5885e864687960d06`을 GitHub 결과와 대조했다.
3. **부분 slice가 C11.4 checkbox를 완료 — PASS.** 문구는 제한 fixture 구현으로 구분하고 C11.4/C11 미완료를 명시한다.
4. **전체 공식 완료 수 변경 — PASS.** `72/213`을 그대로 유지한다.
5. **feature 범위를 일반 CSS admission으로 확대 — PASS.** Ahem direct anonymous text opt-in fixture라고 한정한다.
6. **새 장면 수 누락 — PASS.** 신규 five scenes를 기록한다.
7. **회귀 matrix 장면 수 누락 — PASS.** 기존 26개를 포함해 플랫폼별 31/31로 기재한다.
8. **Android와 iOS 결과를 합쳐서 잘못 셈 — PASS.** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 각각 31/31을 구분한다.
9. **AVD를 실기기나 hardware GPU로 표현 — PASS.** Android는 SwiftShader software renderer로 명시한다.
10. **iOS compositor 이후 pixel 검사 주장 — PASS.** iOS 근거가 compositor 제출 전임을 밝힌다.
11. **glyph paint를 시각 검증했다고 주장 — PASS.** glyph pixels 미검증을 적는다.
12. **workspace test failure 숨김 — PASS.** 262 pass 후 변경 밖 C07.3 문자열 비교 한 건 실패를 기록한다.
13. **PR check가 없는데 CI green 표현 — PASS.** check rollup 없음과 required checks 부재를 설명하고 통과라 부르지 않는다.
14. **CSS Text의 주요 `pre` 동작 생략 — PASS.** 공백 보존, no-wrap, forced LF break, fixed-width overflow를 포함한다.
15. **50px·25px geometry 역할 혼동 — PASS.** text line overflow를 고정-width Grid root와 구분한다.
16. **지원 API나 제품 contract가 늘어난 것으로 오해 — PASS.** 내부 fixture 예외를 공개 CSS/API 지원과 구별한다.
17. **오래된 C11 항목 위치 — PASS.** 새 entry를 기존 `break-spaces`/direct-text records 앞에 배치해 최신순 흐름을 유지한다.
18. **누락된 근거 링크 — PASS.** 계획, Chrome 156 precomparison, runtime evidence와 공식 상태를 연결한다.
19. **Chrome screenshot을 앱 출력으로 오인 — PASS.** 링크 이름은 Chrome precomparison이며 해당 README는 oracle screenshot이 Spinon output이 아니라고 명시한다.
20. **공개 로드맵과 상태 대장 불일치 — PASS.** 공개 페이지가 C11.4 미완료·`72/213`을 유지하며 세부 구현 기록은 상태 대장의 PR #880 항목을 따라간다.

최종 상태: 이 항목은 진행 상황 기록만 추가하며 공식 완료 판정을 바꾸지 않는다.
