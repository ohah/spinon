# PR #1022 Tailnet 로드맵 동기화 근거 검토 30개

**범위:** PR #1021 병합 후 선택한 Tailnet 미리보기 경로 동기화와 이를 기록하는 증거 자료를 검토했다. 원본 소스는 main의 004291f5033942f9037d3e8a0ee96658bbd7595d다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 보고서가 병합되지 않은 후보 HEAD를 원본으로 삼는다. | manifest와 보고서 모두 PR #1021 병합 후 main HEAD를 기록한다. 통과. |
| 2 | 복사 도중 소스 브랜치가 이동한다. | 각 단계에서 HEAD와 origin/main, manifest sourceHead 일치를 확인한다. 통과. |
| 3 | 미추적 작업 파일 때문에 사용자 변경을 덮어쓴다. | 스크립트는 추적 중인 변경이 없어야 진행하고, preview의 선택한 다섯 경로만 다룬다. 다른 미추적 파일은 대상이 아니다. 통과. |
| 4 | 동기화 범위가 README에 적힌 다섯 경로보다 넓다. | 고정 route 목록은 roadmap alias 세 개와 C11.1 Android/iOS PNG 두 개다. 통과. |
| 5 | 같은 HTML source가 아닌 서로 다른 복사본을 각 alias에 게시한다. | 세 roadmap alias 모두 main의 spec/public/roadmap.html에서 읽는다. 통과. |
| 6 | manifest에 이전 상태와 새 상태의 해시가 빠진다. | route별 기존 존재 여부, 이전 byte 수·해시와 새 해시를 manifest에 기록한다. 통과. |
| 7 | 기존 파일 교체 전에 복구 사본을 만들지 않는다. | 기존에 달라진 세 HTML을 preview 외부 백업 경로에 복사하고 이전 SHA-256을 대조한다. 통과. |
| 8 | 백업이 원래 경로 구조를 잃거나 기존 데이터를 덮어쓴다. | 백업 경로 아래 route 구조를 보존하며 COPYFILE_EXCL과 새 디렉터리 생성을 사용한다. 통과. |
| 9 | 게시 과정이 기존 preview 경로를 삭제한다. | 스크립트는 삭제를 호출하지 않고 보고서에 noDeletes를 기록한다. 통과. |
| 10 | route target이 일반 파일이 아닌 디렉터리나 특수 파일이다. | lstat으로 target을 검사하고 regular file만 허용한다. 통과. |
| 11 | route 부모가 preview 바깥 symlink를 통해 다른 파일을 가리킨다. | 각 path segment의 symlink를 거부하며 preview root의 realpath도 고정 경로와 대조한다. 통과. |
| 12 | 절대 경로, 역슬래시, 점 경로 segment로 preview 바깥을 쓴다. | route 검증이 절대 경로, 역슬래시, 빈 segment, . 및 .. segment를 거부한다. 통과. |
| 13 | main의 source 자체가 symlink이거나 regular file이 아니다. | preflight에서 source lstat 후 regular file이며 symlink가 아님을 확인한다. 통과. |
| 14 | 게시 전에 다른 프로세스가 route 파일을 열어 사용 중이다. | 변경할 기존 target마다 lsof를 확인하며 열린 handle이 없어야 진행한다. 통과. |
| 15 | preflight 뒤 target이 다른 내용으로 바뀌었는데 덮어쓴다. | 복사 전과 각 rename 직전에 존재 상태와 이전 해시를 다시 비교한다. 통과. |
| 16 | preflight 뒤 서버 프로세스가 바뀌거나 재시작된다. | copy 단계가 preflight의 listener PID와 현재 PID가 같은지 검사한다. 통과. |
| 17 | 임의의 로컬 서버나 Tailscale이 꺼진 상태에서 성공 보고서를 만든다. | Tailscale BackendState Running, 단일 8768 listener, tailnet DNS, server command를 확인한다. 통과. |
| 18 | 게시 파일에 부분 쓰기가 보인다. | 같은 디렉터리의 유일한 temp 파일에 전체 기록·fsync 후 rename하고 결과 해시를 확인한다. 통과. |
| 19 | 오류 뒤 temp file이 남는다. | rename 묶음의 finally 절에서 남은 temp 파일을 제거한다. 통과. |
| 20 | 백업과 게시 파일이 다른 source byte를 사용한다. | preflight manifest 해시와 copy 단계의 source 해시를 대조하고 최종 target 해시를 확인한다. 통과. |
| 21 | 세 alias 중 하나가 빠지거나 두 화면 자료가 뒤바뀐다. | manifest는 HTML 세 route와 Android 및 iOS PNG route 각각을 개별 기록한다. 통과. |
| 22 | 보고서의 10회 요청 수가 실제 검사 수와 다르다. | 다섯 route에 localhost와 Tailscale 요청을 각각 실행해 총 열 응답을 기록했다. 통과. |
| 23 | HTTP 성공만 확인하고 stale cache 본문을 승인한다. | 요청은 no-cache 헤더와 source hash query를 사용하고 응답 본문 SHA-256을 검사한다. 통과. |
| 24 | 200 응답이어도 응답이 잘렸거나 다른 파일이다. | 각 route별 응답 상태, byte 수, SHA-256을 source와 모두 대조한다. 통과. |
| 25 | Tailnet 본문에 최신 percentage 카드가 없다. | 별도 카드 확인에서 PR #1019 문구와 공식 72/213을 확인한다. 통과. |
| 26 | Android 증거를 iOS 캡처로 표시하거나 해상도가 맞지 않는다. | Agent Browser가 Android PNG를 1080×2400으로 로드했음을 기록한다. 통과. |
| 27 | iOS 증거를 Android 캡처로 표시하거나 해상도가 맞지 않는다. | Agent Browser가 iOS PNG를 1206×2622로 로드했음을 기록한다. 통과. |
| 28 | 데스크톱 카드가 실제 Tailnet 페이지가 아닌 로컬 캡처다. | 사용자가 지정한 Tailnet URL에서 페이지를 열어 1280×800으로 캡처했다. 통과. |
| 29 | 모바일에서 카드가 viewport 밖으로 밀리거나 수평 overflow가 생긴다. | Tailnet 페이지를 390×844에서 확인했고 documentElement.scrollWidth 390, scrollX 0을 기록했다. 통과. |
| 30 | 좁은 sync 결과를 사이트 전체 또는 C11 완료로 과장하거나 Agent Browser를 남긴다. | 다섯 route만 검증했고 전체 build·전체 route·C11 완료 주장은 하지 않았다. 확인 후 Agent Browser session inactive 및 agent-browser PID 부재를 기록했다. 통과. |

## 판정

PR #1022는 선택한 Tailnet preview 경로의 재현 가능한 동기화 근거만 추가한다. 공식 상태 대장은 변경하지 않았고 C11은 미완료이며 완료 수는 72/213이다.
