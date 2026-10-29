# Tailnet roadmap sync helper 검토 20개

**검토 대상:** `prepare-selected-root.mjs`, `complete-selected-build.mjs`, `sync-tailnet.mjs`, RSPress 설정. 이 스크립트는 요청된 `/spinon/` 개발 미리보기의 선택 경로만 복사하며 GitHub Pages를 배포하지 않는다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 기존 임시 root나 build output을 무심코 지운다. | 두 경로 모두 존재하면 즉시 중단하며 선택된 고유 경로를 사용한다. |
| 2 | RSPress가 기본 docs/public tree 전체를 복사한다. | `publicDir: false`이며 허용된 Markdown root와 static file만 선택한다. |
| 3 | 선택 source에 예상하지 못한 symlink가 들어온다. | 각 source path를 `lstat`해 symlink와 비정규 파일을 거부한다. |
| 4 | URL 경로가 선택 root를 벗어난다. | 목적 경로가 선택 root 하위인지 확인하고 `.`·`..`·절대 경로·역슬래시를 거부한다. |
| 5 | alias가 중복돼 다른 문서를 덮어쓴다. | 각 목적 파일은 생성 전 존재 여부를 검사하고 `wx`로 생성한다. |
| 6 | 선택 source가 복사 중 변경된다. | source와 목적물의 SHA-256을 파일마다 대조한다. |
| 7 | build output에 필요한 screenshot/JSON route가 빠진다. | 선택된 14개 문서·정적 route를 build 완료 시 모두 확인한다. |
| 8 | `.html` 또는 정적 파일 복사로 기존 404 route가 교체된다. | manifest에서 `404.html`을 명시적으로 제외하고 Tailnet에 포함하지 않는다. |
| 9 | 사전 확인하지 않은 목적 파일을 덮어쓴다. | preflight가 각 URL의 존재 상태·이전 hash·새 hash를 고정하며 copy에서 재검증한다. |
| 10 | 다른 서버가 8768 포트에서 대상 서버처럼 보인다. | 정확히 한 listener와 `tailnet-preview-server.mjs` 실행 인자를 요구한다. |
| 11 | 검증되지 않은 네트워크 listener를 허용한다. | 프로세스의 유일한 socket이 `127.0.0.1:8768` LISTEN인지 확인한다. |
| 12 | 사용 중인 기존 대상 파일을 교체한다. | 교체 전에 `lsof`를 조회하고 예상 preview server 외 process가 열었으면 중단한다. |
| 13 | 교체 파일을 백업하지 않고 잃는다. | 이전 bytes를 새 대상 경로와 같은 상대 구조로 `wx` 저장하고 size/hash를 확인한다. |
| 14 | backup 디렉터리의 과거 결과를 덮어쓴다. | copy pass는 기존 backup root를 재사용하지 않는다. |
| 15 | 실패한 route를 지우거나 대상 tree를 정리한다. | 모든 모드가 `no_deletes: true`를 기록하며 삭제 명령을 호출하지 않는다. |
| 16 | 부분 파일이 preview에 보인다. | 고유 임시 파일에 쓴 뒤 같은 filesystem 내 `rename`으로 교체한다. |
| 17 | preflight와 copy 사이에 source 또는 target이 바뀐다. | copy 직전 양쪽의 이전 상태·size·SHA-256이 preflight와 다르면 중단한다. |
| 18 | localhost 응답만으로 Tailnet 페이지가 최신이라고 주장한다. | remote verify는 지정 origin의 각 경로에서 HTTP 200, 바이트 수와 SHA-256 일치를 요구한다. |
| 19 | backup 존재만 보고 보존에 성공했다고 처리한다. | verify 모드가 각 교체본 backup의 원본 size와 hash를 다시 대조한다. |
| 20 | 개발 미리보기를 공식 Pages로 배포한다. | 스크립트는 local preview file root만 갱신하며 Pages workflow, gh-pages, 서버 재시작을 호출하지 않는다. |

## 판정

20개 경로 안전·원본 보존·원격 확인 관점을 검토했다. 전체 적용 결과는 preflight, copy, remote hash report와 Agent Browser 캡처에 기록한다.
