# C11.4 anonymous Grid text `white-space:pre` 병합 상태 문서 검토 20개

검토 대상은 PR #880 병합 뒤 갱신한 공식 상태, 상위 계획, 작업 계획, 내부 계약, 실행 근거다. 구현 및 PR diff 검토와 다른 문서 일관성·이력 실패 경로를 확인했다.

1. **PR이 실제로 병합되지 않았는데 병합으로 기재 — PASS.** GitHub PR #880 상태는 `MERGED`, `mergedAt=2026-10-30T23:06:20Z`다.
2. **merge SHA 오기 — PASS.** PR metadata의 merge commit은 `c592212bbf858cda0f310bb5885e864687960d06`이고 갱신 문서 네 곳의 값과 같다.
3. **PR head와 병합한 내용 혼동 — PASS.** 원래 PR head는 `2a208d47c931f588802048ed605cadcd0e947558`; 문서는 이를 merge commit으로 쓰지 않고 GitHub가 반환한 `mergeCommit.oid`를 사용한다.
4. **UTC 시각을 로컬 시각으로 잘못 기재 — PASS.** 네 문서 모두 GitHub `mergedAt`의 UTC ISO timestamp를 그대로 사용한다.
5. **empty check rollup을 CI success로 오인 — PASS.** 문서는 PR checks 미보고 및 main required status checks 미설정을 명시하고 성공 주장을 하지 않는다.
6. **C11/C11.4 상위 체크를 좁은 slice로 완료 처리 — PASS.** 공식 상태 및 계약은 두 체크를 미완료로 유지한다.
7. **전체 완료 수를 부분 구현으로 증가 — PASS.** 전체 완료 `72/213`을 그대로 기록한다.
8. **feature candidate를 일반 CSS 지원으로 확대 — PASS.** `white-space:pre`는 고정 Ahem opt-in fixture candidate로만 설명한다.
9. **검증 장면 수 계산 오류 — PASS.** 신규 5개와 기존 26개를 합친 31개가 양쪽 simulator에서 각각 31/31이다.
10. **focused test 숫자 전사 오류 — PASS.** Text-layout 59/59, HostDocument 31/31, runtime 3/3, FFI 1/1, boundary 24/24가 원 실행 기록과 일치한다.
11. **workspace 전체 test의 실패 누락 — PASS.** 262 pass와 C07.3 aspect-ratio serialization comparison 한 건의 failure 및 두 값이 유지된다.
12. **artifact 검사 범위를 앱 bundle까지 과장 — PASS.** 문서는 macOS ARM64 static archive의 0/5·0 대 5/5·2 결과만 적는다.
13. **실패 AVD 시도 은폐 — PASS.** SurfaceFlinger startup 실패와 cold-boot 후 전체 성공 matrix를 분리해 기술한다.
14. **simulator를 physical Android device로 오해 — PASS.** Android target은 API 37 ARM64 AVD / SwiftShader software renderer로 표기한다.
15. **iOS compositor 이후 pixel을 검증했다고 주장 — PASS.** iOS compositor 이후 출력과 glyph pixels가 미검증임을 계속 밝힌다.
16. **계획의 PR lifecycle 상태가 낡음 — PASS.** 구현 계획은 pending wording을 없애고 PR #880의 병합 및 체크 한계를 적는다.
17. **상위 C11.4 계획과 child plan 사이 merge state 불일치 — PASS.** 두 문서 모두 같은 PR, UTC timestamp와 merge SHA를 기록한다.
18. **공식 상태와 내부 Grid 계약의 이력 불일치 — PASS.** 둘 다 동일 merge proof와 CI 범위를 기록하고 내부 계약 버전 `0.1.0`을 유지한다.
19. **runtime evidence README와 공식 상태의 source attribution 불일치 — PASS.** README는 implementation PR #880을 연결하면서 원 실행 hash 및 platform evidence를 보존한다.
20. **문서 변경이 Tailnet preview source의 current main과 분리 — PASS.** 변경은 병합된 `main` 위에서 만들어졌고 preview sync는 후속 PR 병합 후 그 main을 기준으로 실행한다.

최종 상태: 병합 이력, 검증 경계, 공식 체크 및 집계가 서로 일치한다. 이 문서 갱신은 C11 구현 범위나 공식 완료 수를 바꾸지 않는다.
