# Tailnet 동기화 증거 PR 변경분 검토 30관점

검토 대상은 main 병합 이후 생성한 선택 source manifest·RSPress build·publication·backup·HTTPS hash 확인과 화면 증거다. 이 검토는 2026-11-11 현재 변경분을 대상으로 하며, 제품 코드 변경이나 전체 문서 링크 검사를 뜻하지 않는다.

| # | 검토 관점 | 확인 |
|---:|---|---|
| 1 | merge source ref가 `origin/main`을 가리키는가 | pass02 manifest에 PR #1339 merge SHA 기록 |
| 2 | 선택 ref와 materialized checkout tree가 같은가 | 두 tree가 `2491db9590f922e936b867b6c3a6d19b31513fcd` |
| 3 | tracked checkout diff를 거부하는가 | 선택 스크립트가 `git diff --quiet`로 중단 조건 확인 |
| 4 | 일반 파일마다 committed blob과 checkout bytes를 비교하는가 | SHA-256 일치 assertion |
| 5 | symlink 입력을 거부하는가 | source path component와 tracked evidence 파일을 `lstat` 검사 |
| 6 | evidence 폴더를 로컬 전체가 아니라 ref의 tracked tree에서 열거하는가 | `git ls-tree` 결과만 staging |
| 7 | markdown image 경로가 repo 바깥으로 벗어나는가 | normalized path prefix assertion |
| 8 | main에 없는 참조 asset을 실수로 publish하는가 | committed-file lookup이 없는 asset을 제외 |
| 9 | 선택 root route가 absolute path나 traversal을 허용하는가 | route validator가 `/`, `..`, 역슬래시 거부 |
| 10 | 기존 build pass를 덮어쓰는가 | root 및 manifest 존재 시 실패 |
| 11 | selected-root 중복 route가 다른 bytes를 숨기는가 | 기존 route와 SHA-256 일치 assertion |
| 12 | generated index가 기존 preview 시작 페이지를 교체하는가 | publication에서 `index.html` 제외 |
| 13 | generated 404가 기존 preview 오류 페이지를 교체하는가 | publication에서 `404.html` 제외 |
| 14 | publication 파일 bytes와 manifest가 다를 수 있는가 | route별 size 및 SHA 검사 후 작성 |
| 15 | roadmap의 #1338 merge SHA와 진행 카드를 확인하는가 | stage preflight assertion |
| 16 | 관련 #1334 계획 이력 검사가 사라졌는가 | 기존 merge SHA와 3개 링크 확인 유지 |
| 17 | 공식 완료 수를 바꾸거나 잘못 읽는가 | roadmap·STATUS 모두 `78/214` assertion |
| 18 | C11 checkbox가 완료로 승격되는가 | C11과 C11.4 unchecked assertion |
| 19 | 내부 계약이 잘못된 구현 merge를 가리키는가 | 계약 0059의 #1338 및 SHA 확인 |
| 20 | publication route 수가 manifest와 달라지는가 | stage 단계에서 route set과 manifest 일치 |
| 21 | preview 외부 경로를 수정하는가 | 고정 `SpinonPreview` root 외 경로 없음 |
| 22 | server listener가 다른 서비스인데도 계속하는가 | Tailscale 상태, PID command, loopback port 검사 |
| 23 | 교체 target이 열린 동안 덮어쓰는가 | preflight와 copy 직전 `lsof` 검사, 0 handle 확인 |
| 24 | 기존 교체 bytes를 백업 없이 덮어쓰는가 | 고유 backup root에 기존 SHA 검증 후 보관 |
| 25 | 이전 backup을 조용히 덮어쓰는가 | backup root 존재 시 거부 |
| 26 | preflight 후 target이 변경되어도 복사하는가 | target 존재·SHA 재검증 |
| 27 | publication source가 preflight 후 바뀌어도 복사하는가 | source bytes SHA 재검증 |
| 28 | 개별 파일 쓰기가 부분 상태로 노출되는가 | 같은 디렉터리 임시 파일 후 atomic rename |
| 29 | 새 preview가 publication과 일치하는가 | local 275/275 route size·SHA 확인 |
| 30 | 공개 HTTPS route가 stale/404 bytes를 제공하는가 | HTTPS 275/275 HTTP 200·size·SHA 일치 |

별도 사용자 상태 보존: server는 재시작하지 않았고, publication 대상 밖의 파일은 삭제하지 않았다. 교체 41개는 이전 SHA로 backup 검증했다. Agent Browser 화면 근거는 [README](./README.md)와 해시가 고정된 desktop/mobile 캡처에 기록했다.
