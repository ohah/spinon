# C11.4 `line-break` 상태·Tailnet 동기화 PR diff 검토 (30 관점)

검토 범위는 `spec/STATUS.md`, C11/C11.4 계획 문서, `spec/public/roadmap.html`, 선택 문서 빌드·동기화 helper와 이 폴더의 게시 evidence다. 구현 변경을 다시 세지 않고, 병합 뒤 문서·게시 변경을 별도 실패 경로로 확인했다.

1. **Merge identity:** PR #945 merge SHA와 시각이 `gh pr view` 결과와 일치한다. 통과.
2. **C11 parent state:** C11 parent checkbox가 unchecked로 유지된다. 통과.
3. **C11.4 state:** intrinsic track sizing checkbox가 unchecked로 유지된다. 통과.
4. **Global count:** 전체 공식 집계 `72/213`을 변경하지 않았다. 통과.
5. **Scope wording:** 다섯 고정 scene만 제한 opt-in slice로 설명한다. 통과.
6. **Check attribution:** PR에 check run이 보고되지 않은 사실을 CI 통과로 표현하지 않는다. 통과.
7. **Platform attribution:** Android API 37 AVD와 iOS 26.2 Simulator 결과를 실기기 결과로 바꾸지 않는다. 통과.
8. **Renderer limits:** SwiftShader/Simulator 결과에서 Android hardware GPU를 추론하지 않는다. 통과.
9. **Text paint limits:** route screenshot을 glyph paint 또는 compositor 이후 pixel 근거로 쓰지 않는다. 통과.
10. **Conformance limits:** WPT 또는 전체 CSS Grid 적합성으로 범위를 넓히지 않는다. 통과.
11. **Language ownership:** `lang=ja|en`은 exact Grid style owner에서 읽는 제한 경계로 유지한다. 통과.
12. **Computed-value default:** 기존 `line-break:auto` 기본 동작을 변경한 것으로 서술하지 않는다. 통과.
13. **Candidate limitations:** 일반 CSS admission과 언어 inheritance를 완료로 주장하지 않는다. 통과.
14. **Child plan state:** 개별 구현 계획에 PR #945와 5/5 결과가 반영됐다. 통과.
15. **C11.4 parent summary:** `plan/c11-4-grid-track-sizing.md`가 최신 merged slice를 표시한다. 통과.
16. **C11 parent summary:** `plan/c11-css-grid.md` 표와 상태 문구가 같은 결과를 반영한다. 통과.
17. **Official ledger placement:** STATUS의 실행 근거가 unchecked C11.4 항목 아래 추가됐다. 통과.
18. **Public chronology:** roadmap 카드가 2026-11-01 최신 C11.4 항목으로 들어갔다. 통과.
19. **Public wording:** roadmap 카드가 C11.4/C11 미완료와 `72/213`을 명시한다. 통과.
20. **Roadmap destinations:** 계획, 계약, 사전 비교와 runtime evidence 링크가 실제 route를 가리킨다. 통과.
21. **Selected source scope:** RSPress 선택 root는 C11.4에 연결된 문서와 evidence만 포함한다. 통과.
22. **Toolchain stability:** 문서 빌드에 고정된 RSPress `2.0.22`를 사용했고 dependency version은 바꾸지 않았다. 통과.
23. **Dead-link claim:** 선택 빌드 설정에서 전체 dead-link 검사를 끈 사실과 전체 검사 미실행을 명시한다. 통과.
24. **Evidence mirrors:** `internal/` 및 `spec/internal/`의 두 link route가 필요한 곳에 동일 source bytes를 사용한다. 통과.
25. **Build-helper isolation:** 선택 root는 새 고유 경로로 만들며 이전 build/evidence를 삭제하거나 덮어쓰지 않는다. 통과.
26. **Route safety:** helper는 절대 경로, `..`, symlink source 및 route 충돌을 거부한다. 통과.
27. **Publication manifest:** 110개 route의 바이트 수·SHA-256·필수 route를 고정했다. 통과.
28. **Tailnet preflight:** 서버 PID/root/port와 기존 local/remote roadmap hash를 동기화 전에 확인했다. 통과.
29. **Copy safety:** 86개 create, 9개 replace, 15개 same으로 분류했고 열린 22개 경로는 모두 동일 hash였다. 이전 교체 파일을 백업했으며 삭제는 없다. 통과.
30. **Live verification and cleanup:** Agent Browser에서 desktop/mobile 및 네 연결 페이지를 확인하고 가로 넘침 0px을 확인했다. 세션과 browser process를 닫았고 preview 서버는 그대로 실행 중이다. 통과.

30개 관점에서 문서·route·복사·범위 claim의 차단 결함을 찾지 못했다. 전체 docs dead-link 검사, 전체 CSS/WPT 적합성 및 C11 완료를 의미하지 않는다.
