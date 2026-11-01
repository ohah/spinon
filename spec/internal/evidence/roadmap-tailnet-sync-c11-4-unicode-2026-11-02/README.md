# PR #991 병합 후 C11.4 Unicode roadmap Tailnet 동기화 · 2026-11-02

## 변경 범위

PR [#991](https://github.com/ohah/spinon/pull/991)은 merge SHA `108d0eed91220f81971f8c2eb7c4d2b727b26452`로 Rebase and merge됐다. `spec/STATUS.md`와 계획 문서는 merge SHA·시각을 기록하고, `spec/public/roadmap.html`에는 구현 candidate 카드 한 개를 추가한다. C11/C11.4 checkbox는 미완료이며 공식 완료 수 `72/213`은 그대로다.

## 선택 build·preview 동기화

선택 root는 Git index의 tracked source에서 roadmap, STATUS, C11 계획·계약 및 C11.4 Unicode evidence route를 모았다. source HEAD는 `108d0eed91220f81971f8c2eb7c4d2b727b26452`, tree는 `476f6c76178e318468046df7a7b565ab862fb588`, tracked diff SHA-256은 `658dfbc7622c659c146affce1ed9b0d7126df0a8fa340912c9bb4b724e063c60`이다. 선택 root는 301개 파일·9,362,368 bytes, 생성 publication은 351개 route·17,566,604 bytes다.

첫 `bunx rspress build` 시도는 잠금 버전 대신 RSPress `1.47.2`를 내려받아 실패했다. 저장소에 설치된 RSPress `2.0.22` 실행 파일과 고정 config로 다시 빌드했고 통과했다. 재현 명령은 다음과 같다.

```sh
mise exec -- packages/docs/node_modules/.bin/rspress build --config \
  spec/internal/evidence/roadmap-tailnet-sync-c11-4-unicode-2026-11-02/rspress.config.ts
```

Manifest의 roadmap 카드 로컬 링크 8개는 경로와 SHA-256을 대조했다. 전체 문서 링크 검사와 전체 사이트 dead-link 검사는 실행하지 않았다. 선택 route에서 기존에 열려 있던 `plan/c11-css-grid` HTML·Markdown 경로는 publication 후보에서 제외했다.

Tailnet backend `Running`과 기존 preview server PID `28040`을 확인했다. `/Users/yoonhb/Library/Application Support/SpinonPreview`에 선택 route 351개를 복사했다(신규 303, 교체 21, 동일 27). 변경 대상 open handle은 0개였고, 21개 교체 파일의 backup을 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-c11-4-unicode-2026-11-02/pass01`에서 모두 검증했다. 삭제는 없고 preview server도 재시작하지 않았다. 복사 뒤 로컬·Tailnet HTTPS route 351개 모두 HTTP 200 및 SHA-256 일치, backup 21개 모두 hash 일치였다.

## 결과

세부 값은 `preflight.json`, `copy-and-backup.json`, `served-verification.json`, `selected-root.json`, `selected-publication.json`, `selected-route-audit.json`에 보존했다. Agent Browser로 실제 Tailnet roadmap의 새 #991 카드, merge SHA와 `72/213` 문구를 확인했다. 데스크톱은 1280px viewport/document width 일치, 모바일은 390px viewport/document width 일치와 가로 overflow 부재를 확인했다. 화면은 `tailnet-roadmap-agent-browser-desktop-pr991-card-focused.png` (1280×900) 및 `tailnet-roadmap-agent-browser-mobile-pr991-card-focused.png` (390×844)에 있다. DOM·viewport 결과와 screenshot SHA-256은 `agent-browser-verification.json`을 참조한다. 검증을 마친 뒤 해당 Agent Browser session을 닫고 그 Chrome 및 Agent Browser 프로세스가 사라진 것을 확인했다.

로드맵은 C11.4 구현 진척을 보여주지만 이 candidate는 일반 CSS admission, arbitrary locale/font/input, glyph raster, 실기기/hardware GPU, full WPT 또는 전체 C11.4/C11 완료를 뜻하지 않는다.
