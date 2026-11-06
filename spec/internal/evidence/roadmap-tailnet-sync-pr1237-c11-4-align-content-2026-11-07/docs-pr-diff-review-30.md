# PR #1237 post-merge docs와 Tailnet diff 검토

이후 docs PR의 상태·계약·공개 roadmap·Tailnet 변경을 서로 다른 30개 실패 경로로 검토했다.

1. **Merge SHA 오기:** STATUS, contract, plan, public roadmap, Tailnet에 `fddf66295a1f66134a061c0c221410964cb3077a`를 대조했다.
2. **Merge 시각대 혼동:** GitHub의 `2026-11-06T20:33:30Z`를 KST 날짜 2026-11-07 기록과 맞췄다.
3. **PR 상태 불일치:** 상태 문구가 #1237 merged 및 Rebase and merge로 일치한다.
4. **계획 PR 참조 손실:** plan에 선행 계획 PR #1236을 남겼다.
5. **C11 완료 오기:** C11 parent와 C11.4는 미완료로 유지한다.
6. **공식 count 증가:** `78/214`를 STATUS·public roadmap·Tailnet에서 대조했다.
7. **내부 버전 상승:** contract 숫자 버전 `0.1.0`을 유지한다.
8. **지원값 누락:** 계약과 공개 roadmap은 computed allowlist 10개 범위를 연결한다.
9. **기존 stretch semantics 손실:** `normal|stretch`는 기존 stretch 동작으로 명시한다.
10. **새 values 과대 범위:** 8개 위치·분배 값은 fixed 3-item shape에 한정한다.
11. **Track sizing 의미 혼합:** 새 values는 `[10,10,10]px` base를 보존한다고 기록한다.
12. **Generic profile 오염:** legacy intrinsic track profile의 거부 경계를 contract에서 유지한다.
13. **Shorthand 축 누출:** `justify-content`가 새 값으로 넓어지지 않는다는 거부 경계를 명시한다.
14. **정확한 근거 연결 실패:** Chrome precomparison, Rust tests, runtime evidence 링크를 검사했다.
15. **Android/iOS 역할 혼동:** API 37 AVD와 iPhone Simulator를 분리 기록한다.
16. **Screenshot 과대 주장:** simulator와 Tailnet 화면은 visual smoke로 한정한다.
17. **Broad test green 오기:** base/candidate suite 실패 수와 동일 failure names를 유지한다.
18. **CI 상태 오기:** workflow/check/status가 비어 있어 CI green 주장을 하지 않는다.
19. **Plan stale wording:** `candidate`에서 실제 merged PR 상태로 갱신했다.
20. **Contract stale gate:** validator가 열 개 computed values를 받고 shape 외 입력을 거부하도록 설명을 고쳤다.
21. **Roadmap 카드 순서:** 새 C11.4 카드가 #1220/#1216보다 앞에 있다.
22. **Tailnet target 혼동:** server mapping이 실제 제공하는 `/SpinonPreview/roadmap.html`만 교체했다.
23. **다른 route 손상:** sync JSON은 파일 변경 경로가 `/spinon/roadmap.html` 하나임을 기록한다.
24. **기존 preview 덮어쓰기:** 이전 제공 bytes를 새 backup 경로에 보존했다.
25. **백업 불일치:** backup byte count와 SHA가 교체 전 제공본과 같다.
26. **부분 파일 제공:** 같은 디렉터리 temp file + atomic rename으로 교체했다.
27. **서버 중단:** sync 전·후 PID 28040과 Tailscale Running을 확인하고 process를 재시작하지 않았다.
28. **local response mismatch:** local HTTP 200 body hash가 source와 같다.
29. **Tailnet response mismatch:** HTTPS 200 body hash와 merge/count markers가 source와 같다.
30. **Responsive/browser lifecycle 오류:** desktop/mobile에서 최신 카드·width·overflow를 확인한 뒤 Agent Browser를 닫고 서버는 계속 실행 중임을 확인했다.

검토에서 발견한 문서 결함은 contract의 recovery 설명이 실제 유효 회복값 `stretch` 대신 `center`라고 적은 점이었다. source test를 다시 확인하고 `stretch`로 고친 뒤 plan/contract/STATUS/roadmap 간 범위를 재대조했다. 30개 항목 검토 후 추가 수정 사항은 없었다.
