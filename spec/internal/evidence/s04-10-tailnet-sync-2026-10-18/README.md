# S04.10 Tailnet 미리보기 동기화 근거

## 범위

[확인 frame 순수 Rust 후보 PR #291](https://github.com/ohah/spinon/pull/291)은 Rebase and merge로 병합됐다. merge SHA는 `e906aed109a7af2b9f7ccfd0cd64ba4b806f5869`다. 이 문서 변경은 병합된 후보와 그 미완료 경계를 Tailnet 문서 미리보기에 맞추고 동기화 근거를 남긴다.

- [Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)
- [Tailnet 공식 상태 대장](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html)
- [내부 계약 0079](https://macstudio.tailed42f2.ts.net/spinon/internal/0079-s04-10-confirmed-frame-registry.html)
- [순수 Rust 후보 구현 근거](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/s04-10-presented-frame-gate-2026-10-18/README.html)

S04.10과 S05.2는 미완료로 남아 있다. 이 후보는 Android/iOS presentation callback, 입력 clock/order, 앱 route, Simulator UI, 실기기 또는 compositor 이후 표시를 검증하지 않는다. 문서 화면 캡처는 앱 동작 화면으로 취급하지 않는다.

## 문서 빌드와 전달

Rspress `2.0.22`로 S04.10에 필요한 선택 문서 묶음만 빌드했다. 최종 HTML route 수, 동기화 전후 파일 수, backup 수, HTTP 및 SHA-256 결과는 아래의 `final-*.json` 기록을 따른다. 저장소 전체 문서 빌드와 전체 Markdown 링크 검사는 수행하지 않았다. 선택 빌드 로그는 [final-selected-build.log](final-selected-build.log)에 있다.

Tailnet preview는 저장소 checkout을 직접 제공하지 않아 렌더 산출물과 근거 파일을 별도 preview root에 copy-only로 반영했다. 대상 파일을 삭제하지 않았고, 기존 파일을 바꾸기 전에 전용 backup root에 저장했다. Listener는 유지했다. 사전 inventory, manifest, copy check, backup check와 local/Tailnet served hash 결과는 아래 기록에 있다.

- [최종 preflight](final-preflight.json)
- [최종 file manifest](final-manifest.json)
- [최종 copy verification](final-copy-verification.json)
- [최종 backup verification](final-backup-verification.json)
- [최종 HTTP·served hash verification](final-served-verification.json)

## Agent Browser 확인

Agent Browser에서 Tailnet 로드맵, 상태 대장과 후보 구현 근거 페이지를 열었다. 로드맵은 미완료 207개와 공식 완료 `74/207`을 표시했다. 상태 대장에서 S04.10과 S05.2 checkbox는 둘 다 unchecked/disabled였다. [브라우저 관찰 JSON](browser-verification.json), [요청 로그](agent-browser-network.log), [로드맵 화면](tailnet-roadmap-agent-browser.png), [상태 대장 화면](tailnet-status-agent-browser.png), [후보 근거 화면](tailnet-evidence-agent-browser.png)을 함께 보관한다.

문서 head가 선언한 `/spinon/icon.svg`는 HTTP 200으로 제공됐다. 브라우저는 페이지가 선언하지 않은 root `/favicon.ico`도 별도로 요청해 404를 받았다. 이는 기록에서 제외하지 않았으며 사이트 본문이 선언한 icon route와 구분했다.

브라우저 작업 후 Agent Browser session을 닫고 task-created process가 남지 않았는지 확인했다. 결과는 [종료 근거](browser-session-shutdown.json)에 있다. screenshot은 문서 미리보기의 근거이며 app 화면이나 플랫폼 표시 callback 증거가 아니다.

## 변경 실패 관점 검토

[Tailnet 동기화 PR 변경 관점 30개](pr-delta-review-30.md)는 preview source/staleness, status overclaim, hash/backup/copy 실패, Browser route·resource 경계와 evidence privacy를 각각 확인한다.
