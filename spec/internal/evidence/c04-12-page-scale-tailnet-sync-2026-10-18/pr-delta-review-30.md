# PR 변경 실패 관점 검토 30개

1. **공식 완료 상위 ID** — C04.12 상위 체크 상태를 유지하고 로드맵 설명도 미완료로 표시하는지 확인한다.
2. **하위 체크와 상위 집계** — 선택한 page-scale 하위 결과를 C04.12 전체 완료로 확대하지 않는지 확인한다.
3. **완료 수 원본** — 완료 수를 새 숫자로 하드코딩하지 않고 배포된 `STATUS.md`에서 계산하도록 둔다.
4. **로드맵 배포 파일** — 사이트가 복사해 쓰는 `spec/public/roadmap.html`에 변경이 실제 반영됐는지 확인한다.
5. **기존 note 교정** — page scale이 snapshot에만 있다고 쓰던 오래된 문장을 PR #279 이후 상태로 교정했는지 확인한다.
6. **배율 입력** — renderer 변환이 DPR과 초기 `page_scale`을 곱한다는 표현이 구현 계약과 일치하는지 확인한다.
7. **표현 대상** — box·border·image·pre-rasterized glyph로 한정하고 일반 DOM 전체를 지원한다고 쓰지 않는지 확인한다.
8. **좌표 경계** — CSS layout/hit-test 좌표 유지와 physical surface 변환을 혼동하지 않는지 확인한다.
9. **clip 의미** — scale된 scene이 physical target 경계에서 잘리는 설명이 CSS overflow 전면 지원처럼 보이지 않는지 확인한다.
10. **기본 runtime 연결** — opt-in snapshot mapping이 기본 `RuntimeSession`에 자동 연결된다고 오해하게 하지 않는지 확인한다.
11. **Chrome oracle** — Chrome 155 iPhone 12 emulation이 기준 입력이며 Spinon 출력 캡처가 아님을 구분한다.
12. **viewport case 수** — scale 0.5, scale 2, width 500의 세 case를 모두 적었는지 확인한다.
13. **WGPU 표본 수** — 두 앱 플랫폼 각각 3 case와 30 surface sample로 설명했는지 확인한다.
14. **별도 readback** — CSS clip·border·opacity group 검증이 simulator 화면 캡처와 구별되는지 확인한다.
15. **Android 환경** — API 37 AVD로 표기하고 실기기라고 확대하지 않는지 확인한다.
16. **iOS 환경** — iPhone 17 Pro/iOS 26.2 Simulator를 정확히 기재했는지 확인한다.
17. **화면의 증명력** — Simulator 캡처를 일반 HTML body paint로 표현하지 않는지 확인한다.
18. **실기기 경계** — Android/iOS physical-device 검증을 했다고 주장하지 않는지 확인한다.
19. **계획 링크 경로** — 새 page-scale plan 링크가 `/spinon/plan/c04-12-page-scale-rendering.html`에서 열리도록 출력됐는지 확인한다.
20. **실행 근거 링크 경로** — page-scale README 링크가 정확한 evidence route로 제공되는지 확인한다.
21. **내부 인덱스** — internal README 변경이 해당 sync evidence를 찾을 수 있게 연결하는지 확인한다.
22. **기존 C04.12 링크** — root-document, color-scheme, visual viewport 자료에 대한 기존 링크를 깨뜨리지 않는지 확인한다.
23. **Rspress 범위** — 임시 selected root로 관련 문서만 렌더하고 전체 docs build 성공으로 보고하지 않는지 확인한다.
24. **빌드 버전** — Rspress `2.0.22` 고정 버전을 바꾸지 않는지 확인한다.
25. **Dead link 판단** — 선택 build의 dead-link 검사를 비활성화한 한계를 숨기지 않는지 확인한다.
26. **복사 범위** — roadmap, 관련 rendered pages, 필요한 static assets 및 source evidence만 대상으로 삼는지 확인한다.
27. **삭제 통제** — Tailnet 미리보기에서 기존 파일을 삭제하지 않는지 확인한다.
28. **교체 백업** — 변경할 기존 대상별 백업을 복사 전에 만들고 해시를 대조하는지 확인한다.
29. **listener 수명** — PID 1191 listener를 재시작하지 않고 local/Tailnet 제공 hash만 검증하는지 확인한다.
30. **사용자 환경** — tmux, terminal-browser, physical device에 접근하거나 상태를 바꾸지 않는지 확인한다.
