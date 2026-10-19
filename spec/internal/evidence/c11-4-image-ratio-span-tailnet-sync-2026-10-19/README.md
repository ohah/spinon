# C11.4 PR #354 병합 후 Tailnet 로드맵 동기화 근거

2026-10-19 PR [#354](https://github.com/ohah/spinon/pull/354) 병합 commit `08caa59b42f75eb2939104fffa527995eb1d4f51`을 기준으로 선택 문서와 Tailnet 미리보기를 갱신했다. C11.4는 계속 미완료이며 공식 카운터는 `72/212`다.

## 로드맵과 렌더

- [`spec/public/roadmap.html`](https://github.com/ohah/spinon/blob/main/spec/public/roadmap.html)에 PR #354의 fixed-track image-ratio 후속 설명을 추가했다. 완료 체크나 집계 값을 하드코딩하지 않았다. 로드맵은 공식 상태 대장을 읽어 카운터를 만든다.
- 렌더 입력 기준은 `origin/main`의 `08caa59b`다. 유일한 source override는 `spec/public/roadmap.html`이며 정확한 입력과 SHA-256은 [`selected-build-inputs.json`](selected-build-inputs.json)에 있다.
- 프로젝트 고정 RSPress `2.0.22`로 Markdown route 16개와 정적 로드맵을 선택 렌더했다. 최종 build output은 67개 파일, 4,361,202 bytes다. 경로·해시는 [`rendered-manifest.json`](rendered-manifest.json)에 있고 실행 log는 [`render-build.log.gz`](render-build.log.gz)에 있다.
- 이번 선택 빌드에서는 dead-link/image 검사 옵션을 끄고 지정 route만 렌더했다. 전체 문서 build나 전체 링크 적합성을 확인한 결과로 해석하지 않는다. GitHub Pages는 배포하지 않았다.

## Tailnet 파일 반영

- `tailscale serve status`에서 기본 Funnel `/`가 `127.0.0.1:8768`로 전달됨을 확인했다. 기존 preview server는 `/spinon/` 접두부를 제거하고 `SpinonPreview` 루트에서 파일을 제공한다. 중첩 `SpinonPreview/spinon` 경로를 만들지 않았다.
- 선택된 route Markdown, 기존 preview asset, C11.4 실행 근거, 렌더 결과를 합쳐 345개 파일을 관리했다. 전체 경로·원본 SHA-256은 [`copy-manifest.json`](copy-manifest.json)에 있다.
- 첫 반영은 기존 18개 파일을 별도 backup에 보존한 뒤 18개 교체, 273개 추가, 54개 동일을 확인했다. 로드맵 상단에 PR #354 note가 빠진 것을 Agent Browser에서 발견해 source note를 추가했다.
- 최종 pass 2에서는 바뀐 `roadmap.html` 한 파일을 교체하기 전에 pass 2 backup을 만들고 SHA-256을 확인했다. 같은 디렉터리 임시 파일에 기록·fsync한 뒤 원자 교체했다. 최종 345개 중 1개 교체, 344개 동일, 추가 0개, 삭제 0건이다. 이전 파일 backup은 [`backup-verification-pass-1.json`](backup-verification-pass-1.json)과 [`backup-verification.json`](backup-verification.json), copy 결과는 [`copy-verification-pass-1.json`](copy-verification-pass-1.json)과 [`copy-verification.json`](copy-verification.json)에 있다.
- 로컬 preview server의 `/spinon/` URL 345개에서 HTTP 200 및 응답 body SHA-256 345/345가 일치했다. 상세 결과는 [`served-verification.json`](served-verification.json)에 있다.

## Agent Browser 확인

Agent Browser로 실제 Tailnet 도메인에서 다음 페이지를 열었다.

- [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html): 공식 카운터 `72/212`, PR #354 설명 링크, C11.4 체크 `false`.
- [상태 대장](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html): fixed-track 후속 문단과 구현·PR diff·실행 근거 링크가 모두 `/spinon/` 안에서 열린다. C11.4 체크는 `false`다.
- [C11.4 계획](https://macstudio.tailed42f2.ts.net/spinon/plan/c11-4-grid-track-sizing.html): 5.333px/12px CRD 기대값, Chrome 155의 12px/27px 관찰, 제한 범위와 미완료 상태를 확인했다.
- [이미지 ratio span 근거](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c11-4-image-ratio-span-2026-10-19/README.html): Android 1080×2400과 iOS 1206×2622 screenshot 두 장이 모두 로드됐다.

DOM 확인과 화면 캡처는 [`browser-verification.json`](browser-verification.json)에 기록했다. [PR #355](https://github.com/ohah/spinon/pull/355) 본문도 Agent Browser로 확인했고 raw screenshot 4/4가 각각 1280×633 크기로 로드됐다. 캡처는 [로드맵 카운터](tailnet-roadmap-count-agent-browser.png), [PR #354 로드맵 항목](tailnet-roadmap-agent-browser.png), [상태 대장 후속](tailnet-status-agent-browser.png), [C11.4 미완료 체크](tailnet-status-checkbox-agent-browser.png), [계획](tailnet-plan-agent-browser.png), [실행 근거 페이지](tailnet-evidence-agent-browser.png), [Android/iOS Simulator 화면](tailnet-evidence-simulator-agent-browser.png)이다. 사용한 Agent Browser session은 작업 후 종료했고 결과는 [`browser-session-shutdown.json`](browser-session-shutdown.json)에 있다.

## 검증 범위

이 동기화는 선택 문서 렌더, 기존 Tailnet preview file update, 브라우저 화면과 URL 확인만 검증한다. 실제 기기 동작이나 새 simulator 실행을 주장하지 않는다. PR #354의 근거에서는 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 제한 9-scene matrix를 기록했으며, multi-track/gutter, 다른 writing mode, percentage/calc 조합, PNG texel paint, WPT 및 전체 CSS Grid 적합성은 남아 있다. C11.4 체크와 공식 완료 수는 바꾸지 않았다.

[`pr-diff-review-30.md`](pr-diff-review-30.md)는 이번 roadmap/evidence diff의 30개 실패 관점 검토다. [`SHA256SUMS`](SHA256SUMS)는 evidence directory의 제출 파일을 검증한다.
