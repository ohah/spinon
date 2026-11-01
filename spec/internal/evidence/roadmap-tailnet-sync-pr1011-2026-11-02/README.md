# PR #1011 병합 후 Tailnet 로드맵 동기화 근거

## 범위

기준 소스는 PR #1012 병합 이후 `origin/main`과 같은 `fbf2c69125ef5242b43616ff817556e65f5057c0`이다. C11 구현이나 공식 상태는 변경하지 않았다. C11 상위는 미완료이며 공식 집계 `72/213`을 유지한다. 이 동기화는 로컬 문서 미리보기에서 이미 서비스 중인 기존 Tailnet 서버로 선택 문서 route만 게시한 작업이다.

## 게시 결과

- Tailnet: `macstudio.tailed42f2.ts.net`, 검증 시점의 Tailscale 상태 `Running`; 기존 미리보기 서버 PID `28040`을 유지했다.
- 게시 root: 기존 `SpinonPreview` 디렉터리. 기준 스냅샷은 91,348 route, 4,136,831,023 bytes였다.
- 선택 게시물: 156 route, 9,304,119 bytes. 그중 변경 127개(신규 106, 기존 교체 21), 동일 29개다. 교체 전 21개 파일은 별도 사용자 지원 백업 디렉터리에 저장하고 전부 이전 SHA-256과 대조했다.
- 기존 열람 중인 파일은 읽기 전용 서버 핸들 하나뿐이었다. 같은 디렉터리에 임시 파일을 쓴 뒤 atomic rename으로 교체했다.
- 기존 route 삭제는 하지 않았다. 게시 후 91,454 route이며 기준 route 중 사라진 경로는 0개다.
- 게시 선택 route 묶음 SHA-256: `76ca8bf5b4ffa60df24c9f0f0d045838531edd62d0abb30289411e60a7ab11ce`.

세부 route·크기·해시 및 단계별 결과는 같은 디렉터리의 `route-publication-manifest-postmerge-public.json`, `tailnet-preflight-postmerge-public.json`, `tailnet-copy-postmerge-public.json`, `tailnet-served-verification-postmerge-public.json`에 있다. 사용자 홈의 절대 경로와 로컬 전체 스냅샷은 공개 증거에서 제외했다. 검증에 사용한 로컬 스냅샷은 그대로 보존 중이다.

변경 자료를 별도로 대조한 [PR 변경 검토 30개](pr-diff-review-30.md)와 공개 증거 파일의 [SHA-256 목록](SHA256SUMS.txt)을 함께 둔다.

## HTTP 및 화면 확인

- 로드맵, 공식 상태 HTML/Markdown, C11 계획, 실행 근거, Android/iOS 캡처 등 필수 15 route를 localhost와 Tailnet HTTPS에서 각각 요청했다. 30개 응답 모두 HTTP 200이고, 각 route의 local/Tailnet 응답 SHA-256이 같으며 선택 게시 원본 해시와도 일치했다.
- Agent Browser에서 실제 Tailnet 로드맵을 데스크톱 1280×633과 모바일 390×844로 열었다. 두 화면에서 PR #1011의 merge SHA와 공식 `72/213`을 확인했다. 모바일 `documentElement.scrollWidth`는 viewport 폭과 같은 390px이었다.
- 캡처: [데스크톱](tailnet-roadmap-pr1011-agent-browser-desktop.png), [모바일](tailnet-roadmap-pr1011-agent-browser-mobile.png).
- 화면 확인 후 `agent-browser close --all`을 실행했고, `agent-browser` 및 `chrome-headless-shell` 프로세스 조회 결과는 비어 있었다.

## 제한과 보존 사항

- 이 HTTP 확인은 선택한 156 route의 파일 해시와 필수 15 route의 실제 HTTP 전달을 확인한다. 91,000여 개 전체 route의 모든 파일 내용을 다시 비교한 전체 미러 검사는 아니다. 기준 경로 삭제 여부와 게시된 route의 내용은 별도로 확인했다.
- PR #1012의 선택 RSPress build와 [전체 문서 build 진단](full-docs-build-diagnostics.md)은 그 PR의 근거를 따른다. 저장소 전체 dead-link 검사를 통과했다고 주장하지 않는다. PR #1011의 GitHub check run도 보고되지 않았다.
- 기존 미리보기 원본과 사용자 백업은 삭제하거나 교체하지 않았다. 작업 중 만든 `build/spinon/roadmap-tailnet-sync-pr1011-20261102/postmerge/baseline`(4.1 GB) 및 부분 복사 `.../publication`(2.8 GB)은 로컬 임시 복사본으로 남겨 두었으며 이 PR에는 포함하지 않았다.
