# C13.3 native wheel·trackpad 계획 PR 적대적 검토 20개 · 2026-10-16

PR diff, 근거 링크, 상태 연결과 pass/negative criteria를 별도 20개 관점으로 검토했다. 계획 실패 검토 표를 재사용하지 않는다.

1. **C13.3 완료로 잘못 표시되는가?** — STATUS unchecked와 미완료를 유지한다. 문제 없음.
2. **versioned interface가 바뀌는가?** — 내부 계약 0063은 링크만 두고 변경하지 않는다. 문제 없음.
3. **plan→evidence 링크가 깨졌는가?** — 첫 링크 검사는 두 sibling evidence 파일의 상위 경로를 하나 더 오르는 오류를 찾았다. 두 경로를 고쳐 재검사했다. 수정 완료.
4. **status에서 계획을 찾을 수 있는가?** — wheel 후속 문단에 새 plan link를 추가했다. 문제 없음.
5. **Android action/source가 모호한가?** — ACTION_SCROLL와 mouse/mouse-equivalent touchpad로 한정한다. 문제 없음.
6. **Android factor API가 minSdk와 충돌하는가?** — minSdk 29, factor API 26+다. 문제 없음.
7. **Android scale이 중복 적용되는가?** — axis factor physical px와 surface px/CSS px를 분리한다. 문제 없음.
8. **H/V 부호가 잘못 연결됐는가?** — API viewport sign과 CSS right-handed sign을 축별 식으로 기록한다. 문제 없음.
9. **API34 alternate path가 묵시 지원되는가?** — classified move/gesture axes는 미지원으로 쓴다. 문제 없음.
10. **ADB injection이 physical proof로 읽히는가?** — emulator/synthetic limitation을 적는다. 문제 없음.
11. **iOS source를 추측하는가?** — discrete와 continuous mask recognizer를 분리한다. 문제 없음.
12. **iOS finger pan 중복이 가능한가?** — empty allowedTouchTypes guard가 있다. 문제 없음.
13. **iOS translation이 cumulative임을 표시했는가?** — Apple reference와 차분 규칙이 있다. 문제 없음.
14. **iOS sign이 근거 없이 확정됐는가?** — proposed formula와 preflight gate가 있다. 문제 없음.
15. **momentum lifecycle를 발명하는가?** — phase 부재 시 None이다. 문제 없음.
16. **zoom/modifier가 queue에 새는가?** — Android Ctrl, iOS Control, pinch, unknown input negative controls가 있다. 문제 없음.
17. **target이 worker 시점에 재계산되는가?** — enqueue에서 고정하고 stale을 reject한다. 문제 없음.
18. **capacity rejection 뒤 delta가 replay되는가?** — last-observed baseline과 no-replay 규칙이 있다. 문제 없음.
19. **plan PR에 동작 screenshot이 없는가?** — 동작을 바꾸지 않는 plan-only diff라 후속 implementation PR에서 양 플랫폼 캡처한다. 타당함.
20. **Tailnet preview가 Pages 배포로 확대되는가?** — merge 뒤 지정된 Tailnet root만 Agent Browser로 확인하고 Pages/gh-pages를 건드리지 않는다. 문제 없음.

## 결과

계획 PR 차원 merge blocker는 찾지 못했다. 양 플랫폼 native-input preflight는 implementation gate다. 이 문서는 native input support를 증명하지 않는다.
