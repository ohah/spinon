# C11.4 PR #391 Tailnet 로드맵 동기화

PR [#391](https://github.com/ohah/spinon/pull/391)의 Rebase and merge 결과(merge SHA `e3119ea21bdd6196f334c104edb3bb33dfe70b0c`)를 사용자가 지정한 정적 Tailnet 미리보기 `/spinon/`에 반영한다. GitHub required status check은 보고되지 않았으므로 CI 통과로 표시하지 않는다. `spec/STATUS.md`의 C11.4 checkbox와 공식 완료 집계는 변경하지 않는다.

이번 갱신은 공개 로드맵의 PR #391 항목, 공식 상태 대장, 내부 인덱스, 집중 계획과 실행 근거를 동기화한다. 실행 근거는 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 138/138 scene, 14개 신규 typed auto-track scene, 960 frame scalar·1,019 used-track size의 교차 비교를 기록한다. 두 플랫폼의 수치 비교 차이는 0 CSS px다. 이는 simulator 결과이며 실기기, Android hardware GPU, iOS compositor 이후 표면, WPT 또는 전체 CSS Grid 지원 완료를 뜻하지 않는다.

## Tailnet 반영 결과

Pass16 초기 동기화에서는 선택된 1,606개 파일을 반영했다. 112개를 새로 만들고 86개를 교체했으며, 1,408개는 기존 바이트와 같았다. 교체 전 backup 86/86개가 SHA-256으로 일치했고, 교체 대상 중 열린 파일은 0개였다. 전체 1,606개 경로가 HTTP 200, byte 수, SHA-256 모두 일치했다. 삭제는 없었다.

Pass16 Agent Browser는 로드맵의 PR #391 항목과 SHA, `STATUS`의 C11.4 미완료 checkbox, 계획의 병합 상태를 확인했다. 실행 evidence README에서 새 캡처 3개가 GitHub blob 404로 빠지는 것을 발견해 pass17 선택 빌드에 Android/iOS `columns-fill-negative-positive-cycle.png`와 iOS `typed-auto.png`를 추가했다. pass17 Browser에서는 이 세 이미지가 모두 Tailnet에서 로드됐다. 리뷰 결과를 포함한 최종 pass18 동기화의 전체 파일 검증은 [preflight](https://github.com/ohah/spinon/blob/docs/c11-4-pr391-tailnet-sync-20261020/spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/preflight-pass18.json), [열린 파일 기록](https://github.com/ohah/spinon/blob/docs/c11-4-pr391-tailnet-sync-20261020/spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/open-handles-before-copy-pass18.json), [backup/copy 결과](https://github.com/ohah/spinon/blob/docs/c11-4-pr391-tailnet-sync-2026-10-20/copy-and-backup-pass18.json), [served byte/hash 결과](https://github.com/ohah/spinon/blob/docs/c11-4-pr391-tailnet-sync-2026-10-20/served-verification-pass18.json)에 남긴다.

## Agent Browser 화면

Pass18 Agent Browser가 확인한 Tailnet 화면이다. roadmap, STATUS, plan 캡처는 변경된 병합/미완료 상태를 보여준다. evidence 페이지의 Android/iOS 실행 캡처도 Tailnet에서 직접 제공한다.

![Tailnet 로드맵의 C11.4 PR #391 항목](https://raw.githubusercontent.com/ohah/spinon/docs/c11-4-pr391-tailnet-sync-20261020/spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/tailnet-roadmap-agent-browser-pass18.png)

![Tailnet 공식 상태 대장의 C11.4 미완료 상태](https://raw.githubusercontent.com/ohah/spinon/docs/c11-4-pr391-tailnet-sync-20261020/spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/tailnet-status-agent-browser-pass18.png)

![Tailnet C11.4 typed implicit 계획](https://raw.githubusercontent.com/ohah/spinon/docs/c11-4-pr391-tailnet-sync-20261020/spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/tailnet-plan-agent-browser-pass18.png)


## 반영 범위와 한계

- `/Users/yoonhb/Library/Application Support/SpinonPreview`의 선택된 정적 문서 파일만 갱신한다. 교체 파일은 전용 `SpinonPreviewBackups/c11-4-pr391-tailnet-sync-2026-10-20/pass16` 및 `pass17`, `pass18`에 보관하고 SHA-256을 대조한다. 미리보기에서 파일을 삭제하지 않는다.
- Tailnet preview server와 기존 tmux 세션을 종료하지 않는다. GitHub Pages workflow, 앱 또는 simulator는 이번 문서 동기화에서 실행하지 않는다.
- Agent Browser를 사용해 `/spinon/roadmap.html`, `/spinon/STATUS.html`, 계획 및 evidence 페이지를 확인한다. 이번에 시작한 Agent Browser 세션은 확인 뒤 닫는다.
- C11.4와 전체 CSS Grid 지원은 미완료다. Simulator 캡처는 실기기 또는 compositor 이후 표면 검증을 대신하지 않는다.
