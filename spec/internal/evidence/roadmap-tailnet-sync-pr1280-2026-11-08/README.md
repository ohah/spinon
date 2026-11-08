# C11.4 PR #1280 병합 상태와 Tailnet 로드맵 동기화

## 발견한 상태 차이

Agent Browser로 확인한 Tailnet 페이지에는 `진행 중 1/2`와 `C11.4 · 진행 중` 카드가 이미 보였다. 다만 상세 문단은 이전 제공본이라 Layout `11/11`과 PR diff 검토 대기를 표시했다. 현재 코드 근거는 Layout `12/12`, PR 적대 검토 `30/30`이며 PR [#1280](https://github.com/ohah/spinon/pull/1280)은 Rebase and merge 됐다. 저장소 원본과 제공본의 SHA-256이 달라 제공 파일이 오래된 것이 직접 원인이었다.

공식 완료 `78/214`는 그대로다. 이 C11.4 세부 slice에는 별도 상위 ID가 없고 상위 C11.4/C11 전체는 아직 끝나지 않았다. 완료 수와 진행 중 세부 작업은 서로 다른 지표다.

## 수정 범위

- 공식 상태 대장과 계획에 PR #1280의 UTC 병합 시각과 merge SHA를 기록한다.
- 공개 로드맵의 진행 카드는 PR #1280 병합 상태를 가리키고, C11.4/C11 미완료 및 `78/214`를 유지한다.
- GitHub workflow/status 결과가 비어 있으므로 CI green으로 적지 않는다.
- Tailnet 제공본은 선택한 RSPress route를 다시 빌드해 갱신한다. 게시 manifest는 276개 파일을 대상으로 추가 166, 동일 90, 백업 후 원자 교체 20개를 기록한다. 공개 snapshot tarball은 0개다. 경로를 삭제하거나 preview server를 재시작하지 않는다.

## 검증 근거

- [`before-agent-browser.json`](./before-agent-browser.json)과 [`before-tailnet-desktop.png`](./before-tailnet-desktop.png)는 오래된 제공본의 화면 및 텍스트를 보존한다.
- [`stage-and-build.mjs`](./stage-and-build.mjs), [`rspress.config.ts`](./rspress.config.ts), [`selected-rspress-build-pass04.log`](./selected-rspress-build-pass04.log)와 [`staging-manifest-pass04.json`](./staging-manifest-pass04.json)은 선택 route 입력과 RSPress 2.0.22 build를 기록한다. RSPress 출력표의 terminal padding과 EOF 공백은 diff check를 위해 정리했다. 초기 staging guard의 두 정적 marker 가정 실패와 config import 실패, 제공 파일을 바꾸지 않았다는 판정은 [`staging-attempts.json`](./staging-attempts.json) 및 [`selected-rspress-build-pass03.log`](./selected-rspress-build-pass03.log)에 기록했다.
- [`publication-preflight-pass04.json`](./publication-preflight-pass04.json), 게시 manifest·결과와 [`served-verification-pass04.json`](./served-verification-pass04.json)은 source/served SHA-256, 백업, open-handle 검사, server PID와 Tailscale 상태를 기록한다. 10개 중요 route가 HTTP 200과 게시 SHA 일치를 보였다.
- [Agent Browser 데스크톱 결과](./after-agent-browser-desktop.json)와 [모바일 결과](./after-agent-browser-mobile.json), [데스크톱 화면](./after-tailnet-desktop.png) 및 [모바일 화면](./after-tailnet-mobile-viewport.png)은 `C11.4 · 진행 중`, 병합된 PR #1280, 최신 검증 수치, 문서 폭 및 현재 카드 링크를 확인한다. 모바일 390×844에서 문서 폭은 390px이고 진행 카드가 보인다. 전체 사이트 dead-link 검사는 하지 않는다.
- PR #1281 본문에 데스크톱·모바일 캡처를 넣었고 Agent Browser에서 두 이미지가 각각 1440×1000, 390×844로 정상 로드됨을 확인했다. 확인에 쓴 두 Agent Browser session을 모두 닫았다. [`agent-browser-lifecycle.json`](./agent-browser-lifecycle.json)에 종료 상태를 기록한다.
- [문서 변경 PR의 적대 검토 30개](./roadmap-pr-diff-review-30.md)는 공식 상태·계산 집계·제공 경로·화면 반응형을 서로 다른 실패 관점으로 확인한다.

PR #1280의 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 화면은 OS screenshot visual smoke다. PixelCopy/Metal surface readback, 실기기 및 전체 C11.4/C11 적합성을 뜻하지 않는다.
