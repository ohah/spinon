# PR #1087 병합 후 Tailnet 로드맵 갱신 · 2026-11-03

## 갱신 이유

PR [#1087](https://github.com/ohah/spinon/pull/1087)이 병합되어 local **main**은 **52b58b6bd7fde6073e1c5992e0c93a815a53a684**다. 이 병합은 공식 상태 ID와 제목을 함께 사용해 반복 ID·알파벳 접미 ID를 포함한 로드맵 표시를 원본 **spec/STATUS.md**와 맞춘다. 이어서 PR [#1086](https://github.com/ohah/spinon/pull/1086)의 제한 **minmax()** 설명을 Tailnet 화면에 게시해야 했다.

병합 전 Agent Browser에서 Tailnet 로드맵은 PR #1082까지의 C11.4 기록을 보여 줬고 #1086 카드는 아직 없었다. 로컬 **main**의 새 상태표 집계는 215개 표시 행, 완료 78개다. C11 전체와 C11.1은 이 동기화에서 완료로 표시하지 않는다.

## 선택 발행

가장 최근 Tailnet 기준선은 PR #1084 동기화의 선택 root **build/spinon/roadmap-tailnet-sync-c11-4-full-size-kana-2026-11-03/pass01/root**이며 source HEAD는 **c82639bd1**이다. 이 기준선에 현재 **main**의 추적 문서 변경 48개를 overlay했다. 상태 파일과 독립 roadmap 경로 alias를 포함해 90개 source→route overlay, 선택 root 223개 파일을 만들었다. 전체 RSPress 빌드를 수행하지 않고 RSPress 2.0.22 선택 빌드만 실행했다. 선택 빌드는 dead-link·dead-image 검사를 끈 설정이며 전 저장소 링크 검증을 뜻하지 않는다.

선택 publication은 170개 route, 17,757,023 bytes다. Tailnet preflight에서 Tailscale backend **Running**, 기존 preview server PID **28040**, 변경 route 열린 파일 0개를 확인했다. 기존 preview 서버는 재시작하지 않았다. 복사 결과는 139개 생성, 19개 교체, 12개 동일이며 삭제는 없다. 교체 파일 19개를 새 백업 경로에 보존하고 각 백업 SHA-256을 확인했다. RSPress 로그의 정렬용 후행 공백만 제거했으며 명령 출력 내용과 종료 코드는 유지했다.

- 선택 root: [selected-root.json](./selected-root.json)
- 선택 route와 hash manifest: [selected-routes.json](./selected-routes.json)
- RSPress 선택 빌드 로그: [selected-build.log](./selected-build.log)
- 사전 검사: [preflight.json](./preflight.json)
- 복사·백업: [copy-and-backup.json](./copy-and-backup.json)
- 제공 route 검증: [served-verification.json](./served-verification.json)
- 동기화 요약: [tailnet-sync-report.json](./tailnet-sync-report.json)
- 게시 스크립트: [prepare-selected-root.mjs](./prepare-selected-root.mjs), [rspress.config.ts](./rspress.config.ts), [stage-publication.mjs](./stage-publication.mjs), [sync-tailnet.mjs](./sync-tailnet.mjs)

최종 검증에서 선택 route hash 170/170, 필수 local/Tailnet HTTP route 17/17이 publication manifest와 일치했다. 기존 route의 사전 내용은 **/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1087-2026-11-03/pass01**에 보존했다.

## Agent Browser 확인

터미널 브라우저가 아닌 Agent Browser로 실제 Tailnet HTTPS URL을 열었다. 화면의 집계는 **215개 표시 · 공식 완료 78/215**, checkbox 215개 중 checked 78개, input ID 중복 0개였다. C11.1 **minmax()** 카드에는 GitHub PR·계획·실행 근거 링크가 각각 연결되어 있었다. 계획 route를 직접 열어 올바른 H1과 이미지 1개 로드를 확인했다.

데스크톱 viewport 1440×1000과 모바일 390×844에서 **documentWidth == viewportWidth**, **bodyWidth == viewportWidth**였다. 모바일 카드 폭은 362px로 viewport 안에 들어왔다. 화면과 기계 판독 결과는 [desktop](./tailnet-roadmap-agent-browser-desktop.png), [mobile](./tailnet-roadmap-agent-browser-mobile.png), [browser-verification.json](./browser-verification.json)에 있다.

Agent Browser **close --all** 뒤 프로세스 scan에 Agent Browser 프로세스가 남지 않았다. Tailnet preview server PID 28040은 계속 실행 중이다.

모든 evidence 파일의 SHA-256은 [SHA256SUMS.txt](./SHA256SUMS.txt)에 있다.

## 한계와 상태

- PR #1086은 Root HTML 기본 경로의 제한 **minmax()** slice다. 이 갱신은 CSS Grid 전체 지원이나 C11/C11.1 완료를 주장하지 않는다.
- 앱 실행, Android/iOS simulator, PixelCopy/Metal, 기기 또는 GPU 검증을 이 문서 발행으로 확대 해석하지 않는다.
- GitHub Pages나 공식 문서 사이트 배포는 수행하지 않았다. Tailnet 개발 미리보기의 선택 route만 갱신했다.

이 동기화 변경의 별도 실패 관점 30개 대조는 [PR diff review](./pr-diff-review-30.md)에 기록했다.
