# C11.1 Root HTML `grid-auto-rows: fr` Tailnet 로드맵 동기화 · 2026-11-03

## 공개된 상태

구현 PR [#1070](https://github.com/ohah/spinon/pull/1070)은 2026-11-02T16:37:30Z Rebase and merge됐고 merge SHA는 `f40cb3b93d2555dafecd352bd35fb349b6bf7a64`다. `spec/public/roadmap.html`, `spec/STATUS.md`, 계획과 내부 계약을 그 병합 상태에 맞췄다. C11.1/C11은 미완료로 남고 공식 완료 수는 `72/213`이다. 이번 Tailnet 갱신은 비공개 개발 미리보기만 대상으로 한다.

## 선택 publication과 동기화

`spec/public/roadmap.html`의 PR #1070 항목, 계획, 계약 0059, 구현 precomparison/runtime 근거와 링크 대상을 Rspress 2.0.22의 선택 root로 빌드했다. publication source HEAD는 `101936f7314f7027b3883ce4a49732b150111530`이고 base `origin/main`은 구현 PR merge commit `f40cb3b93d2555dafecd352bd35fb349b6bf7a64`였다. 전체 문서 사이트 빌드나 모든 링크 검증은 아니다. 선택 publication은 65개 route, 6,239,289 bytes이며 각 source·size·SHA-256은 [manifest](./selected-publication.json)에 기록했다. 새 카드의 10개 local link는 publication에 모두 포함됐다.

Tailscale backend는 `Running`이었다. 이미 실행 중인 preview server PID `28040` (`127.0.0.1:8768`)를 재시작하지 않았다. preflight는 local HTTP 200, 65개 route의 source hash, 교체 대상 열린 파일 0개를 확인했다. 복사 결과는 48개 생성, 5개 교체, 12개 동일이다. 교체 전 파일 5개는 `~/Library/Application Support/SpinonPreviewBackups/roadmap-c11-grid-auto-row-fr-tailnet-sync-2026-11-03/pass01`에 hash를 검증해 보존했다. route 삭제는 없었다.

복사 후 local preview hash 65/65, Tailnet HTTPS HTTP 200 및 hash 65/65, backup hash 5/5를 확인했다. 실행 원본: [preflight](./preflight.json), [copy/backup](./copy-and-backup.json), [served verification](./served-verification.json). 동기화 실행기는 symlink와 열린 route를 거부하고 기존 파일을 backup한 뒤 atomic rename하며, 선택하지 않은 파일은 삭제하지 않는다.

첫 publication helper 실행은 이미 생성된 선택 Rspress `site` 폴더를 새 publication 경로와 함께 존재 거부 목록에 넣어 중단됐다. guard를 publication/manifest만 새 경로인지 확인하도록 수정했다. 이 실패는 Rspress 산출물이나 Tailnet 파일을 바꾸지 않았고, 수정 후 publication과 sync가 통과했다.

## Agent Browser 시각 확인

[데스크톱 캡처](./tailnet-roadmap-agent-browser-desktop.png) 1280×900, [모바일 캡처](./tailnet-roadmap-agent-browser-mobile.png) 390×844를 저장했다. 두 폭에서 문서 가로 크기가 viewport와 같았고 새 카드도 화면 폭 안에 들어왔다. 카드의 PR #1070, `72/213`, 넓은 test 36/37 제한 및 화면 근거를 확인했다. 카드의 계획 링크를 열어 계획 heading이 렌더링되는 것도 확인했다. [기계 판독 결과](./agent-browser-verification.json)에 viewport·card bounds를 기록했다.

Agent Browser `close --all` 뒤 session 목록이 비었고 브라우저 CLI process scan도 0개였다. 기존 Tailnet preview server는 사용자가 쓰는 서비스이므로 그대로 두었다.

문서·선택 publication·Tailnet 동기화 변경은 [PR diff 실패 관점 30개 검토](./roadmap-pr-diff-review-30.md)로 대조했다.

## 범위와 한계

Simulator 화면은 visual smoke capture이며 PixelCopy/Metal readback 근거가 아니다. Android renderer는 ANGLE/SwiftShader software path다. 이 로드맵 갱신은 WPT 적합성, 실기기·hardware GPU, 전체 CSS Grid 또는 C11 완료를 뜻하지 않는다. PR #1070의 GitHub check rollup은 비어 있었고 main required status checks도 설정되지 않았다.
