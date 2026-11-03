# C11 fit-content 반영 Tailnet 로드맵 동기화 근거

2026-11-03에 PR [#1094](https://github.com/ohah/spinon/pull/1094) 병합본을 기준으로 승인된 Tailnet 문서 미리보기를 갱신했다. 이 증거는 로드맵 게시 상태를 확인하며 C11 기능 완료를 뜻하지 않는다.

## 입력과 게시 범위

- 원본 커밋: `0326850ceb20ee19cea81ae5e0df1080cc018063` (PR #1094 merge SHA)
- 원본 트리: `36844d6e5afdad46d1ce4f744335071836c7dc7e`
- tracked diff SHA-256: 빈 diff (`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`)
- 선택 입력: Git index에서 추린 111개 tracked source file, 5,587,717 bytes; symlink 없음
- RSPress 선택 빌드 산출: 198 route, 15,835,207 bytes
- 공식 상태: `72/213`; C11 체크박스 미완료

선택 빌드는 RSPress 2.0.22로 완료했다. 전체 문서 저장소 빌드의 성공으로 확대 해석하지 않는다. 선택 입력과 게시 route 목록은 [`selected-root.json`](./selected-root.json), [`selected-publication.json`](./selected-publication.json)에 있다.

## 동기화와 제공 확인

Tailnet preflight에서 기존 로컬 및 HTTPS roadmap 모두 HTTP 200과 같은 SHA-256을 반환했다. Tailnet backend는 실행 중이었고 미리보기 서버 PID `28040`을 유지했다. 문서 static preview PID `33319`도 유지했으며 둘 다 재시작하지 않았다.

198개 route 중 169개를 생성하고, 11개를 교체하고, 18개는 동일함을 확인했다. 대상 파일 삭제는 없었다. 교체 전 백업은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-c11-fit-content-2026-11-03/pass01`에 보존했다. 교체 대상 파일을 열고 있는 프로세스는 0개였다. 상세 결과는 [`preflight.json`](./preflight.json)과 [`copy-and-backup.json`](./copy-and-backup.json)에 있다.

동기화 뒤 HTTPS의 198개 route 확인이 완료됐다. 핵심 7개 route는 로컬 미리보기와 HTTPS 미리보기 모두 HTTP 200이며, 바이트 수와 SHA-256이 일치했다. 검증 기록은 [`served-verification.json`](./served-verification.json)에 있다. 로드맵의 PR #1093 merge SHA와 계획·계약·실행 근거 링크를 실제 Agent Browser에서 확인했다. 공식 `72/213` 집계와 C11 미완료 상태도 페이지에 남아 있다.

## 실제 페이지 화면

Agent Browser에서 Tailnet URL `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`을 열어 확인했다. 문서 너비가 데스크톱 viewport 1440px, 모바일 viewport 390px와 각각 같아 수평 overflow가 없었다. 확인 후 Agent Browser 세션과 남은 daemon을 닫았고, Tailnet/static preview 서버는 보존했다.

| 뷰포트 | 문서 너비 | 스크린샷 |
| --- | ---: | --- |
| 데스크톱 1440×1000 | 1440px | [tailnet-roadmap-desktop.png](./tailnet-roadmap-desktop.png) |
| 모바일 390×844 | 390px | [tailnet-roadmap-mobile.png](./tailnet-roadmap-mobile.png) |

## 범위와 해석

- 이 동기화는 PR #1094에 반영된 C11.1 제한 `fit-content(px)` 변경과 official status를 게시한다.
- 공식 구현 집계는 `72/213`으로 유지했고 C11 및 C11.1 완료 체크를 추가하지 않았다.
- `fit-content()` 전체, 일반 author CSS admission, 전체 CSS Grid, WPT 적합성 또는 실기기 검증을 주장하지 않는다.
- 스크린샷 및 주요 manifest hash는 [`artifact-hashes.sha256`](./artifact-hashes.sha256)에 기록했다.
