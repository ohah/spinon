# C11.4 PreWrap 병합 후 Tailnet 로드맵 동기화

## 대상과 결과

PR [#870](https://github.com/ohah/spinon/pull/870)이 `Rebase and merge`로 병합된 뒤 `spec/STATUS.md`, 내부 계약 0059, PreWrap 계획 및 공개 로드맵을 병합 SHA `5265793d71a02a6206a09b9bbbc9658696dfd992`와 현재 검증 한계에 맞췄다. C11/C11.4 checkbox는 미완료로 유지했고 공식 상태는 `72/213`이다. GitHub PR checks 결과가 없고 main에는 required status checks가 없어 CI 통과로 기록하지 않았다.

수정한 선택 publication을 Tailnet preview에 반영했다. Tailscale backend는 `Running`, preview server PID `28040`은 `127.0.0.1:8768`에서 응답했다. 서버를 재시작하지 않았다. 340 route, 18,649,180 bytes를 준비했고 사전 비교는 신규 21, 교체 31, 동일 288이었다. 교체 대상 open handle은 0개였고, 기존 파일 삭제는 없었다. 교체 31개를 별도 backup 경로에 보존했다.

copy 단계 첫 호출은 기존 pass01 백업 상위 디렉터리가 있어 `mkdir`가 `EEXIST`를 반환하며 파일 변경 전에 멈췄다. 기존 pass01은 건드리지 않았다. pass02 전용 copy script가 이미 있는 부모를 symlink 없이 확인한 뒤 새 하위 경로를 만드는지 검사하도록 보강했고 다시 실행했다. 최종 verify에서 340/340 HTTPS route가 HTTP 200, 응답 바이트와 SHA-256이 publication과 일치했으며 31/31 backup hash가 맞았다. 삭제는 없었다.

Agent Browser에서 [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)과 [공식 상태](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html)을 확인했다. 로드맵은 PR #870, merge SHA, 실패한 workspace assertion, `72/213`, CI 통과를 주장하지 않는 문구를 표시했다. 상태 페이지는 C11과 C11.4 checkbox가 모두 꺼진 것을 보였다. [Agent Browser 관찰 JSON](agent-browser-roadmap-pass02.json) · [STATUS 관찰 JSON](agent-browser-status-pass02.json) · [로드맵 화면](tailnet-roadmap-agent-browser-pass02.png) · [브라우저 종료 확인](agent-browser-lifecycle.json). 터미널 브라우저는 사용하지 않았다.

## 재현과 무결성 자료

- source HEAD `5265793d71a02a6206a09b9bbbc9658696dfd992`, source tree `ba873209563842463191db864eaef73a7e1834f6`, tracked dirty diff SHA-256 `267a75e029ea67fb5b23bf9e3cf183f13c54f7876b12fe5336df927566abb51f`는 [selected root manifest](selected-root.json)에 있다.
- RSPress 2.0.22 결과는 [build log](rspress-build.log)와 [publication manifest](selected-publication.json)에 있다. 선택 publication은 전체 문서 dead link 검사가 아니다.
- 대상 전 상태·hash와 open handle은 [preflight](preflight.json), 복사 결과·backup receipt는 [copy report](copy-and-backup.json), local/HTTPS 검증은 [served verification](served-verification.json)에 있다.
- [동기화 스크립트](sync-tailnet.mjs)는 `preflight`, `copy`, `verify`로 분리되어 있다. [root 구성기](prepare-selected-root.mjs), [publication 구성기](stage-publication.mjs), [Rspress config](rspress.config.ts)를 사용했다.

이 증거는 선택된 340 route의 Tailnet 배포와 해당 페이지의 제한된 브라우저 관찰만 뒷받침한다. 전체 문서 링크, 전체 C11 적합성, 앱 glyph pixel 또는 실기기 동작을 증명하지 않는다.
