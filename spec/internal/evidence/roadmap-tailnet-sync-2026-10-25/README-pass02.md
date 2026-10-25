# Tailnet 로드맵 동기화 · pass02

2026-10-25 main `489dabc158b1ed55c59d023027070e3662829a7f`을 기준으로 C11.4 PR #615 병합 기록이 포함된 선택 문서 사이트를 빌드하고 미리보기 경로를 동기화했다. 공식 C11.4 상태는 미완료, 집계는 `72/213`이다.

## 빌드와 동기화

- RSPress 선택 빌드는 19개 생성 페이지, 620개 선택 route, 총 27,768,061 bytes다. 선택 root, 생성 route manifest와 빌드 로그는 같은 source head를 기록한다.
- 첫 빌드 명령은 저장소 루트에 없는 `node_modules/.bin/rspress`를 호출해 시작 전에 실패했다. `bun install --frozen-lockfile`은 211개 설치/229개 패키지를 검사해 lockfile 변경 없이 종료했다. 그 뒤 `packages/docs/node_modules/.bin/rspress`로 재실행해 빌드했다. 첫 실패 로그와 성공 로그([gzip 원본](selected-build-pass02-retry01.log.gz), SHA-256 `a8db67b63172ab87720a29a0fa221d2ba1a3276f171d63a19fe9880b5b469b69`)를 보존한다. 성공 로그는 공백을 수정하지 않고 전송 크기만 줄였다.
- Rspress가 참조하는 C11.3.3 페이지의 sibling PNG 세 파일은 selected root에 해시가 같은 파일로 연결했다. [복구 manifest](selected-root-repair-pass02.json)
- `tailnet-preflight-pass02.json`: 기존 route 590 same, 27 replace, 새 route 3 create, 삭제 0. 교체 전 기존 파일 12,369,896 bytes를 pass02 백업 root에 보존한다.
- 복사 전 교체 대상 27개에 대한 `lsof` 결과는 모두 닫힌 상태다. [open-handle 검사](target-open-handles-pass02.json)
- 복사 후 local preview 620/620, backup 27/27, Tailnet HTTP 200 및 remote body SHA-256 620/620을 확인했다. [복사 결과](tailnet-copy-pass02.json) · [전체 hash 검증](tailnet-verify-pass02.json)
- `no_deletes: true`; 기존 파일 삭제나 pass01 백업 덮어쓰기는 없다. pass02 백업은 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-25/pass02`에 있다.

## Agent Browser 확인

[Agent Browser 검증](tailnet-agent-browser-pass02.json)에서 Tailnet roadmap이 PR #615의 병합 상태·SHA `0ccec669e5506b6caef53314702c7fb60090e45c`와 공식 `72/213`을 표시하고 C11.4 미완료를 유지하는지 확인했다. 계획 페이지와 새 30-view review 페이지도 열어 merge SHA, #615 및 리뷰 표 30행을 확인했다. [로드맵 화면](tailnet-roadmap-agent-browser-visible-pass02.png)

이 동기화 PR의 별도 실패 관점 30개 검토는 [PR diff review](pr-diff-review-30-pass02.md)에 기록했다.

PR #618 본문에서도 raw screenshot link를 Agent Browser로 열어 1280×633 이미지가 실제 로드되는지 확인했다. 잘못 입력한 첫 SHA는 404였고, 실제 HEAD `d87b6eda7b00f8a6c771fa32b9686c7c1dbf4dfd`로 고친 뒤 로드에 성공했다. 확인 후 Agent Browser 세션을 닫았으며 `pgrep -l agent-browser`에 프로세스가 남지 않았다. Tailnet은 온라인이고 `127.0.0.1:8768` preview listener는 계속 동작한다.
