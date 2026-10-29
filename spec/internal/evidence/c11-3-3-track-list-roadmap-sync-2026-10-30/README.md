# C11.3.3 track-list cycling 상태·로드맵·Tailnet 동기화 근거

## 변경 범위와 공식 상태

구현 PR [#814](https://github.com/ohah/spinon/pull/814)의 bounded behavior를 `spec/STATUS.md`, 내부 계약 0059, C11.3.3 계획과 공개 `spec/public/roadmap.html`에 반영했다. C11.3.3 candidate 및 C11 checkbox는 미완료이고 공식 완료는 `72/213`으로 유지한다. PR #814는 2026-10-29T18:48:29Z Rebase and merge였고 merge SHA는 `4b40ffc3b16cbdde4bb3f66e3e1036ac8a3f8cd3`다. 구현 PR에 required check 설정과 check run은 보고되지 않았다.

## 선택 문서 빌드

- RSPress v2.0.22로 지정된 Markdown·정적 파일만 골라 빌드했다. 최종 source commit은 `1dcdaf2f043f6d1abd85f787e6a6b05f37985dd6`이다.
- 최종 root는 56개 파일·4,191,958 bytes, 선택 build는 51개 파일·6,755,847 bytes다. 14개 필수 문서·화면 route를 확인했고 기존 preview `404.html`은 manifest에서 제외했다.
- 첫 pass의 완료 단계는 선택 root에 simulator `result.json`·화면 PNG가 포함되지 않아 실패했다. 해당 자산을 명시적으로 포함해 pass02를 다시 만들었다. pass02 완료 단계에서는 RSPress가 원본 `roadmap.html`을 출력하지 않아 route 검증이 실패했다. 이를 출력에 복사하도록 고친 뒤 pass02 확인을 마쳤다. 그 후 계획 상태 문구를 사실에 맞게 갱신하고 새 pass03 경로로 최종 빌드를 만들었다. 이전 pass 산출물은 보존했다.
- 최종 source manifest: [`selected-root-pass03.json`](./selected-root-pass03.json) · 최종 output manifest와 필수 route: [`selected-build-verification-pass03.json`](./selected-build-verification-pass03.json).

## Tailnet 미리보기 동기화

- 기존 `SpinonPreview` `/spinon/` route를 갱신했다. port 8768 listener는 기존 `tailnet-preview-server.mjs`였고 서버를 재시작하거나 종료하지 않았다.
- 최종 preflight는 51 route를 신규 17, 교체 15, 동일 19로 분류했다. 교체 전 15개 파일·2,141,971 bytes를 별도 backup root에 보존했다.
- 복사는 32개 파일, backup은 15개이며 삭제는 0개다. 검증은 local target 51/51, backup 15/15, Tailnet remote HTTP 200 및 SHA-256 일치 51/51을 기록한다.
- 원자료: [preflight](./tailnet-preflight-pass03.json) · [copy와 backup](./tailnet-copy-pass03.json) · [local 및 remote 검증](./tailnet-verify-pass03.json) · [요약](./tailnet-sync-summary-pass03.json).
- 첫 preflight는 preview listener에 잠시 연결된 local established socket 때문에 중단됐다. `lsof`에서 확인된 것은 listener와 127.0.0.1 client connection뿐이었고 잠시 뒤 connection이 닫혔다. 실행 중인 프로세스는 종료하지 않았으며 후속 preflight·copy·verify가 통과했다.

## Agent Browser 표시 확인

터미널 브라우저를 사용하지 않았다. Agent Browser `spinon-c1133-roadmap-sync-pass03`로 Tailnet 페이지를 확인한 뒤 세션을 닫았고, `ps`에서 Agent Browser process가 남지 않았다.

- 로드맵에는 PR #814, merge SHA, 37 scene·118 node, 범위 한계와 `72/213`이 보인다. [화면](./tailnet-roadmap-agent-browser-pass03.png)
- 상태 페이지에는 PR #814의 2026-10-30 기록이 표시되고, C11·C11.3·C11.3.3 candidate checkbox는 모두 unchecked다. [화면](./tailnet-status-c11-agent-browser-pass03.png)
- 계획 페이지의 완료 동기화 문구와 내부 구현 evidence 페이지를 열어 37/118 및 두 플랫폼 28/28 표본 링크를 확인했다.
- Android AVD 화면은 1080×2400, iOS Simulator 화면은 1206×2622이며 두 이미지 모두 browser `complete=true`, `naturalWidth > 0`이다. [browser 관찰](./agent-browser-verification-pass03.json) · [세션 종료](./browser-close-pass03.json) · [화면 SHA-256](./SHA256SUMS-pass03.txt).
- PR #815 본문의 두 Tailnet 화면은 Agent Browser에서 1280×633으로 모두 표시됐다. 첫 확인에서 두 번째 이미지 링크가 브랜치 이름 `docs/c11-3-3-track-list-roadmap-sync`를 잘못 사용해 `naturalWidth=0`이었다. URL을 실제 branch `docs/c11-3-3-track-list-cycling-roadmap-sync`의 raw 이미지 경로로 고친 뒤 2/2 모두 로드됨을 확인했다. [초기 관찰](./pr815-agent-browser-images-initial.raw.json) · [수정 후 관찰](./pr815-agent-browser-images-final.raw.json) · [PR 브라우저 종료](./browser-close-pr815.json).

## 검토와 제한

- 문서 변경의 30개 실패 관점은 [PR diff 검토](./roadmap-sync-pr-diff-review-30.md)에 기록했다. 선택 빌드·Tailnet 동기화 helper는 [별도 20개 검토](./tailnet-sync-script-review-20.md)를 거쳤다.
- Tailnet은 개발 미리보기만 갱신했다. GitHub Pages workflow, `gh-pages`, simulator runtime 재실행은 수행하지 않았다.
- 결과는 `RuntimeBlockFlexGridTemplateV1` template-area bounded slice에 한정한다. numeric implicit placement, backward/negative sizing, non-px track list, Spinon WPT 적합성, 실기기·hardware GPU 및 iOS compositor 이후 pixel은 미검증이다.
