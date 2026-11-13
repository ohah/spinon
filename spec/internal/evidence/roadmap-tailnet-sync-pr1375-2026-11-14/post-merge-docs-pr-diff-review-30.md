# PR #1375 병합 후 상태·로드맵 문서 PR diff 검토 30개

기준은 PR #1375 병합 커밋 `22161976cab0bfd68a1d545eeb616441a0ab7a0e`다. 검토 대상은 상태 대장, 공개 로드맵, C11.4 계획·내부 계약·실행 근거 및 선택 Tailnet 미리보기 증거다. 각 항목은 별도의 실패 경로 또는 상태 경계를 확인한다.

1. 상태 대장 기준일이 실제 문서 변경일과 맞는지 확인한다 — 2026-11-14로 일치.
2. 상위 C11.4 완료 checkbox가 미완료인지 확인한다 — `- [ ]` 유지.
3. 상위 C11 완료 checkbox가 미완료인지 확인한다 — `- [ ]` 유지.
4. 공식 완료 집계가 `78/214` 그대로인지 확인한다 — 변경 전후 동일.
5. 변경만으로 내부 숫자 계약이 `0.1.0`에서 올라가지 않는지 확인한다 — `0.1.0` 유지.
6. PR #1375 링크와 제목이 실제 병합된 구현 PR을 가리키는지 확인한다 — PR URL·제목이 일치.
7. PR source head가 병합 근거와 일치하는지 확인한다 — `63e508c1231f4c5212bd048584caac0da5c70f4f` 일치.
8. PR merge SHA가 `main`의 현재 HEAD와 일치하는지 확인한다 — `22161976cab0bfd68a1d545eeb616441a0ab7a0e` 일치.
9. PR merge 시각과 병합 방식이 기록과 일치하는지 확인한다 — UTC 시각과 Rebase and merge 일치.
10. 0개의 check/workflow를 CI 성공으로 오해하지 않도록 쓰였는지 확인한다 — CI green 아님을 명시.
11. Chrome 사전 oracle 138,240 scenes와 RuntimeSession 실행 수가 구별되는지 확인한다 — 별도 표기.
12. mixed RuntimeSession 3,840/3,840 표본이 전체 oracle 행렬처럼 표현되지 않는지 확인한다 — “표본” 및 사전 oracle 분리.
13. uniform endpoint 11,520/11,520과 mixed 결과가 분리됐는지 확인한다 — 별도 결과로 표기.
14. auto-placement 5,760/5,760이 row-span mixed profile의 성능·동작 증거로 섞이지 않는지 확인한다 — regression으로만 표기.
15. Android AVD를 실기기나 hardware GPU라고 부르지 않는지 확인한다 — ARM64 공유 snapshot AVD로 한정.
16. iOS Simulator startup smoke를 물리 iPhone 검증이라고 부르지 않는지 확인한다 — Simulator로 구분.
17. light/dark startup smoke와 mixed feature app runtime이 구분되는지 확인한다 — mixed 앱 미검증을 명시.
18. mixed feature의 surface-pixel parity를 검증했다고 주장하지 않는지 확인한다 — 미검증을 명시.
19. checked-in merge 기록 링크가 실제 Markdown route와 일치하는지 확인한다 — source·RSPress route hash로 확인.
20. 공개 로드맵 상단의 최신 병합 카드가 #1375를 가리키는지 확인한다 — Agent Browser 본문에서 확인.
21. 공개 로드맵 최신 카드의 merge SHA가 #1375의 SHA인지 확인한다 — Agent Browser에서 일치 확인.
22. 지난 #1372 기록이 최신 카드 갱신으로 사라지지 않고 이력에 남는지 확인한다 — Agent Browser 본문에 존재.
23. 공개 로드맵의 진행 중 칸이 선택되지 않은 slice를 진행 중으로 표시하지 않는지 확인한다 — `0/2`, 현재 진행 없음.
24. 다음 bounded candidate가 미선정이라는 상태를 보존하는지 확인한다 — 칸반 카드와 최신 요약에 표시.
25. `spec/STATUS.md`가 구현 완료 수의 공식 원본임을 유지하는지 확인한다 — 원본 역할 설명 유지.
26. 공개 로드맵의 동적 칸반 집계가 공식 상태 체크를 읽는 계약을 보존하는지 확인한다 — 기존 fetch/집계 코드를 변경하지 않음.
27. 로드맵의 선택된 내부 링크가 선택 publication에 포함되는지 확인한다 — 5개 현재 카드 route 전부 manifest에 있음.
28. RSPress 렌더링 뒤 `.html` route가 링크 목적지와 맞는지 확인한다 — 130 route 제공본 해시 검증.
29. desktop/mobile 레이아웃에서 긴 merge 카드가 잘리거나 상호작용을 막지 않는지 확인한다 — 1440×900·390×844 확인, 문서 가로 overflow 없음; 칸반은 끝까지 가로 스크롤 가능.
30. 이번 PR이 GitHub Pages 자동 배포나 전체 Tailnet 사이트 동기화로 확대되지 않았는지 확인한다 — 선택 130 route만 갱신, 기존 root·서버·무관 route 유지.

검토 중 발견한 불일치는 수정하고 해당 항목을 다시 확인한다. 최종 결과와 route/hash는 이 폴더의 동기화 manifest와 Agent Browser 캡처에 기록한다.
