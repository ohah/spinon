# Tailnet roadmap sync · pass08

2026-10-25에 PR [#624](https://github.com/ohah/spinon/pull/624) 병합 후의 `origin/main` (`f197239b0168e2356f4d0ece03f5d2e4c40278fe`)을 기준으로 Tailnet 미리보기를 갱신했다. 최초 pass08 선택에서 `origin/main`에 없는 예비 20관점 검토 파일과 별칭 4개가 포함된 사실을 PR diff 검토에서 발견했다. 아래 수정 pass08은 해당 로컬 파일을 제외하고 선택 build·Tailnet hash 검증을 다시 수행했다. 공식 C11.4 checkbox는 미완료이고 완료 집계는 `72/213`이다.

## 선택 build와 route

- 수정된 Rspress output에는 HTML 파일 757개가 생성됐다. Tailnet stage는 이 가운데 생성된 24개 HTML route를 골라 base/raw route와 합쳐 258개 route를 반영했다. Manifest의 `selectedGeneratedRouteCount`는 전체 페이지 수가 아니라 이 선택 route 수다. config는 dead-link/image 검사를 끄므로 전체 `docs:build` 통과를 뜻하지 않는다. 저장한 build log는 표 정렬용 trailing whitespace만 정리했다. [build log](rspress-build-pass08-corrected.log)
- pass07 선택 source root를 hardlink한 뒤 `origin/main`에 추적되는 STATUS, roadmap, C11.4 계획과 cyclic runtime evidence만 overlay했다. 예비 review 파일 `merge-roadmap-pr-diff-review-20.md`와 그 별칭 네 개는 [source manifest](selected-root-pass08-corrected.json)에 hash와 함께 제외 기록이 있다.
- 최종 선택 root에는 258 routes, 25,913,401 bytes가 있다. Route별 크기/SHA-256은 [수정 selected build manifest](selected-build-verification-pass08-corrected.json)에 있다. 선택 script는 각 원본 파일이 pinned `origin/main`에 있고 로컬 bytes와 일치하는지 확인한다.
- 최초 pass08의 262-route source/build/preflight/sync/verify manifests와 `prepare-selected-root-pass08.mjs`, `rspress-pass08.config.ts`, `stage-selected-build-pass08.mjs`, `preflight-tailnet-pass08.mjs`, `sync-tailnet-pass08.mjs`, `verify-tailnet-pass08.mjs`는 검토 이력만을 위해 보존했다. 이 스크립트들은 잘못된 route를 다시 만들 수 있으므로 재실행하지 않고, `*-corrected` 스크립트만 현재 절차로 사용한다.

## Tailnet 반영과 검증

- 최신 live 관측에서 Tailnet backend는 `Running`, Mac Studio는 online, preview PID 28040은 `127.0.0.1:8768`에서 수신하고 local HTTP 200을 반환했다. 이전 pass08 실행 노트에는 PID 1180의 오래된 read handle을 확인해 preview server만 재시작했다고 기록돼 있지만, 종료 명령의 원본 출력은 이 PR에 보존되지 않았다. 이 보고서는 새 PID의 현재 liveness를 증명한다. [서버 관측과 근거 한계](preview-server-restart-pass08.json)
- 최초 pass08 preflight에서 네 review 별칭은 모두 target 미존재 `create`로 기록됐다. 수정할 때 각 파일의 정확한 hash와 `lsof` 닫힘을 확인하고 byte-exact 백업 후 네 경로만 제거했다. 각 경로는 로컬과 Tailnet에서 404다. [별칭 correction 및 백업](tailnet-review20-alias-correction-pass08.json) · backup root: `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-25/pass08-untracked-review20-correction`
- 수정 pass08 preflight는 258개 경로 중 25개 생성, 24개 교체, 209개 동일을 확인했다. 24개 교체 전 파일은 [pass08-corrected backup root](</Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-25/pass08-corrected>)에 byte-exact로 보존했다. [수정 preflight](tailnet-preflight-pass08-corrected.json) · [교체 대상 handle 검사](target-open-handles-pass08-corrected.json) · [sync 결과](tailnet-copy-pass08-corrected.json)
- 최종 수정 검사에서 local preview 258/258, corrected backup 24/24, Tailnet HTTP 200 및 response body SHA-256 258/258이 일치했다. 별도로 제외한 네 별칭은 404다. [최종 corrected verification manifest](tailnet-verify-pass08-corrected.json)

## Agent Browser 확인

- 수정 sync 뒤 Agent Browser로 Tailnet roadmap을 다시 열었다. DOM에서 PR #623 병합 기록, `72/213`, orthogonal row 상태를 확인했고 [최종 화면](tailnet-roadmap-agent-browser-pass08-corrected.png)을 캡처했다. [관측 manifest](tailnet-roadmap-agent-browser-pass08-corrected.json)
- 실행 후 `agent-browser close --all`을 호출했다. 세션 목록은 비었고 Agent Browser process도 남지 않았다. [종료 관측](agent-browser-lifecycle-pass08-corrected.json). 터미널 브라우저는 사용하지 않았다.

## 범위

- 이 작업은 docs preview sync이며 제품 코드·공식 완료 체크·완료 집계·내부 계약 버전을 바꾸지 않는다.
- 최종 Tailnet route hash 일치는 선택한 258개 파일에 대한 확인이다. 최초 262-route report는 correction 전의 역사 기록이다. 선택 build는 저장소 전체 dead-link 검사를 실행하지 않았고, 이번 단계에서 제품 테스트나 simulator 실행을 하지 않았다.
