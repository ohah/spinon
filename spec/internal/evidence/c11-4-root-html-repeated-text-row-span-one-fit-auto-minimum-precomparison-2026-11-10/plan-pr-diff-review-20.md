# 계획 PR 변경분 검토 20개

검토 대상은 one-fit row-span `min-height:auto` 계획 PR의 staged diff, fixture, 5,760-scene Chrome 관측과 상태/내부 계약 변경이다. 계획 자체의 20개 실패 관점 검토와 분리해 실제 PR 파일·링크·주장 경계를 확인했다.

1. **공식 완료 상태 오기:** `spec/STATUS.md`는 C11.4 미완료와 `78/214`를 유지한다. **통과.**
2. **계획을 구현 완료로 표시:** 신규 행은 미구현 계획 후보라고 명시하고 제품 코드를 바꿨다고 하지 않는다. **통과.**
3. **내부 숫자 계약 변경:** internal `0.1.0` version은 유지한다. **통과.**
4. **공개 API/FFI 지원 주장:** 공개 surface 추가를 주장하지 않고 내부 bounded candidate로 기록한다. **통과.**
5. **상태 대장과 내부 계약의 범위 불일치:** 두 문서 모두 positions 1–3, one-fit, row-span, all-auto text minimum으로 한정한다. **통과.**
6. **기존 implementation index와 링크 깨짐:** plan index에서 계획·검토·evidence 경로를 상대 경로로 연결한다. 변경한 7개 Markdown의 신규 후보 링크 21개를 검사해 누락 0건을 확인했다. **통과.**
7. **계획 부모/선행 관계:** one-fit row-span, positions 4–6, two-fit span, one-fit auto-placement 선행 문서로 연결한다. **통과.**
8. **규범 출처 불일치:** W3C 2025 CRD §6.6/§7.2/§11.5를 직접 링크하며 fit-content auto minimum 설명이 oracle과 맞는다. **통과.**
9. **fixture가 계획 입력과 다름:** explicit row, 40×220 geometry, 6-member list, 2/3 text items, 2/3-row span 및 fixed text style이 계획과 맞는다. **통과.**
10. **scene count 산술 오류:** 3×30×4×2×4×2 = 5,760이며 baseline/candidate 각각 2,880이다. Browser summary와 NDJSON count가 일치한다. **통과.**
11. **비교 cohort 비대칭:** scene key는 fit position/tail/cap/alignment/topology를 보존하고 minimum mode만 달라 2,880 pairs가 있다. **통과.**
12. **fit position이 implicit list index와 혼동:** explicit row는 별도이며 positions 1–3만 tested three-row topology에 매핑된다. **통과.**
13. **content-floor 통계 과장:** baseline violation 0을 명시하고 “overflow 복구” 주장 없이 automatic minimum의 row/frame 차이로 결론을 제한한다. **통과.**
14. **source-order 결과 누락:** forward/reverse 2,880 pair를 점검해 max delta 0 CSS px임을 summary에 남겼다. **통과.**
15. **stretch 결과 누락:** 2,880 start/stretch pair에서 non-auto track max delta 0이며 auto-only growth를 기록한다. **통과.**
16. **cap/percentage type 손실:** `0%`, `20%`, `120%`, `20px`를 분리하고 definite 220px basis를 적었다. **통과.**
17. **evidence hash 불일치:** `SHA256SUMS`의 fixture, capture script, summary, raw observations, screenshot 및 README가 모두 검증된다. **통과.**
18. **screenshot이 수치 근거처럼 표현됨:** Chrome screenshot은 paired visual illustration이고 scene/geometry 판단은 별도 NDJSON·verifier라고 구분한다. **통과.**
19. **Agent Browser lifecycle/Terminal Browser 혼동:** browser session close, 빈 session list, daemon process 종료를 기록했다. 기존 Terminal Browser 프로세스는 사용·종료하지 않았다. **통과.**
20. **실기기·simulator·CI 범위 오기:** 이 PR은 Chrome 사전 비교만 포함하며 Android/iOS, product runtime, WPT, CI 또는 Tailnet publication을 이미 실행했다고 주장하지 않는다. **통과.**

최종 staged diff의 `git diff --cached --check`와 precomparison `SHA256SUMS` 검증도 통과했다. 이 검토는 실제 계획 PR 변경분을 대상으로 했으며, 계획 문서의 별도 20개 검토나 후속 implementation/runtime 검토를 대신하지 않는다.
