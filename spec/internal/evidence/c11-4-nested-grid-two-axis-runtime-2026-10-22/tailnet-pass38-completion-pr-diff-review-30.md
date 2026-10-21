# Tailnet pass38 완료 기록 PR diff 적대 검토 30개

검토 대상은 `spec/STATUS.md`의 2026-10-22 C11.4 진행 문장을 pass38 완료 사실로 교체하고, 상세 수치와 한계를 가진 동기화 README를 추가한 변경이다. 근거는 pass38 selected-build summary, 로컬 target·backup 검증, Tailnet 원격 검증, Agent Browser 관찰 기록 및 이미 병합된 PR #453·#455다.

1. **변경 범위:** pass38 진행 문장 한 곳을 완료 사실로 고쳤고 별도 동기화 README를 추가했다. 다른 C11.4 결과는 다시 쓰지 않았다. PASS.
2. **기록 주체:** 이 문장은 이번 pass38 갱신을 뜻하고 다음 pass의 상태를 예고하지 않는다. PASS.
3. **PR #455 참조:** source main SHA에 공개 roadmap PR #455 merge 내용이 포함된다. PASS.
4. **source SHA:** `571a3aa9cb894e6cb6ccd2cde1d26078f5df4b86`를 GitHub merge commit 및 현재 pass38 summary와 대조했다. PASS.
5. **source tree:** `90e126f43048120cadf1dcbee461917e3b848487`가 그 commit의 tree와 일치한다. PASS.
6. **도구 버전:** Rspress `2.0.22`는 workspace lock과 실제 pinned build log에 일치한다. PASS.
7. **선택 build 범위:** full documentation build를 성공했다고 쓰지 않고 selected build라고 제한한다. PASS.
8. **대상 수:** 2,910은 selected site와 preflight target count에 일치한다. PASS.
9. **HTTP 상태:** 2,910개 전부 200이고 실패가 0인 remote summary를 대조했다. PASS.
10. **hash 상태:** 전체 응답이 selected build의 byte 수·SHA-256에 일치한 원격 기록과 대조했다. PASS.
11. **partial success 과장:** 전부 일치한다는 표현은 `all_verified=true`에 근거한다. PASS.
12. **origin 범위:** 이 결과를 Tailnet preview에 한정하며 GitHub Pages 배포라고 표현하지 않는다. PASS.
13. **Agent Browser 사용:** 페이지 확인 수단을 Agent Browser라고 정확히 표시한다. PASS.
14. **실제 확인 경로:** roadmap, status, plan, Android/iOS Simulator PNG 경로가 모두 Browser verification record에 있다. PASS.
15. **HTML 상태 과장:** browser 확인을 앱 runtime 테스트로 바꾸어 말하지 않는다. PASS.
16. **C11.4 체크:** 주변 문장의 C11.4 미완료 상태를 그대로 보존한다. PASS.
17. **공식 완료 수:** `72/213`을 유지하고 변경으로 늘리지 않는다. PASS.
18. **PR #453 merge 사실:** 주변 계획·roadmap의 PR #453 결과와 새 pass38 기록이 충돌하지 않는다. PASS.
19. **기기 범위:** Android/iOS는 Simulator screen route로만 지칭하며 Android 실기기 결과를 주장하지 않는다. PASS.
20. **WPT 범위:** 이 status 문구가 WPT 통과를 의미하지 않는다. PASS.
21. **iOS 최종 pixel 범위:** iOS compositor 이후 화면 검증으로 확대하지 않는다. PASS.
22. **렌더 기대값:** `70×20`/`100×10` scene 값은 이 status 변경 문장에서 별도로 새 주장하지 않는다. PASS.
23. **체크리스트 노출:** parent C11.4 `[ ]` checkbox는 변경 대상이 아니며 diff에서 손대지 않았다. PASS.
24. **API/계약:** 공개 API와 내부 계약의 version/status를 바꾸지 않는다. PASS.
25. **시간 순서:** pass38 main SHA는 PR #456의 완료 기록보다 앞선 source이며, 문장이 pass38을 과거 결과로 기록한다. PASS.
26. **현재 main 해석:** PR merge 이후 pass38 source commit이 현행 main인 척 쓰지 않는다. 정확한 당시 SHA/tree를 넣는다. PASS.
27. **증거 경로:** status 문장에서 `internal/evidence/.../README.md`를 연결하고 새 README에 pass38 build·sync·browser 관찰을 자체 기록한다. PASS.
28. **Simulator route:** 새 README는 기존 tracked Android/iOS Simulator PNG와 runtime README에 연결되며 이번 preview 캡처를 앱 화면 증거처럼 넣지 않는다. PASS.
29. **주변 문서 링크:** 기존 계획·사전 비교·runtime README 링크와 status 기록을 삭제하지 않았다. PASS.
30. **다음 selected build:** PR merge 뒤 새 README가 선택 root의 route allowlist에 포함되어 Tailnet에서 렌더링될 것이다. 이번 status 문구는 pass38 당시 결과만 기록한다. PASS.

판정: 완료 기록은 pass38 당시 source와 원격 검증 수치에 한정되며 공식 C11.4 상태는 바꾸지 않는다. 차단 결함 없음.
