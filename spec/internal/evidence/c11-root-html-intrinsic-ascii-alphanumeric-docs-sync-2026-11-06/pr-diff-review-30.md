# C11.4 ASCII alphanumeric 상태·공개 roadmap PR diff 검토 30개

검토 대상은 PR [#1213](https://github.com/ohah/spinon/pull/1213)의 실제 GitHub diff, head `2b42235ff0753057a508c8b2c496782867a84d5a`이며 파일 5개다. 병합된 구현 PR #1212의 metadata와 구현 실행 근거를 STATUS, 내부 계약, 계획 요약 및 공개 roadmap에 대조했다.

1. **상태 집계 — PASS:** 변경 diff에 공식 완료 수 계산·상위 항목 checkbox가 없고 `78/214`를 명시적으로 유지한다.
2. **C11.4 완료 경계 — PASS:** C11.4 checkbox는 미완료 그대로이며 제한 slice를 상위 완료로 승격하지 않는다.
3. **C11 완료 경계 — PASS:** C11 상위 항목 미완료를 STATUS·계획·공개 roadmap에서 유지한다.
4. **PR 식별 — PASS:** STATUS와 내부 계약은 실제 merged PR #1212를 연결한다.
5. **Merge metadata — PASS:** GitHub PR의 merge timestamp `2026-11-06T06:56:57Z`와 full merge SHA `64483c9dd93741606de739891211058be612f954`가 문서에 일치한다.
6. **구현 의미 — PASS:** 상태 기록은 token allowlist 확장으로 설명하고 일반 CSS Grid 지원 완료라고 표현하지 않는다.
7. **Rust 결과 — PASS:** Style-to-layout 11/11 및 RuntimeSession default/fixture-test 각 15/15가 runtime evidence README와 일치한다.
8. **플랫폼 구분 — PASS:** API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 구분하며 Android 실기기라고 하지 않는다.
9. **STATUS 링크 — PASS:** C11.4 후보 행의 plan, 내부 계약, runtime evidence 상대 경로가 저장소 경로와 맞는다.
10. **C11.4 plan summary — PASS:** `c11-4-grid-track-sizing.md`에 #1212 merge SHA prefix가 full SHA와 일치한다.
11. **집중 plan metadata — PASS:** 전용 계획은 full merge SHA/timestamp와 10-scene evidence를 연결한다.
12. **계획 완료 경계 — PASS:** plan 상태가 merged로 바뀌었지만 C11.4/C11 checkbox와 `78/214`는 미변경이다.
13. **내부 계약 버전 — PASS:** 내부 contract `0.1.0`은 유지되며 범위 추가만으로 버전을 올리지 않았다.
14. **계약 이력 — PASS:** candidate 표현을 merged PR #1212로 바꾸고 같은 섹션의 상태 서술도 merged 제한 구현으로 정리했다.
15. **token grammar — PASS:** 내부 계약과 공개 roadmap 모두 `[A-Za-z0-9]+`를 정확히 쓴다.
16. **방향별 token — PASS:** 숫자-only, letter-then-digit, digit-then-letter 예시가 해당 구현/evidence와 맞는다.
17. **Separator contract — PASS:** SPACE/TAB/LF/CR 분리자 조건을 보존하고 다른 whitespace가 허용됐다고 주장하지 않는다.
18. **Rejection boundary — PASS:** Unicode letter/digit와 punctuation 거부 및 기존 fail-closed 한계를 유지한다.
19. **Runtime invariants — PASS:** source/node identity, source order, revision 및 기존 wrapping/layout 경계가 계약에서 유지된다.
20. **Scene matrix 규모 — PASS:** Chrome 157의 independent 10-scene track/row/line 비교를 근거로 기록한다.
21. **Independent oracle 구분 — PASS:** 기대값/사전 비교 문서와 runtime evidence를 각각 링크하고 모바일 표본을 전체 matrix로 확대하지 않는다.
22. **Android sample 범위 — PASS:** 지정 overflow 표본 PixelCopy 1/1 exact를 기록하고 AVD·SwiftShader 한계를 명시한다.
23. **iOS sample 범위 — PASS:** 지정 Metal drawable 표본 1/1 exact이며 compositor 제출 전이라는 한계를 공개 roadmap에 기록한다.
24. **Artifact boundary — PASS:** 기본 Rust rlib와 fixture-test executable scan 범위만 주장하며 APK/iOS app-bundle 전체 검사로 바꾸지 않는다.
25. **CI 주장 — PASS:** PR #1212에 보고된 status/workflow가 없다는 evidence와 일치하고 CI 통과로 표기하지 않는다.
26. **Public roadmap PR metadata — PASS:** #1212 link, merge time, merge SHA가 merged PR metadata와 일치한다.
27. **Public roadmap 위치 — PASS:** 새 카드가 관련 C11.4 ASCII letter-word card 직후에 들어가 후속 범위를 읽기 쉽다.
28. **Public HTML 범위 — PASS:** roadmap 변경은 새 진행 카드 한 단락이며 CSS·navigation·다른 작업 카드 동작을 바꾸지 않는다.
29. **Public links — PASS:** 계획, 내부 계약, precomparison, runtime evidence는 main 경로를 쓰고 화면 PNG는 병합된 SHA의 raw content를 가리킨다.
30. **Tailnet 시점 — PASS:** PR 본문은 Tailnet 제공본을 아직 갱신했다고 주장하지 않고, #1213 병합 후 별도 동기화·Agent Browser 근거를 남긴다고 명시한다.

**발견 사항:** 문서 diff에서 수정이 필요한 불일치는 찾지 못했다. Tailnet URL의 실제 제공본은 이 PR 병합 전이라 아직 이 변경을 반영하지 않으며, 병합 후에만 동기화한다.
