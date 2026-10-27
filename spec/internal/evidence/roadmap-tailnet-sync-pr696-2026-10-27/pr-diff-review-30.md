# Tailnet roadmap 단일 경로 동기화 문서 PR diff 적대 검토 30개

검토 대상은 PR #696 병합 뒤 Tailnet preview의 roadmap 한 파일을 바꾼 실행 근거, C11.4 계획의 상태 문구, README와 Agent Browser 화면이다. 동기화 전후의 출처·대상 hash, preview 서버 상태, 경로 범위, 공식 완료 집계, 사후검증의 일시 연결 타이밍을 서로 다른 실패 가설로 대조했다.

| # | 실패 가설 | 근거·판정 |
|---:|---|---|
| 1 | 출처 HEAD가 실제 병합된 `main`과 다르다. | `preflight.json`과 `sync-result.json`은 `d019db91d9f592af99d9cfed22bcd1b1600f1445`를 기록한다. 동기화 스크립트가 `origin/main`과 정확히 대조한다. 통과. |
| 2 | 로컬 roadmap 파일이 그 HEAD의 tracked blob과 다르다. | 스크립트는 로컬 바이트를 `git show origin/main:spec/public/roadmap.html` 출력과 같다고 검사한다. 통과. |
| 3 | 신규 source의 크기나 SHA가 기록과 다르다. | source와 이후 local/Tailnet 응답은 모두 347,943 bytes, SHA-256 `7f8091cb06785e7c93c4c86fe17c2156a904251c7e01b24e8fe9599dc654ce64`다. 통과. |
| 4 | 교체 전 Tailnet 대상이 예상과 다른 파일이었다. | preflight에서 regular file과 기존 346,098 bytes/hash `aa46929adb6856c3a45ed7aa04f493563917fbcf25d05f39f6ee9f9ecd1c8cd8`를 확인했다. 통과. |
| 5 | 같은 이름의 task backup이 이미 있어 이전 자료를 덮어쓴다. | task backup 디렉터리가 없음을 확인한 뒤 고유 `pass01` 경로를 만들었다. 통과. |
| 6 | 백업이 원본과 바이트 단위로 다르다. | 백업 생성 직후 byte equality와 hash를 검사했다. preflight에는 교체 전 원본 크기/hash, 결과 JSON에는 생성한 백업의 크기/hash를 남겼다. 통과. |
| 7 | 대상이 symlink이거나 regular file이 아니어서 예상 밖 경로를 바꾼다. | 스크립트가 `lstat`, regular-file 검사와 symlink 거부를 교체 전에 적용한다. 통과. |
| 8 | 다른 프로세스가 roadmap 파일을 읽거나 열어 교체를 방해한다. | preflight `lsof`에는 대상 파일 open handle이 없었다. 통과. |
| 9 | 8768 포트에 둘 이상의 preview listener가 있어 엉뚱한 서버를 검사한다. | 유일 listener PID를 요구하고 작업 전·후 같은 PID `28040`을 확인한다. 통과. |
| 10 | 해당 PID가 알려진 Tailnet preview server가 아니다. | `ps` 명령행은 `tailnet-preview-server.mjs`를 가리킨다. 통과. |
| 11 | 동기화 시점에 Tailscale backend가 내려가 있었다. | preflight와 후속 확인에서 `BackendState=Running`이었다. 통과. |
| 12 | 사전검사 중 preview server에 요청 연결이 남아 동시 요청 위험이 있다. | preflight `lsof`는 listener 하나만 보여 주었다. 통과. |
| 13 | 기존 로컬 route가 예상한 이전 roadmap을 제공하지 않는다. | 동기화 전 local HTTP 200 body는 이전 대상 hash와 같았다. 통과. |
| 14 | 기존 Tailnet route가 오래되었거나 local과 다르다. | 동기화 전 Tailnet HTTP 200 body도 이전 대상과 같은 hash였다. 통과. |
| 15 | 작업이 계획된 roadmap 외 다른 preview 경로까지 교체한다. | 스크립트는 `roadmap.html` 대상 하나만 두고 결과도 `/spinon/roadmap.html` 한 경로만 기록한다. 통과. |
| 16 | 새 파일을 먼저 설치해 백업 없이 이전 roadmap을 잃는다. | 백업 파일을 fsync하고 바이트/hash를 재확인한 다음 target을 교체한다. 통과. |
| 17 | 새 바이트가 부분 기록되어 요청자가 잘린 파일을 읽는다. | 같은 디렉터리에 fsync한 임시 파일을 준비한 뒤 `os.replace`와 directory fsync를 사용한다. 통과. |
| 18 | 쓰기 실패 뒤 임시 파일을 남겨 preview 디렉터리를 오염시킨다. | `finally` 절이 남은 임시 파일을 제거한다. 통과. |
| 19 | 교체 후 local route가 HTTP 오류를 반환한다. | 후속 no-cache 요청은 HTTP 200이었다. 통과. |
| 20 | Tailnet route가 이전 stale HTML을 계속 제공한다. | 후속 Tailnet HTTP 200 body가 새 source의 SHA와 byte 수 모두 일치한다. 통과. |
| 21 | route 응답의 크기만 같고 내용은 다르다. | local과 Tailnet 각 body를 source 바이트와 직접 비교했고 SHA-256도 같았다. 통과. |
| 22 | 성공 검증을 위해 preview server를 재시작하거나 PID가 바뀐다. | 작업 전후 listener PID가 `28040`으로 같고, 재시작하지 않았다. 통과. |
| 23 | 스크립트가 사후 `lsof` 지연을 실제 route/content 실패로 잘못 설명하거나, 뒤늦은 수정본을 실제 실행한 것처럼 기록한다. | 첫 실행은 route hash 검사 후 3초 listener-only poll에서 열린 loopback 연결 때문에 종료됐다. README와 JSON에 그 사실과 별도 후속 코드 수정(`Connection: close`, 15초 대기)을 구분해 적었고, 수정본을 live target에 재실행하지 않았다. 통과. |
| 24 | 초기 연결이 남은 관찰만으로 최종 상태가 조용하다고 과장한다. | `Connection: close` 후속 요청 뒤 `lsof`에는 listener만 남았음을 별도로 확인했다. 통과. |
| 25 | roadmap 화면이 공식 완료 수를 잘못 올린다. | Tailnet DOM 확인과 PR 항목은 공식 `72/213` 유지라고 보이며 source 본문도 그 수를 명시한다. 통과. |
| 26 | C11.4 runtime slice를 전체 항목 완료로 읽게 한다. | 화면에 `C11.4는 미완료`가 있고 공식 checkbox/STATUS 파일은 이 PR에서 수정하지 않는다. 통과. |
| 27 | roadmap의 PR #695 이력에 merge SHA/시각이 잘못 표시된다. | 브라우저 DOM과 화면은 PR #695, merge SHA `d6dd9fd15d2897804142e6bb119149a633952a9b`를 렌더링한다. 통과. |
| 28 | 캡처가 Tailnet이 아닌 다른 환경이나 이전 페이지에서 왔다. | Agent Browser URL은 사용자 제공 Tailnet host의 `/spinon/roadmap.html`이며 DOM title과 현재 PR #695 이력이 확인됐다. 통과. |
| 29 | 화면 파일이 누락·빈 파일이거나 해상도 정보를 과장한다. | 두 PNG가 각각 1200×829로 식별되고 README에서 대응 화면에 연결된다. 통과. |
| 30 | 검증 범위가 full-site 배포·다른 경로·CI 성공까지 확대된다. | README는 단일 route sync로 한정하고 full docs build/Pages deploy를 하지 않았다고 기록한다. 이 문서 PR은 CI 성공을 주장하지 않으며 checks는 실제 GitHub 결과와 분리해 보고한다. 통과. |

**판정:** 30개의 서로 다른 실패 가설을 동기화 코드, source/backup hash, local/Tailnet 응답, Agent Browser 화면, 계획 diff에 대조했다. 초기 `lsof` 타이밍 실패를 숨기지 않고 후속 route와 socket 관찰을 분리해 남겼다. 공식 C11.4 상태와 `72/213`에는 변경이 없다.
