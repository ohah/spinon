# Tailnet PR #954 동기화 변경 실패 관점 검토 30개

검토 대상은 선택 문서 root 구성, Rspress 설정, publication staging, 안전 동기화 스크립트, 검증 기록이다. 검토는 정적 Tailnet 미리보기 갱신 경계에 한정한다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 동기화 대상이 GitHub Pages 또는 외부 공개 배포로 바뀌는가 | 대상은 사용자가 지정한 Tailnet preview root 하나다. PASS |
| 2 | preview server를 불필요하게 재시작하는가 | listener PID 28040을 확인했고 재시작하지 않았다. PASS |
| 3 | 기존 preview 경로를 전체 삭제하거나 초기화하는가 | publication은 이전 route를 보존하며 sync script에 delete 동작이 없다. PASS |
| 4 | 같은 이름의 기존 증거 파일을 덮어쓰는가 | 산출물 경로는 새 `pr954` 증거 폴더와 build root를 사용한다. PASS |
| 5 | 이전 preview 파일이 백업 없이 교체되는가 | preflight에서 변경 파일 19개를 식별하고 교체 전에 각각 백업했다. PASS |
| 6 | backup 폴더의 기존 자료가 덮어써지는가 | pass01 backup 경로가 비어 있음을 확인하고 배타적으로 생성했다. PASS |
| 7 | 백업 내용이 바이트 단위로 보존되지 않는가 | 19/19 backup SHA-256이 교체 전 대상과 일치한다. PASS |
| 8 | preview 경로 상위가 심볼릭 링크로 탈출하는가 | root와 대상 경로의 symlink를 거부하고 실제 preview root를 확인한다. PASS |
| 9 | publication route에 `..`, 절대경로 또는 역슬래시가 들어가는가 | 경로 조립 전 각 route를 검사한다. PASS |
| 10 | 이전 publication이 검증 없이 기준으로 사용되는가 | 이전 179개 route 수·크기·SHA-256을 manifest와 모두 대조했다. PASS |
| 11 | 이번 source HEAD가 GitHub main과 다른가 | 선택 root와 staging 모두 `origin/main` tree 일치를 요구한다. PASS |
| 12 | 선택 근거에 미추적 simulator 복사본이 섞이는가 | 관련 두 evidence 디렉터리는 `git ls-files` 결과만 복사했다. PASS |
| 13 | 작업 범위와 무관한 Rust/V8 빌드 산출물을 문서 사이트에 복사하는가 | 선택 root는 roadmap, 관련 계획·계약·근거, fixture로 제한했다. PASS |
| 14 | Rspress 버전이 프로젝트 고정 버전과 달라지는가 | 저장소 설치본 Rspress 2.0.22를 사용했다. PASS |
| 15 | 문서 빌드가 Tailnet 이외의 공개 경로를 배포하는가 | Rspress는 로컬 선택 build만 만들고 Pages workflow는 실행하지 않았다. PASS |
| 16 | 전체 문서 링크 검사를 통과했다고 과장하는가 | dead-link 전체 검사를 실행하지 않았다고 기록했다. PASS |
| 17 | 공개 roadmap에서 병합 PR #952의 식별이 누락되거나 잘못되는가 | 카드에서 PR 번호와 merge SHA `be62a4dd…`를 확인했다. PASS |
| 18 | stale `plan-only` 문구가 남는가 | Agent Browser가 현재 카드를 읽었고 `plan-only`가 없음을 확인했다. PASS |
| 19 | C11.4나 C11을 부분 slice만으로 완료 처리하는가 | 카드와 공식 상태가 미완료, `72/213`으로 유지된다. PASS |
| 20 | Android/iOS Simulator 결과를 실기기 결과로 표현하는가 | 공개 카드가 AVD와 iOS Simulator 범위로 표기하고 실기기 미검증을 남긴다. PASS |
| 21 | simulator 화면을 glyph paint 증거로 오해하게 하는가 | 카드가 glyph pixels를 검증하지 않았다고 명시한다. PASS |
| 22 | PR #952 카드의 계획·내부 계약·실행 근거 링크가 빠지는가 | 9개 내부 링크를 검사해 모두 HTTP 200을 받았다. PASS |
| 23 | 링크된 원본 PNG/JPG가 route에서 빠지는가 | Android와 iOS 대표 PNG route가 선택 publication에 있고 HTTP 200이다. PASS |
| 24 | 변경된 `STATUS.html` 또는 내부 계약 페이지가 이전 HTML로 남는가 | 최신 Rspress 생성 페이지를 이전 publication 위에 적용했고 해시를 대조했다. PASS |
| 25 | Rspress generated route가 manifest 밖으로 빠지는가 | staging 후 파일 수와 각 route/hash를 manifest에 고정했다. PASS |
| 26 | local preview가 다른 서버 프로세스나 포트를 보고 있는가 | Tailscale 상태, PID, 명령행, `127.0.0.1:8768` listener를 확인했다. PASS |
| 27 | 변경 파일을 열고 있는 프로세스를 무시하고 원자 교체하는가 | preflight에서 열린 대상 handle 0개를 확인했다. PASS |
| 28 | 복사 중 source/target이 바뀐 것을 놓치는가 | copy 직전에 preflight hash와 존재 여부를 재검사하고 임시파일 rename을 쓴다. PASS |
| 29 | Tailnet이 오래된 CDN/browser 내용만 보여 주는가 | 브라우저 카드 내용과 모든 선택 HTTPS 응답의 SHA-256을 확인했다. PASS |
| 30 | Agent Browser 세션이 끝난 뒤 프로세스가 남는가 | 세션을 닫고 agent-browser 및 해당 Chrome 프로세스가 없음을 확인했다. PASS |

남은 범위: 전체 저장소 dead-link 검사는 하지 않았다. 이 검토는 문서 동기화 안전성과 PR #952 카드의 게시 상태만 다루며 CSS 동작이나 C11 완료를 추가 판정하지 않는다.
