# C11.4 zoomed-grid Tailnet 로드맵 동기화

2026-10-29 · PR #783 구현과 PR #784 상태·계약 갱신 후 `spec/public/roadmap.html`을 최신 runtime 결과에 맞췄다. 공식 C11.4 미완료 상태와 `72/213`은 유지한다. 외부 GitHub Pages는 배포하지 않았다.

## 변경된 로드맵 항목

새 runtime 항목은 PR #783 Rebase and merge (`2026-10-29T01:51:27Z`, merge SHA `664f4a848a3bf21bec4d2be6080633a8e3b4c5a4`)와 PR #784 Rebase and merge (`2026-10-29T01:54:34Z`, merge SHA `4abc761fad14a8c233152c40c4620b5b22773a93`)를 연결한다. 두 PR의 GitHub check rollup은 비어 있어 CI 통과로 기록하지 않았다.

페이지는 사전 비교와 실제 runtime 결과를 별도 문단으로 보여준다. Chrome CSSOM `40px`, Taffy typed limit `80px`, contribution `800px`, used row `80px`을 구분한다. Android API 37 ARM64 software AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator의 compositor 제출 전 Metal 표본은 각각 4/4 exact다. 일반 CSS `zoom`, 전체 WPT와 전체 C11.4는 미지원이며 공식 체크박스와 `72/213`은 미완료다.

## 빌드와 동기화

- 전체 `bun run docs:build`는 기존 문서의 dead-link 검사와 `rust,ignore` Shiki 언어 오류로 실패했다. 이 실패는 수정 범위 밖의 이전 문서에서 발생했다. 이 결과를 전체 docs build 성공으로 보고하지 않는다.
- 선택 route RSPress build는 [pass02 build log](./tailnet-build-pass02.log)에서 통과했다. 일회성 selected root의 link/image dead-check만 끄고 로드맵, STATUS, 내부 계약 0059, C11.4 계획과 관련 증거 route를 빌드했다.
- 두 RSPress log의 표 정렬용 후행 공백과 끝의 빈 줄만 제거해 diff 검사를 통과시켰다. 로그의 문장과 빌드 결과는 바꾸지 않았다.
- 선택 source는 84개 파일, 2,979,804 bytes다. Tailnet 복사는 build 산출물 및 raw evidence를 합친 111 route, 6,161,652 bytes만 대상으로 했다.
- Tailscale backend는 `Running`; 기존 preview listener PID `28040`은 유지했고 서버를 재시작하지 않았다. 변경된 기존 target 14개 중 open handle이 있는 파일은 0개였다.
- 14개 변경 파일은 먼저 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-c11-4-zoom-2026-10-29-pass02`에 보존했다. 새 파일 11개를 추가하고, 변경 14개를 원자적으로 교체하고, 동일 파일 86개는 그대로 뒀다. 파일 삭제는 없었다.
- 로컬 preview와 Tailnet origin에서 각 route를 확인한 222 response 모두 HTTP 200과 source SHA-256 일치다. [preflight](./target-preflight-pass02.json) · [open handles](./target-open-handles-pass02.json) · [copy log](./target-copy-log-pass02.json) · [served hash check](./served-hash-check-pass02.json).

첫 sync script 실행은 두 증거 폴더의 `README.md`가 같은 target route로 충돌하는 것을 발견해 쓰기 전에 중단했다. route prefix를 각각 유지하도록 고쳐 pass01을 동기화했다. 첫 Agent Browser 화면은 runtime 문단이 288px 열에 들어가는 점을 보여줬다. 두 문단에 `hero-progress` 전체폭 class를 적용하고 selected-site를 다시 빌드·동기화했다. pass01 파일과 화면은 이 evidence 폴더에 보존한다.

## Agent Browser 확인

- 동기화 전 `roadmap.html`에는 zoomed-grid runtime marker와 PR #783 링크가 없었다.
- pass02 뒤 로드맵에 PR #783/#784 merge SHA, frame·track 결과, 72/213 및 미지원 범위가 보인다. 최종 runtime 문단 폭은 1240px이다.
- `STATUS.html`에서 merge SHA, 빈 check 기록, 공식 완료 수를 확인했다.
- 내부 계약 0059의 zoomed assertion 절과 runtime README·병합 후 review route를 열었다. 병합 후 review 표는 30행이다.
- Android screenshot은 `1080×2400`, iOS screenshot은 `1206×2622`로 열렸다.
- 최종 로드맵 화면: [Agent Browser screenshot](./tailnet-roadmap-agent-browser-pass02.png), 1280×633, SHA-256 `d10bbb719ebf362f1a3a2eb45139a1691911fa9aac1f1b091f197a31f0b41bba`.
- 전체 route 및 browser 표본: [Agent Browser verification](./agent-browser-verification-pass02.json). 확인 세션 종료와 process cleanup 결과도 이 JSON에 기록한다.

문서 diff는 [로드맵·Tailnet 적대적 검토 30개](./roadmap-pr-diff-review-30.md)에서 확인했다. 이 동기화는 internal Tailnet preview에만 반영되며 GitHub Pages 배포를 뜻하지 않는다.
