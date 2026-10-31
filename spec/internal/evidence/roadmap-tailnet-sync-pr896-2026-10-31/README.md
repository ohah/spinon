# PR #895/#896 병합 뒤 Tailnet 로드맵 동기화 · 2026-10-31

## 기준과 build

기준은 PR #896 병합 뒤 `main` `2e8ab8cf5211635920430d69ff4ce34dd09f5ca4` (tree `08c94be8181a778a8d4c041462d566bbb10e0747`)다. PR #895는 공개 로드맵에 PR #893 기록을 추가했고 `f317c9016c8dfb9ebae8c5030a29fc1d4c821672`로 병합됐다. 선택된 첫 RSPress build는 실행 README의 두 PNG 링크에 `./`가 없어 asset import 단계에서 실패했다. 해당 문서 문제는 PR #896으로 수정·병합됐다. 실패 log는 [수정 전 build](./rspress-build-before-image-link-fix-failed.log)에 보존했다.

수정된 main source에서 tracked Git blob만 골라 2,100개 파일, 45,553,798 bytes의 root를 구성했다. RSPress `2.0.22` build는 pass01에서 성공했고 publication pass02는 2,257 routes, 59,823,129 bytes다. 이 단계는 전체 문서 링크 검사를 수행하지 않는다. pass01 publication 검증은 temporary required-route 목록의 파일명을 잘못 적어 멈췄으며 Tailnet에는 쓰지 않았다. 정확한 tracked route `status-sync-pr-diff-review-30.html`로 고친 pass02 검증이 통과했다. [source manifest](./selected-root.json) · [build log](./rspress-build-pass01.log) · [통과 publication manifest](./selected-publication-pass02.json) · [pass01 guard 실패](./selected-publication-pass01.log).

## Tailnet 반영과 bytes 검증

- 기존 preview server PID `28040`의 단일 loopback listener `127.0.0.1:8768`을 유지했다.
- `/Users/yoonhb/Library/Application Support/SpinonPreview`에서 선택한 2,257 routes를 비교했다. create 1,455, replace 67, same 735, delete 0이다. 교체 전 9,185,083 bytes를 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr896-2026-10-31/pass01`에 보존했다.
- source 대비 target hash는 2,257/2,257 일치했다. 기존 bytes 백업은 67/67 일치했고, copy 결과의 열린 파일을 보유한 비-preview process는 없었다.
- HTTPS 검증은 2,257/2,257 HTTP 200, 2,257/2,257 SHA-256 일치, 실패 0이다. 이는 선택된 파일 집합만 확인하며 전체 문서 링크 검사는 아니다.

근거: [preflight](./preflight-pass01.json) · [atomic copy와 백업](./copy-and-backup-pass01.json) · [target 및 backup 검증](./target-verification-pass01.json) · [HTTPS 검증 요약](./served-verification-pass01.log) · [served hash 전체 결과](./served-verification-pass01.json).

## Agent Browser 확인

사용자가 지정한 Agent Browser로 [Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 열었다. 렌더된 최신 C11.4 항목에서 PR #893, merge SHA `ae292fff135a925ef30a3e2437082e96c2386e9c`, `140/140`, 공식 `72/213`, C11.4/C11 미완료를 확인했다. [화면](./tailnet-roadmap-agent-browser-pass01.png) · [DOM 확인 결과](./roadmap-agent-browser-verification-pass01.json).

로드맵에서 연결된 Android PNG는 Agent Browser에 1080×2400, iOS PNG는 1206×2622로 표시됐다. 두 캡처는 fixture route/present 상태만 보이며 glyph ink나 pixel equality를 입증하지 않는다. [Android route 결과](./android-image-agent-browser-pass01.json) · [Android 화면](./android-image-agent-browser-pass01.png) · [iOS route 결과](./ios-image-agent-browser-pass01.json) · [iOS 화면](./ios-image-agent-browser-pass01.png).

확인 뒤 browser session을 닫았고 `agent-browser` executable process 0개를 확인했다. Terminal Browser는 사용하지 않았다. [publication/copy 변경 검토 30개](./tailnet-sync-pr-diff-review-30.md).

GitHub Pages는 배포하지 않았다. C11.4와 C11은 미완료이고 공식 완료 `72/213`을 유지한다.
