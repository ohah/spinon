# PR #1318 병합 후 공식 상태·로드맵·Tailnet 동기화

구현 PR [#1318](https://github.com/ohah/spinon/pull/1318)을 Rebase and merge한 뒤 상태 대장, 내부 Grid 계약, 계획과 공개 roadmap에 병합 결과를 기록했다. C11.4/C11 공식 체크는 미완료이며 완료 집계 `78/214`, 내부 계약 `0.1.0`은 그대로다.

## source와 문서 반영

- 구현 PR #1318의 merge SHA는 `e9f7f4abf32619ccf2739d077c5c8db16ff35f13`이다. GitHub merge 시각은 `2026-11-09T21:45:22Z`, PR check rollup은 비어 있었다.
- 문서 source commit은 `32b9774f67b29ed86fe3e0f7be91869d62930064`, tree는 `69ea5e96b5a42d3f4a20e766fdb5b5031fff6db0`다.
- `spec/STATUS.md`, 계획, 내부 계약 0059, 공개 roadmap에 bounded 지원 범위와 PR metadata를 반영했다. 다음 slice는 선택 전으로 표시하고 완료 수를 올리지 않았다.
- 실행 근거는 [구현 evidence](../c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-simulators-2026-11-10/README.md)에 연결했다. RuntimeSession 5/5, 기존 `min-height:0` 2,880 scenes와 `min-height:auto` 2,880 scenes, feature-gated floor trace, Android API 37.2 ARM64 read-only AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 light/dark draw smoke 기록을 확인할 수 있다. Android는 llvmpipe software Vulkan이며 PR checks는 없었다.

## 선택 빌드와 Tailnet 제공

- RSPress `2.0.22`로 source file 53개를 선별해 route 135개, `18,906,456` bytes를 만들었다. 전체 문서 dead-link validation은 수행하지 않았다.
- 게시 전 Tailscale backend는 Running, preview server PID `90711`은 `127.0.0.1:8768`에서 정상 응답했다. 서버를 재시작하지 않았다.
- Preflight에서 135 route 중 생성 48, 교체 37, 동일 hash 50개를 분류했고 교체할 37개 모두 열린 file handle이 0개였다. 게시 전에 기존 37개 파일을 아래 rollback backup에 보존했다.
- Copy 뒤 local route/hash `135/135`, Tailnet HTTPS HTTP 200 및 SHA-256 `135/135`, backup `37/37`을 검증했다. 기존 `index.html`·`404.html`을 제외했고 삭제는 0건이다.
- Agent Browser `0.31.1` desktop `1440×1000`, mobile `390×844`에서 roadmap의 PR #1318, merge SHA, 실행 evidence, 상태 `78/214`, C11.4/C11 미완료를 확인했다. 두 화면 모두 document/body 가로 폭이 viewport와 같았고, mobile kanban 폭은 362px였다. 인라인 plan link의 bounding-box 중앙 locator 시도는 옆 PR 링크 영역에 걸렸지만, 실제 글자 조각 위치를 pointer로 눌러 plan route에 진입했고 제목·구현 결과·범위 문구를 확인했다.
- Agent Browser session을 닫고 전용 프로세스가 종료된 것을 확인했다. 기존 Terminal Browser는 사용하거나 종료하지 않았다. 시뮬레이터도 시작하지 않았다.

선택 build는 [summary](./selected-build-summary.json), source는 [root manifest](./selected-root.json), 생성 route는 [publication manifest](./selected-publication.json), 게시 전 확인은 [preflight](./preflight.json), copy/backup은 [copy report](./copy-and-backup.json), HTTP·hash 결과는 [served verification](./served-verification.json)에 있다. 화면은 [desktop](./tailnet-roadmap-agent-browser-desktop-pr1318.png) · [mobile](./tailnet-roadmap-agent-browser-mobile-pr1318.png)이다. 기존 Tailnet 파일의 rollback backup은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1318-c11-4-2026-11-10/pass01`에 보존했다.

전체 문서 build, 일반 CSS Grid 적합성, WPT, 실기기, hardware GPU 및 전체 C11.4/C11은 이 게시나 screenshot으로 확인하지 않았다.
