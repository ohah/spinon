# Tailnet 로드맵 동기화 · 2026-10-27

## 결과와 공식 상태

공식 상태 대장과 공개 로드맵에 반영된 C11.4 direct-text 6-scene 결과를 Tailnet 미리보기에 게시했다. 문서 변경 PR [#680](https://github.com/ohah/spinon/pull/680)은 2026-10-27T03:40:24Z Rebase and merge로 병합됐고 merge SHA는 `0cf264b7fdfe315854d0d53ece3989f4a8854f4d`다. 선택 build source tree `3396de1a81daa90c4bd8fa1096d48aaa5c2682a5`와 병합 main tree가 일치한다.

공식 집계는 `72/213`이며 C11 및 C11.4 체크는 미완료다. 이 게시 작업은 문서 배포 근거다. 앱 동작을 추가 검증하거나 C11 완료를 뜻하지 않는다. 구현 실행 범위와 한계는 [PR #679 실행 README](../c11-4-generic-direct-text-runtime-matrix-2026-10-27/README.md)에 있다.

## 선택 빌드와 경로 범위

- Rspress `2.0.22` 선택 build가 통과해 18개 생성 문서 페이지를 만들었다. 선택 publication은 생성 페이지, Markdown 원본, 링크된 matrix·화면·archive 결과와 정적 자원을 포함한 84개 route 파일, 총 `16,279,517` bytes다. [표시용 공백 정리 build log](rspress-build-pass01.log), [원본 build 출력](rspress-build-pass01.log.raw.gz), [root manifest](selected-root-pass01.json), [publication manifest](selected-build-pass01.json)를 보라.
- 생성 설정은 전체 문서 root의 기존 문제 때문에 dead-link/dead-image 검사만 끈다. 이를 전체 docs build 또는 전체 링크 건전성 통과로 해석하지 않는다.
- 별도 넓은 스캔은 생성 HTML 22개에서 내부 링크 17,638개를 살폈고 선택 publication과 기존 preview root에 없는 참조 379개를 기록했다. 이는 과거 evidence와 선택 범위 밖 route를 포함한 전체 참조 그래프 통과가 아님을 보인다. 실패 결과 원본은 [pass01 JSON](selected-link-check-pass01.json.gz)에 gzip 압축해 보존했다.
- 이 게시의 좁은 목표인 새 direct-text 로드맵 문단의 Tailnet 링크 8개와 핵심 문서·화면·matrix 12개 route는 모두 선택 publication에서 확인했다. PNG signature도 유효했다. [pass02 검사 결과](selected-c11-4-links-pass02.json)와 [검사기](verify-c11-4-links-pass02.mjs)를 보라.

재현 스크립트는 [선택 root 준비](prepare-selected-root-pass01.mjs), [선택 Rspress config](rspress-pass01-selected.config.ts), [선택 route 구성](stage-selected-build-pass01.mjs), [좁은 링크 검사](verify-c11-4-links-pass02.mjs)에 있다. 이전 pass08 selected root는 hardlink 원본으로 읽었고 overlay 과정에서 원본 파일을 덮어쓰지 않았다.

## Tailnet 사전 확인과 동기화

- Tailscale backend는 Running, Mac Studio는 online이었다. preview server PID `28040`은 `127.0.0.1:8768`에서 수신 중이었고 Tailnet roadmap의 사전 요청은 HTTP 200이었다. 서버를 재시작하지 않았다. [preflight](tailnet-preflight-pass01.json)를 보라.
- 교체 전 모든 대상 경로를 검사했고 교체 대상 7개 모두 open handle이 없었다. 84개 가운데 72개를 새로 만들고 7개를 교체했으며 5개는 이미 일치했다. 교체 파일의 기존 내용 `3,069,074` bytes는 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-27/pass01`에 byte-exact 보존했다. 삭제한 route는 없다. [대상 handle 결과](target-open-handles-pass01.json)와 [copy report](tailnet-copy-pass01.json)를 보라.
- 게시 후 local 84/84 route와 HTTPS 84/84 response body가 build manifest의 길이 및 SHA-256과 일치했다. 7/7 backup도 기존 target hash와 일치했다. [검증 스크립트](verify-tailnet-pass01.mjs)와 [최종 검증](tailnet-verify-pass01.json)을 보라.

## Agent Browser 화면 확인과 종료

- Terminal Browser를 사용하지 않았다. Agent Browser에서 [Tailnet roadmap](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 열어 `72/213`, C11 미완료 checkbox, PR #679 direct-text 기록 및 그 링크 8개를 확인했다. [관측 JSON](tailnet-roadmap-agent-browser-pass01.json)과 [화면](tailnet-roadmap-agent-browser-pass01.png)을 보라.
- [계획 페이지](tailnet-plan-agent-browser-pass01.json)에서 PR #679, `72/213`, Android/iOS와 direct-text 한계 문구를 확인했다.
- Tailnet에서 직접 연 Android 화면은 1080×2400, iOS Simulator 화면은 1206×2622로 complete 로드됐다. [Android](tailnet-android-image-agent-browser-pass01.json) 및 [iOS](tailnet-ios-image-agent-browser-pass01.json) 관측 JSON에 각 URL과 intrinsic dimensions가 있다.
- 사용한 Agent Browser session `spinon-c114-review-273954f6b636`를 닫았다. close 응답 뒤 잔류한 해당 session의 Agent Browser/Chrome process group만 SIGTERM으로 종료했다. 이후 session list는 비었고 정확한 Agent Browser 실행 파일과 Chrome process group 모두 사라진 것을 확인했다. 다른 browser process는 종료하지 않았다. [lifecycle 기록](agent-browser-lifecycle-pass01.json).

## 범위

이 동기화는 선택한 84개 Tailnet route 게시와 hash 일치, roadmap 및 plan 화면, 두 Simulator screenshot 제공만 검증한다. 전체 문서 링크 그래프, 전체 docs build, 앱 코드, 실기기, glyph paint, 일반 CSS text admission, 전체 C11.4 Stage A, C11 완료는 검증하지 않았다. 공식 `72/213`과 미완료 상태를 유지한다.
