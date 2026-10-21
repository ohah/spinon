# C11.4 cyclic `fit-content(calc())` Tailnet preview 동기화 · pass42

PR [#483](https://github.com/ohah/spinon/pull/483)의 계획과 Chrome 사전 비교 자료를 Tailnet 미리보기에서 열 수 있도록 선택 build를 갱신했다. 이 pass는 계획 문서·preview evidence만 다룬다. 공식 상태 대장과 public roadmap 내용은 `origin/main`에서 가져온 그대로이고, C11.4는 미완료이며 공식 완료 수 `72/213`을 유지한다.

## 선택 source와 build

- workspace source commit `76a62a6dfb2ed334f8f1ad0909d2a9fe131d8d2a`의 tree와 `origin/main` commit `527d57b14df5a392ddfdd9a5c24437f784e544ef`의 tree가 모두 `87af311336ac424fa803d26ed684ef6c9c8a5527`로 일치했다. 추적 파일 변경은 없었다. [선택 root 복사·source hash](./selected-root-preparation-pass42.json).
- pass41 selected root를 보존 복사한 뒤 현재 status·roadmap, 새 집중 계획과 검토 20개, 부모 계획, HTML fixture 및 관련 evidence 다섯 디렉터리를 선택 root에 반영했다. 같은 evidence 디렉터리가 기반 root에 이미 있을 때는 파일 집합과 모든 바이트가 같은지 확인한다. 새 root/site를 사용했으므로 이전 build 출력은 지우거나 덮어쓰지 않았다. Tailnet의 실제 교체 174개는 별도 pass42 backup에 보존했다.
- RSPress `2.0.22` build는 3,048 files, 129,586,338 bytes, tree SHA-256 `de0de9134aa79b6cb4bb12a79411b66ded6e57a98f2fbc10bb90ebab32b8ecbe`를 만들었다. 새 plan·plan review·Chrome 사전 비교 README·원본 PNG/JSON·fixture route가 build에 포함된다. [build log](./selected-build-pass42.log) · [build route와 tree summary](./selected-build-summary-pass42.json).
- RSPress는 dist가 docs root 하위가 아니라는 경고를 출력했지만, pass42는 전용 새 outDir을 사용했다. build는 완료됐고 pass41 site를 대상으로 쓰지 않았다.

## Tailnet 복사와 hash 검증

- Preview URL은 [`https://macstudio.tailed42f2.ts.net/spinon/`](https://macstudio.tailed42f2.ts.net/spinon/)이고 로컬 대상은 `/Users/yoonhb/Library/Application Support/SpinonPreview`다. 기존 server PID `1191` (`127.0.0.1:8768`)를 유지했다.
- Preflight에서 3,048개 파일을 create `196`, replace `174`, same `2,678`로 분류했다. 삭제는 없다. 교체할 기존 파일 `23,476,560` bytes는 별도 pass42 backup root에 보관한다. [Preflight](./preflight-pass42.json).
- Copy 이후 source와 target `3,048/3,048`, 이전 파일 backup `174/174`의 길이와 SHA-256이 일치했다. 동기화 중 교체 대상 파일을 잡은 비-preview process는 없었다. [복사·백업 결과](./copy-and-backup-pass42.json) · [로컬 재검증](./local-verification-pass42.json).
- Tailnet에서 3,048개 파일 모두 HTTP `200`, hash 일치 `3,048/3,048`, 실패 `0`이다. [원격 검증](./served-verification-pass42.json).
- 이 증거 디렉터리의 25개 산출물과 화면에 대한 SHA-256은 [manifest](./SHA256SUMS-pass42.txt)에 기록했다. manifest는 자기 자신을 포함하지 않는다.

## Agent Browser 화면 확인과 종료

전용 `spinon-tailnet-pass42` session으로 public roadmap, status, 새 계획, 사전 비교 README와 Chrome PNG route를 열었다. roadmap/status에는 현재 `72/213`와 기존 merged calc 항목이 표시되고, 새 계획 route에는 네 scene·독립 기대값·runtime 미시작 경계가 표시된다. evidence README에서는 네 scene과 root 바깥 overflow 표본을 확인했다. Tailnet Chrome PNG는 `complete=true`, `1280×633`이었다.

- [Tailnet roadmap 화면](./tailnet-roadmap-agent-browser-pass42.png) · [roadmap DOM](./roadmap-dom-pass42.json)
- [Tailnet status 화면](./tailnet-status-agent-browser-pass42.png) · [status DOM](./status-dom-pass42.json)
- [새 계획 화면](./tailnet-plan-agent-browser-pass42.png) · [plan DOM](./plan-dom-pass42.json)
- [사전 비교 evidence 화면](./tailnet-evidence-agent-browser-pass42.png) · [evidence DOM](./evidence-dom-pass42.json)
- [Chrome PNG route 화면](./tailnet-chrome-image-agent-browser-pass42.png) · [image DOM과 크기](./chrome-image-dom-pass42.json)
- 확인 뒤 전용 브라우저를 닫았다. 후속 목록은 활성 Agent Browser session 0개, Agent Browser 및 소유 Chrome process 0개다. 별도 세션은 종료하지 않았고 Tailnet server PID `1191`은 유지했다. [종료 결과](./browser-close-pass42.json).

## 범위와 한계

이 pass는 private Tailnet 개발 미리보기와 byte/hash 검증이다. production Pages 배포가 아니며, 새 계획 slice의 Spinon runtime, Android/iOS Simulator, 실기기, CSS border stroke, WPT 또는 전체 C11.4 결과를 검증하지 않았다. public `roadmap.html`과 `STATUS.md`를 수정하지 않았고, 새 plan은 preview에서 직접 route로 확인했다. 계획 단계의 Chrome 156 관측만 유지하며 공식 완료 수는 바꾸지 않는다.

첫 선택-root 준비 실행은 pass41 기반 root에 이미 포함된 cyclic-calc evidence를 새 파일처럼 취급해 멈췄다. Tailnet에는 쓰지 않았고 삭제도 하지 않았다. 재실행 가능하도록 기존 evidence의 파일 집합·내용이 원본과 동일한 경우만 재사용하도록 고쳐 실행했으며 선택 root와 build 검증은 통과했다.
