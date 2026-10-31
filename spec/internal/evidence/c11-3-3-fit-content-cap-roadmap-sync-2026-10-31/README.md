# C11.3.3 fit-content cap 공개 로드맵 링크 갱신

## 변경

공개 roadmap의 PR #911 카드에 병합된 `fit-content()` cap 계획과 Chrome 156 사전 비교 링크를 추가한다. 공식 상태 대장과 `72/213`은 변경하지 않는다. 두 링크는 계획 단계와 Chrome-only 기준을 가리키며 Spinon 구현을 주장하지 않는다.

## 문서 빌드와 브라우저 확인

전체 docs 빌드는 변경 범위 밖의 기존 dead-link 항목에서 실패했다. 따라서 전체 문서 링크 검사를 통과했다고 기록하지 않는다. C11.3.3 카드에 필요한 페이지와 연결 파일만 별도 root에 모아 RSPress `2.0.22`로 빌드했다. [선택 빌드 로그](selected-build-pass03.log.gz)는 성공했고 생성 HTML 17개, 선택 게시 route 57개를 기록했다. 선택 root와 게시 목록은 각각 [manifest](selected-root-pass03.json), [manifest](selected-publication-pass04.json)에서 확인할 수 있다.

Agent Browser `0.31.1` / Chrome `156.0.0.0`에서 후보 roadmap을 확인했다. Desktop viewport는 1280×900, mobile viewport는 390×844 CSS px다. mobile `document.scrollWidth`는 390px, 대상 카드 폭은 362px이며 가로 넘침이 없다. 계획·사전 비교·Chrome README 및 PNG route는 모두 HTTP 200이다. Chrome PNG의 원본 크기는 1280×633이다. [Desktop 카드 화면](roadmap-card-agent-browser-pass05.png) · [Mobile 카드 화면](roadmap-card-mobile-agent-browser-pass03.png) · [Mobile 측정값과 href](mobile-layout-pass03.json) · [선택 route 상태](route-validation-pass01.json).

## Tailnet 게시 경계

검사 시점의 Tailscale backend는 `Running`; 기존 preview 서버 PID 28040은 `127.0.0.1:8768`에서 수신 중이었다. 선택 게시 preflight는 route 57개, 8,469,799 bytes이며 create 35, replace 7, same 15로 분류했다. 바뀔 대상의 열린 파일은 0개였고 삭제는 없다. [Preflight 결과](preflight-pass04.json)와 [동기화 helper](sync-tailnet-pass04.mjs)는 복사 직전 열린 파일 및 main roadmap hash를 다시 검사하고, 교체 전 파일을 백업한 뒤 Tailnet HTTP 응답·SHA-256을 검증하도록 고정했다.

이 증거를 만들 때에는 PR 병합 전이므로 Tailnet 파일을 아직 복사하지 않았다. 이 README의 사후 URL·hash 검증 완료 표시는 실제 게시 후에만 할 수 있다.
