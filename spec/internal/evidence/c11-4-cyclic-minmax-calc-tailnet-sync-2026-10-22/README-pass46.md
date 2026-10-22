# C11.4 cyclic minmax calc Tailnet 로드맵 동기화 · pass46

병합된 구현 PR [#487](https://github.com/ohah/spinon/pull/487)와 상태·공개 로드맵 문서 PR [#488](https://github.com/ohah/spinon/pull/488)의 결과를 Tailnet 개발 미리보기에 반영했다. PR #487 merge SHA는 `2a04286dba1da4bef45fb6b1a1ee9a1cace54e56`, PR #488 merge SHA는 `48eaefe89d1eaad81cd355e961dadd56e0078c00`이다. C11.4는 미완료이며 공식 집계 `72/213`을 유지한다.

## 선택 root와 build

- 선택 source는 `origin/main` `48eaefe89d1eaad81cd355e961dadd56e0078c00`, tree `475b0b7eb866769ff2a52324441fc7c98974dd29`다. 체크아웃과 fetched `origin/main`의 tree가 일치했다.
- 이전 pass45 선택 root의 STATUS·로드맵 원본은 pass45 source `2f6876c62985a5e8cccd62d34c9fb7ad854a9bde`와 byte 단위로 일치했다. 그 build의 `roadmap.html`, `STATUS.html`, `spec/STATUS.html`도 Tailnet 대상과 같아 이전 미리보기의 시작점을 확인했다.
- pass45 선택 root를 새 경로에 보존 복사하고 pass45 이후의 tracked C11.4 계획, 상태, 공개 로드맵, 두 evidence 디렉터리와 고정 HTML fixture만 반영했다. 저장소의 다른 미추적 증거는 선택 root에 넣지 않았다. 삭제와 심볼릭 링크는 없다. [선택 root 입력·source hash](./selected-root-preparation-pass46.json).
- 고정 Rspress `2.0.22` 선택 build는 3,455개 파일, 157,300,840 bytes, tree SHA-256 `4c913831b910e65b0c99d2d34831f23a45bc614171c278786597915d7a0664c8`을 만들었다. 로드맵, 상태, 계획, 구현 전 비교, 런타임 근거·review, 문서 PR review와 Android/iOS PNG route를 확인했다. [build log](./selected-build-pass46.log) · [route/tree summary](./selected-build-summary-pass46.json).
- Rspress 표 출력의 열 맞춤용 후행 공백과 로그 끝의 빈 줄만 저장본에서 정규화했다. build 결과와 route/tree summary는 변경하지 않았다.
- 이 작업은 선택 문서 build다. 전체 Rspress 문서·dead-link·image 검사는 실행하지 않았고 Pages를 배포하지 않았다.

## Tailnet copy와 hash 검증

- Tailnet Serve의 `/`는 `http://127.0.0.1:8768`에 연결되어 있었다. listener는 기존 `tailnet-preview-server.mjs` PID `1191` 하나였고 동기화 전후 유지했다. tmux 세션도 종료하거나 수정하지 않았다.
- preview root는 `/Users/yoonhb/Library/Application Support/SpinonPreview`다. preflight에서 3,455개 파일을 create 441, replace 193, same 2,821로 분류했다. 삭제는 없고 기존 파일 193개, 총 28,393,658 bytes를 별도 backup에 보관했다. [preflight](./preflight-pass46.json.gz) · [copy와 backup](./copy-and-backup-pass46.json.gz).
- 복사 직전 대상 파일 handle을 검사했다. 열린 파일 5개는 preview listener PID 1191만 보유했고, server에는 idle listen socket 하나뿐이었다. 기존 파일은 backup hash를 확인한 다음 sibling temp file과 atomic rename으로 교체했다.
- copy 후 선택 파일 3,455/3,455, backup 193/193의 길이와 SHA-256이 일치했다. 별도 file-only local verifier도 같은 결과를 확인했다. [local verification](./local-verification-pass46.json.gz).
- `https://macstudio.tailed42f2.ts.net/spinon/`에서 선택 build 파일 3,455개를 각각 GET했다. HTTP 200은 3,455/3,455, 기대 SHA-256 일치는 3,455/3,455, 실패는 0이다. [served verification](./served-verification-pass46.json.gz) · [실행 log](./remote-verification-pass46.log).

## Agent Browser 확인과 종료

- [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)에서 PR #487, merge SHA, `72/213`, Ahem text paint와 CSS border stroke의 구분을 확인했다. [DOM](./roadmap-dom-pass46.json) · [화면](./tailnet-roadmap-agent-browser-pass46.png).
- [공식 상태](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html#c11-css-grid)의 C11.4 checkbox는 unchecked였고 `72/213`과 PR #487 merge SHA가 표시됐다. [DOM](./status-dom-pass46.json) · [화면](./tailnet-status-agent-browser-pass46.png).
- [계획](https://macstudio.tailed42f2.ts.net/spinon/plan/c11-4-cyclic-minmax-calc.html), [런타임 근거](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c11-4-cyclic-minmax-calc-runtime-2026-10-22/README.html), [문서 diff review 30개](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c11-4-cyclic-minmax-calc-runtime-2026-10-22/roadmap-pr-diff-review-30.html) route를 열었다. Review 표는 30개 관점이다. [계획 DOM](./plan-dom-pass46.json) · [근거 DOM](./evidence-dom-pass46.json) · [review DOM](./doc-review-dom-pass46.json).
- 로드맵에서 연결한 Android AVD PNG는 `1080×2400`, iPhone 17 Pro/iOS 26.2 Simulator PNG는 `1206×2622`이고 Agent Browser에서 모두 `complete=true`로 열렸다. [Android DOM](./android-image-dom-pass46.json) · [iOS DOM](./ios-image-dom-pass46.json) · [Android 화면](./tailnet-android-image-agent-browser-pass46.png) · [iOS 화면](./tailnet-ios-image-agent-browser-pass46.png).
- 병합된 [PR #488](https://github.com/ohah/spinon/pull/488) 화면에서 `Merged`와 본문 두 screenshot의 원본 크기를 확인했다. [PR DOM](./pr-dom-after-merge-pass46.json) · [PR 화면](./pr-agent-browser-after-merge-pass46.png).
- 작업 전용 Agent Browser namespace `spinon-c114-roadmap-pr488`를 닫고 종료 상태를 기록했다. [종료 확인](./browser-close-pass46.json).

## 범위

미리보기 갱신은 private Tailnet 문서 확인이다. 모바일 앱을 다시 실행하지 않았다. Android 실기기·hardware GPU, iOS compositor 이후 pixels, CSS border stroke, 일반 CSS admission, WPT 및 전체 C11.4는 검증하지 않았다. screenshots의 overflow는 앱 fixture의 Ahem text paint다. GitHub Pages는 배포하지 않았다.

## 재현 도구와 목록

- `prepare-selected-root-pass46.mjs`, `rspress-pass46.config.ts`, `summarize-selected-build-pass46.mjs`: 선택 root 생성·선택 build route 요약.
- `sync-tailnet-pass46.mjs`: 대상 preflight, copy/backup, 로컬 hash 재검증.
- `verify-local-pass46.mjs`: listener 상태와 무관하게 source·preview·backup 파일의 hash를 읽기 전용으로 재검증.
- `verify-tailnet-pass46.mjs`: Tailnet에서 선택 파일의 HTTP 상태·byte length·SHA-256 재검증.
- JSON 세부 기록은 gzip으로 보관한다. sync 스크립트는 `.json.gz` preflight도 읽는다. 재현 시 preflight/copy/local stdout은 파일로 저장한 뒤 gzip하고, remote verifier는 gzip served record를 직접 만든다.
- [30개 실패 관점 PR diff review](./tailnet-sync-pr-diff-review-30-pass46.md) · [artifact SHA-256 목록](./SHA256SUMS-pass46.txt).
