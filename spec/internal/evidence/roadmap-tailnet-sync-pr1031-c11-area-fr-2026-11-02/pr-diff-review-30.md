# PR 변경 적대 검토 30개 관점

검토 대상은 이 디렉터리의 Tailnet 선택 배포 스크립트, 실행 보고서, 로드맵 화면과 설명이다. 실제 로컬/Tailnet 응답 및 Agent Browser 결과도 대조했다.

| # | 실패 관점 | 결과 |
|---:|---|---|
| 1 | 선택 입력이 merge 전 후보 commit에서 만들어질 수 있는가 | `HEAD == origin/main == 1068f1070ddf02d2a9d625c86c0d8902479eb510` 검사를 두 staging 단계에서 확인한다. |
| 2 | 다른 브랜치 또는 tracked WIP를 출처로 삼을 수 있는가 | 두 단계가 `main`과 tracked-clean 상태를 요구한다. 현재 증거는 해당 merged HEAD에서 생성했다. |
| 3 | source provenance 없이 다른 파일 집합을 게시할 수 있는가 | selected-root와 route manifest에 source HEAD/tree, 원본 경로, 바이트, SHA-256을 기록한다. |
| 4 | 기존 build 또는 report를 덮어쓸 수 있는가 | build root와 manifest가 이미 존재하면 중단한다. 이번 출력은 고유한 pass01 경로에서 생성했다. |
| 5 | 경로 입력으로 상위 디렉터리를 탈출할 수 있는가 | 게시 route의 절대 경로, 역슬래시, 빈·`.`·`..` segment를 거부한다. |
| 6 | source symlink를 따라 예상 밖 파일을 복사할 수 있는가 | source는 regular non-symlink 파일만 허용하고 evidence tree에서는 symlink를 거부한다. |
| 7 | 폴더 순서 변화로 같은 source manifest가 불안정해지는가 | directory entry를 정렬해 순회하고 결과 합산도 정렬 route 순서로 계산한다. |
| 8 | 특수 파일·예상 밖 evidence 디렉터리가 조용히 들어가는가 | 알려진 scratch 디렉터리만 제외하며 다른 non-file entry는 오류로 중단한다. |
| 9 | 선택 복사본에 실행물·불필요한 artifact가 우발적으로 섞이는가 | evidence 복사 확장자를 명시적으로 제한했다. Rspress 산출 정적 route는 별도 build output에서 수집한다. |
| 10 | source에서 selected root로 복사하는 도중 내용이 달라지는가 | 복사 직후 byte count와 SHA-256을 다시 비교한다. |
| 11 | source file count/size를 실제와 다르게 보고하는가 | 276개 파일, 6,137,001바이트가 manifest에 남고 route manifest와 source HEAD가 연결된다. |
| 12 | Rspress 업그레이드가 문서 결과에 숨어 들어갈 수 있는가 | 저장소의 고정된 Rspress 2.0.22를 사용하고 변경하지 않았다. |
| 13 | public 디렉터리 내용이 선택 경로 밖으로 자동 유입되는가 | 복제 config가 `publicDir: false`를 설정해 선택 목록 외 정적 파일 복사를 막는다. |
| 14 | 이전 build 파일이 새 출력에 섞일 수 있는가 | `cleanDistPath: true`로 build 출력 경로를 정리한다. build root 자체도 이 실행 전 비어 있음을 보장했다. |
| 15 | 같은 route에 서로 다른 Rspress/선택 파일 내용이 충돌할 때 한쪽을 덮어쓰는가 | 같은 경로의 기존 staged hash와 새 hash가 다르면 staging을 실패시킨다. |
| 16 | preview root가 심볼릭 링크를 통해 예상 밖 위치를 가리키는가 | root가 실제 디렉터리인지와 `realpath`가 지정 경로와 같은지 확인한다. |
| 17 | Tailnet이 꺼졌거나 다른 상태에서 게시를 계속하는가 | preflight와 후속 단계가 Tailscale backend `Running`을 요구한다. |
| 18 | 8768 포트에 여러 프로세스 또는 다른 앱이 응답하는가 | 리스너 PID가 하나인지, 실행 인자에 `tailnet-preview-server.mjs`가 있는지 확인한다. 게시 전·후 PID는 28040이다. |
| 19 | staging 후 다른 쓰기가 바뀐 파일을 덮어쓰는가 | 기존 파일의 존재·크기·해시가 manifest의 직전 값과 다르면 모든 단계가 중단한다. |
| 20 | 열린 파일을 교체해 실행 중 소비자에 불완전한 데이터를 보이는가 | 교체 대상에 열린 file handle이 하나라도 있으면 preflight가 중단한다. 이번에는 0개였다. |
| 21 | 선택 동기화가 미리보기의 기존 파일을 삭제하거나 예기치 않은 전체 정리를 하는가 | route overlay는 생성·교체만 하며 삭제를 구현하지 않고 manifest에도 `noDeletes: true`를 기록한다. |
| 22 | 이전 파일 백업이 기존 백업을 덮어쓰거나 다른 작업 backup과 섞이는가 | PR #1031 실행 전용 고유 backup root를 `mkdir`로 배타 생성한다. |
| 23 | 교체 도중 이전 파일을 먼저 잃고 복구할 수 없는가 | 기존 파일은 새 임시 파일을 교체하기 전에 backup에 복사하고 hash를 검증한다. 43개를 보존했다. |
| 24 | 동시 요청자가 부분 작성 중인 새 파일을 볼 수 있는가 | 임시 파일은 UUID 이름과 exclusive create로 만들고 완성 뒤 rename한다. 기존 mode도 유지한다. |
| 25 | 쓰기 후 대상, backup, 출처 사이의 불일치를 놓치는가 | verify 단계에서 385개 대상의 크기/hash와 43개 backup의 이전 hash를 다시 대조했다. |
| 26 | 로드맵 카드에 연결된 핵심 문서나 simulator 화면 route가 빠지는가 | 34개 필수 route에 로드맵, STATUS, C11 계획·계약, precomparison/runtime 문서와 Android/iOS 캡처를 포함했다. |
| 27 | 파일은 복사됐으나 HTTP 서비스가 다른 바이트를 내놓는가 | 지정 34 route 각각에 local/Tailnet 요청을 보내 총 68개 응답의 HTTP 200·크기·SHA-256을 manifest와 비교했다. |
| 28 | 예전 C11 카드나 잘못된 merge SHA를 새 완료 근거처럼 표시하는가 | 게시 HTML에서 PR #1030, `579435b1955e7ad157baa13d100d75f13a5dd283`, `72/213`, `C11.1/C11`을 확인한다. |
| 29 | 새 카드의 상대 경로가 Tailnet base 밖을 향하거나 없는 route를 가리키는가 | 카드의 로컬 href를 추출해 `/spinon/` base 및 staged/기존 파일 존재를 확인한다. 외부 GitHub 링크는 별도 취급한다. |
| 30 | 좁은 브라우저 확인을 전체 반응형 검증으로 과장하거나 사용자의 Agent Browser를 남기는가 | Agent Browser에서 새 카드만 1280×633 및 390×844로 확인해 카드 경계·문서 폭·표시 marker를 기록하고 세션을 닫았다. 전체 문서·모든 화면 검증은 아니라고 README에 한정했다. |

## 발견 및 한계

- 첫 `git diff --cached --check`에서 Rspress의 폭 맞춤 표 로그 끝 공백이 발견됐다. byte-exact 압축 원본과 해시를 보존하고 검토용 `.log`만 끝 공백을 정규화한 뒤 재검사했으며 통과했다.
- 이 PR 검토는 선택된 route와 지정 URL에 관한 것이며 전체 문서 링크 적합성 검사는 아니다. 전체 사이트 링크 검사는 수행하지 않았다.
- PR #1030 시뮬레이터 화면은 draw/present 보조 캡처이며, 이 검토로 PixelCopy나 Metal drawable readback을 추가 주장하지 않는다.
