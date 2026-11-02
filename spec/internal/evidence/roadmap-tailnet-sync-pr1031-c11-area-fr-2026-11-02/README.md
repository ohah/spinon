# PR #1031 병합 후 C11 로드맵 Tailnet 동기화

2026-11-02 PR [#1031](https://github.com/ohah/spinon/pull/1031)이 `Rebase and merge`된 뒤 `main` HEAD `1068f1070ddf02d2a9d625c86c0d8902479eb510`에서 선택 문서 사이트를 만들고 기존 Tailnet 미리보기에 반영했다. 이 갱신은 C11의 공식 완료 상태를 바꾸지 않으며 `72/213`을 유지한다.

## 게시 결과

- Rspress 2.0.22 선택 빌드 입력 276개, 6,137,001바이트를 준비했다. 게시 경로 385개, 총 17,073,716바이트의 합산 SHA-256은 `8912288348a809058b692c58a78838b47fe2f83e49abffc5277856c378f2ad88`이다.
- 241개 경로가 달라졌다: 새 경로 198개, 기존 경로 교체 43개, 동일 경로 144개. 교체 전 43개 파일은 별도 백업에 보존했고 파일 삭제는 하지 않았다.
- 로드맵·공식 상태·C11 계획·계약·실행 근거와 화면을 포함한 34개 지정 경로를 local `127.0.0.1:8768` 및 Tailnet URL에서 요청했다. 68개 응답이 모두 HTTP 200이며 각 경로의 바이트 수와 SHA-256이 게시 manifest와 일치한다. 세부 응답은 [`tailnet-served-verification.json`](tailnet-served-verification.json)에 있다.
- 로드맵 본문은 504,870바이트, SHA-256 `26bfd29e3fa7c542620359152f71393362831fabfc34f0775042d24b884fe72f`로 local과 Tailnet에서 일치한다. 새 항목은 PR #1030과 merge SHA `579435b1955e7ad157baa13d100d75f13a5dd283`를 표시하고 `72/213` 및 C11 미완료 설명을 보존한다.
- 게시 전 Tailscale 상태 `Running`, 미리보기 서버 PID `28040`, 변경 대상 열린 파일 없음, 기존 로드맵 local/Tailnet 해시 일치를 확인했다. 같은 PID가 게시 뒤에도 유지됐다. 서버를 재시작하지 않았다.

## Agent Browser 확인

사용자 지정 주소 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`에서 PR #1030 카드를 Agent Browser로 확인했다. 데스크톱 `1280×633`에서 문서 폭과 viewport 폭이 모두 1280이고 카드 x=20, right=1260이었다. 모바일 `390×844`에서 문서 폭과 viewport 폭이 모두 390이고 카드 x=14, right=376이었다. 두 크기 모두 merge SHA와 `72/213` 표기를 포함했다. 측정값은 [`agent-browser-observation.json`](agent-browser-observation.json)에 기록했다.

- [데스크톱 Agent Browser 화면](tailnet-roadmap-agent-browser-desktop.png)
- [모바일 Agent Browser 화면](tailnet-roadmap-agent-browser-mobile.png)

확인 뒤 해당 Agent Browser 세션을 닫았고 프로세스 목록에서 Agent Browser 프로세스가 남지 않은 것을 확인했다. 기존 Google Chrome 프로세스는 이 검증에서 열지 않았고 종료하지 않았다.

## 범위와 한계

- 선택된 385개 게시 경로와 지정 34개 URL만 검사했다. 전체 미리보기 디렉터리, 전체 문서 링크, 모든 화면 크기를 검사하지 않았다.
- 모바일 확인은 새 PR #1030 문단의 가로 경계와 표시 내용을 확인했다. 페이지의 다른 다이어그램 및 모든 반응형 레이아웃을 전수 검증한 결과는 아니다.
- PR #1030의 Android/iOS 캡처는 draw/present 시각 증거다. Android PixelCopy나 iOS Metal drawable 표면 readback 증거가 아니다.
- 이전 Tailnet 파일의 복사본은 사용자 데이터 아래 `SpinonPreviewBackups/roadmap-tailnet-sync-pr1031-c11-area-fr-2026-11-02/pass01`에 남겨 두었다.

## 재현 자료

- `prepare-selected-root.mjs`, `rspress.config.ts`, `stage-publication.mjs`, `tailnet-sync.mjs`: 선택 입력·문서 빌드·경로 검증·백업 포함 게시를 수행한다.
- `selected-root.json`, `route-publication-manifest.json`: source HEAD, 파일/경로 해시, 서버 PID, 선택 경로를 기록한다.
- `tailnet-preflight.json`, `tailnet-copy.json`, `tailnet-served-verification.json`: 게시 전 상태, 생성·교체 수, local/Tailnet 응답 확인을 기록한다.
- `selected-build.log`: diff 검사에 맞게 끝 공백만 정리한 Rspress 빌드 로그. 압축 원본은 `selected-build.log.raw.gz`, 원본·압축·정규화 파일 해시는 `selected-build-log-hashes.json`에 기록한다.
- [`pr-diff-review-30.md`](pr-diff-review-30.md): 게시 경계, 백업·해시 검증, HTTP 제공, 브라우저 확인의 실패 경로 30개를 검토한다.
