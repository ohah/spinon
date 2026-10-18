# C04.12 Tailnet 로드맵 sync PR · diff 실패 관점 20개

이번 PR의 Tailnet 동기화 manifest·백업 확인·HTTP 검증·Agent Browser 근거를 서로 다른 20개 실패 관점으로 검토했다.

| # | 검토 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | sync가 #321/#322와 다른 revision을 기준으로 했을 수 있다. | manifest가 source PR #321/#322와 main commit `887f1660`을 고정한다. |
| 2 | Tailnet의 raw roadmap와 repo 원본이 다를 수 있다. | `/roadmap.html`·`/spec/public/roadmap.html` 응답 해시를 최신 원본과 맞췄다. |
| 3 | `/plan/...html` 링크가 소스 Markdown만 복사되어 404일 수 있다. | 렌더된 계획 HTML과 raw Markdown을 함께 제공하고 두 응답을 검사했다. |
| 4 | 새 근거 README route가 생성되지 않을 수 있다. | 근거 README HTML과 세 review HTML route가 모두 payload에 포함됐다. |
| 5 | roadmap light/dark 이미지는 링크만 있고 바이너리가 없을 수 있다. | 두 Chrome PNG와 browser screenshot이 manifest에 포함됐다. |
| 6 | 상태 대장이 PR #321 plan-only 기록보다 오래된 내용일 수 있다. | `STATUS.md`, `spec/STATUS.md`, `STATUS.html`을 최신 main에서 갱신했다. |
| 7 | Tailnet의 두 STATUS raw 경로가 서로 다른 버전을 제공할 수 있다. | 두 파일의 최종 SHA-256이 동일하다. |
| 8 | 로드맵 갱신이 공식 완료 수를 잘못 바꿀 수 있다. | main의 상태 대장을 그대로 복사했고 완료 수와 C04.12/C15/C16/C19 상태를 바꾸지 않았다. |
| 9 | overwrite 중 원본을 잃을 수 있다. | 변경된 다섯 대상 각각의 이전 파일을 별도 backup 경로에 남겼다. |
| 10 | backup이 불완전하거나 다른 파일일 수 있다. | backup 크기와 SHA-256을 이전 값과 대조해 5/5 통과했다. |
| 11 | 사전 조사 뒤 대상 파일이 바뀌었을 수 있다. | 적용 스크립트가 preflight의 이전 해시·크기와 다시 대조한 뒤 복사했다. |
| 12 | 중간 복사로 일부 파일만 잘릴 수 있다. | 같은 디렉터리 임시 파일을 만든 뒤 원자 교체하고 결과 해시를 확인했다. |
| 13 | 사용하지 않는 파일을 지우거나 기존 preview tree를 초기화할 수 있다. | 복사 전용 manifest이며 삭제 목록은 비어 있고 preview 전체를 정리하지 않았다. |
| 14 | 64개만 집계하고 누락 경로를 숨길 수 있다. | 생성·교체·불변 경로 수가 `39+5+20=64`로 합산되고 응답별 결과가 저장됐다. |
| 15 | loopback 서버에서만 되고 Tailnet HTTPS는 실패할 수 있다. | 동일 64개 경로를 로컬과 HTTPS 양쪽에서 각각 요청해 64/64 exact hash를 확인했다. |
| 16 | server restart나 다른 listener로 우회했을 수 있다. | 동기화 전후 PID `1191`, `127.0.0.1:8768` listener가 유지됐다. |
| 17 | 새 로드맵 항목이 화면에 없는데 HTTP만 성공할 수 있다. | Tailnet Agent Browser에서 문단을 찾아 스크롤한 화면과 문구를 보존했다. |
| 18 | 계획 route가 raw HTML이거나 잘못된 페이지일 수 있다. | Agent Browser 제목/heading이 C04.12 기본 root HTML 문서 렌더링 계획으로 표시됐다. |
| 19 | 캡처가 다른 origin 또는 로컬 파일을 보여줄 수 있다. | browser verification이 실제 `macstudio.tailed42f2.ts.net/spinon/` URL을 기록한다. |
| 20 | 검증 뒤 Agent Browser 프로세스가 남아 있을 수 있다. | close 성공, `browserLaunched=false`, 후속 PID 조회에서 프로세스 부재를 확인했다. |

이 검토는 개발 Tailnet 사본과 문서 route만 다룬다. 앱 구현, Simulator 동작, GitHub Pages 배포 또는 전체 문서 링크 검증을 주장하지 않는다.
