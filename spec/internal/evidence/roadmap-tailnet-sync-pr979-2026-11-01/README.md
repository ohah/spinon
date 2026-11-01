# PR #979 Tailnet roadmap preview sync

## 범위

PR [#979](https://github.com/ohah/spinon/pull/979)의 C11.4 `hyphens` 진행 카드를 기존 Tailnet 문서 미리보기에 반영했다. 이 게시 작업은 구현 상태를 변경하지 않는다. C11과 C11.4는 미완료이며 공식 완료 수는 `72/213`이다.

출처는 main commit `39f57f7a6eed4ddd49e06836e89579d0514cc591` (tree `05ad3c22259547128b462a4f6ce0e99ea4b68597`)이다. 게시 범위는 저장소 전체 문서가 아니라 카드와 연결된 자료를 포함한 선택 경로다.

## 실행 결과

- 선택 root: 271개 파일, 9,549,482 bytes.
- 정적 publication: 350개 경로, 19,017,880 bytes.
- 사전 대조: 생성 179, 교체 29, 동일 142. 바뀔 파일의 열린 handle은 0개였다.
- 교체 전 파일 29개를 별도 backup 경로에 보존하고 SHA-256을 다시 확인했다.
- preview root의 파일과 publication hash가 350/350 일치했다.
- 공개 HTTPS URL에서 350/350 경로가 HTTP 200 및 기대 SHA-256과 일치했다.
- 어떤 경로도 삭제하지 않았고, preview server를 재시작하지 않았다.

사전 검사 시 Tailscale backend는 `Running`, 기존 preview listener PID는 `28040`이었다. 동기화 후 2026-11-01 09:41:18 UTC 확인에서도 backend는 `Running`이고 같은 PID `28040`이 port 8768을 듣고 있었다. preflight와 사후 관찰의 PID가 같아 같은 listener가 계속 사용된 것을 확인했다.

Agent Browser로 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html?verify=pr979-2026-11-01`을 열어 페이지 제목과 최신 `PR #977` 카드, `72/213` 문구를 확인했다. 페이지 viewport는 1280px이었다. 확인 후 `agent-browser close --all`을 실행했고 Agent Browser 및 전용 Chrome 프로세스가 남지 않았음을 확인했다.

## 범위와 한계

이 작업은 선택된 350개 경로만 게시했다. 전체 문서 사이트의 링크 검증이나 전체 배포를 주장하지 않는다. 기존 server가 파일을 열고 있어 C11 상위 계획의 generated HTML 및 raw Markdown alias 두 경로는 선택 publication에서 제외했고, 최신 카드가 직접 연결한 계획과 근거 경로는 모두 포함했다. `selected-publication.json`의 `fullDocsLinkValidation`도 `false`다.

`localHashVerified`는 preview root의 로컬 파일을 publication과 비교한 수치다. 공개 경로의 실제 제공 여부는 별도 HTTPS 200·hash 350/350 결과가 뒷받침한다.

## 근거 파일

- `selected-root.json`: source commit/tree와 선택된 271개 입력 manifest.
- `selected-publication.json`: RSPress 산출물, 350개 경로 및 카드 링크 검사.
- `preflight.json`: server 상태, 사전 route 상태, 조치 분류와 열린 handle 검사.
- `copy-and-backup.json`: 생성·교체·동일 경로 집계, 이전 내용 hash.
- `served-verification.json`: 로컬 hash, HTTPS 상태/hash, backup 및 사후 server PID 관찰.
- `prepare-selected-root.mjs`, `stage-publication.mjs`, `sync-tailnet.mjs`, `rspress.config.ts`: 실행 절차와 선택 빌드 설정.
- `pr-diff-review-30.md`: 이 증거 PR의 별도 실패 관점 검토.

실행이 끝난 뒤 검토 중 발견한 사전 대조 경합을 줄이기 위해 `sync-tailnet.mjs`를 보강했다. copy 직전 target의 존재 여부가 preflight와 다르면 중단하며, verify 때 Tailscale 상태와 preview PID가 preflight와 같은지 확인한다. 이 보강은 이미 끝난 pass01 결과를 다시 실행한 것으로 간주하지 않는다. pass01의 동작 결과는 위 JSON manifest에 보존되어 있고, 같은 PID의 사후 관찰은 별도로 기록했다.
