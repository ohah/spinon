# C11.4 구현 병합 후 Tailnet 문서 동기화 근거

2026-10-19 PR #335 병합 후 선택 문서와 정적 로드맵을 다시 렌더링해 기존 Tailnet 미리보기 파일에 반영했다. C11.4는 계속 미완료이며, 공식 로드맵 수는 `72/212`로 유지됐다.

## 렌더 범위

- 기준 commit: `5cff677cb6dce5a96bdbeb083a8ae566c04da30a` (PR #335 Rebase and merge)
- RSPress `2.0.22`로 Markdown route 13개와 정적 roadmap 1개를 포함해 선택 빌드했다. 최종 출력은 42개 파일이며 전체 문서 build나 dead-link 검사 결과로 확대 해석하지 않는다.
- 빌드 명령, 최종 log, 선택 범위 및 링크 수정은 [`render-build-result.md`](render-build-result.md)에 있다.

## 링크 수정

Agent Browser로 최종 Tailnet 페이지를 확인하다가 `STATUS.html`의 C11.4 계획 링크가 `/spinon/` 경로 밖으로 풀리는 문제를 찾았다. [`spec/STATUS.md`](https://github.com/ohah/spinon/blob/main/spec/STATUS.md)의 계획 링크를 canonical GitHub blob URL로 수정했다. 사전 비교 문서와 inline edge 근거의 fixture, font, image, plan 링크도 canonical GitHub URL을 사용한다. 수정 전후의 선택 빌드 및 Tailnet 반영 결과는 pass별 JSON과 log에 기록했다.

## Tailnet 반영

미리보기 서버는 `/spinon/` URL 접두부를 제거하고 기존 preview 루트에서 파일을 읽는다. 첫 pass 4 시도에서 경로를 한 단계 더 붙여 새 파일 72개를 미서빙 하위 디렉터리에 썼다. 서버 매핑과 `lsof` 결과를 확인한 뒤 pass 4 preflight가 모두 새 파일로 기록한 항목만 해시 대조해 제거했다. 기존 preview 파일은 그 시도에서 바뀌지 않았다. 이 경로 수정 기록은 [`path-correction.json`](path-correction.json)에 있다. 올바른 루트에는 다음 순서로 다시 반영했다.

1. 정확히 선택한 72개 파일의 이전 존재 여부와 SHA-256을 기록했다.
2. 바뀔 기존 15개 파일을 먼저 백업하고 백업 SHA-256 15/15 일치를 확인했다.
3. 임시 파일 작성 후 원자적으로 교체했다. 새 파일 14개, 교체 15개, 기존과 동일한 파일 43개이며 삭제는 0건이다.
4. 72개 URL에서 HTTP 200과 제공된 바이트의 SHA-256 72/72 일치를 확인했다.

상세 기록은 [`preflight.json`](preflight.json), [`backup-verification.json`](backup-verification.json), [`copy-verification.json`](copy-verification.json), [`served-verification.json`](served-verification.json)에서 확인할 수 있다. Tailnet preview 서버 자체는 기존 사용 중인 프로세스라 중지하지 않았다.

## Agent Browser 확인

- 로드맵 화면에 공식 완료 `72/212`가 표시되고, 상태 대장에서 C11.4 체크박스가 해제된 상태임을 확인했다.
- 상태, 계획, 사전 비교, inline edge, text intrinsic 근거 페이지를 확인했다. 사전 비교 페이지는 26개 case와 fixture/font/image/plan 링크를 표시한다.
- 최종 화면 캡처는 [`tailnet-roadmap-agent-browser.png`](tailnet-roadmap-agent-browser.png), [`tailnet-status-agent-browser.png`](tailnet-status-agent-browser.png), [`tailnet-plan-agent-browser.png`](tailnet-plan-agent-browser.png), [`tailnet-precomparison-agent-browser.png`](tailnet-precomparison-agent-browser.png), [`tailnet-inline-edge-agent-browser.png`](tailnet-inline-edge-agent-browser.png), [`tailnet-evidence-agent-browser.png`](tailnet-evidence-agent-browser.png)에 있다.
- 세부 URL과 화면 확인 결과: [`browser-verification.json`](browser-verification.json). Agent Browser 종료 및 프로세스 부재 확인: [`browser-session-shutdown.json`](browser-session-shutdown.json).

## 판정 범위

이 변경은 문서 렌더링·Tailnet 미리보기 동기화만 다룬다. iOS/Android 앱 동작, 실제 기기, 시뮬레이터, 전체 CSS Grid 지원 또는 전체 WPT 적합성을 검증하지 않는다. GitHub Pages는 배포하지 않았다.

PR 변경 적대 검토 30개 관점은 [`pr-delta-review-30.md`](pr-delta-review-30.md)에 있다. 이 디렉터리의 `SHA256SUMS`는 나머지 증거 파일의 SHA-256 manifest다.
