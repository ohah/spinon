# C11.4 fixed-gutter Tailnet 동기화 실패 관점 검토 20개

**대상:** pass6/pass7/pass8 선택 문서 build·복사 도구·Tailnet 응답·Agent Browser 표시. 각 항목은 다른 경계나 실패 원인을 확인한다.

| # | 적대 질문 | 확인 결과 |
| ---: | --- | --- |
| 1 | PR #357의 main 병합 SHA가 아닌 stale branch를 기준으로 갱신하지 않았나? | `gh pr view 357`의 merge commit `62c86a1c056c0cd7bcdb9ba65fea35fba934d7e9`와 pass6/pass7의 source branch/HEAD를 대조했다. |
| 2 | 선택 root가 roadmap만 갱신하고 상태·계획·실행 근거를 빠뜨리지 않았나? | `STATUS.md`, C11.4 상위·집중 계획, 0059, 내부 색인, Chrome 사전 관찰, 구현 evidence를 포함한다. |
| 3 | 제한 fixture를 C11.4 전체 완료 또는 지원 CSS로 올려 쓰지 않았나? | 공개 roadmap·STATUS·두 계획에서 C11.4 체크와 완료 집계를 유지하고 percentage/calc·WPT·실기기 한계를 보존한다. |
| 4 | 문서 링크가 Tailnet base 밖으로 나가지 않나? | pass6 Agent Browser가 STATUS에서 141개 same-origin out-of-base 링크를 찾았다. pass7에서 고친 뒤 pass8 adapter가 선택 Markdown 전체를 처리했고, 최종 34개 HTML route의 결과는 모두 0개다. |
| 5 | 현재 C11.4 계획 링크가 선택 route가 아닌 잘못된 상위 `/plan`에 연결되지 않나? | selected focus plan은 `/spinon/plan/c11-4-image-ratio-fixed-gutter.html`로 치환되고 Agent Browser에서 두 참조가 같은 route로 도착했다. |
| 6 | 선택되지 않은 계획·fixture·도구 링크를 존재하지 않는 Tailnet 문서로 연결하지 않나? | 선택 subset 바깥의 repository 상대 링크는 GitHub `main`으로 보낸다. 34개 built HTML route를 검사해 Tailnet same-origin base 바깥 링크가 없음을 확인했다. |
| 7 | 문서 색인·계약·계획에서 source-relative 링크가 선택 subset 때문에 404가 되지 않나? | 선택된 상대 링크는 Tailnet route로, subset 밖 repo 링크는 GitHub `main`으로 보낸다. 최종 34개 route 모두 200이고 색인·계약·집중 계획·근거 페이지의 링크가 열린다. |
| 8 | C11.4 implementation README의 `runs/...`가 패키지 import로 오인되지 않나? | Rspress 오류에서 선행 `./` 누락을 발견했다. Markdown 이미지 경로를 명시적 상대 경로로 고친 뒤 선택 build가 성공했다. |
| 9 | Markdown 이미지와 정적 JSON·log·script가 같은 방식으로 stage되어 페이지 compiler가 보조 script를 문서로 실행하지 않나? | Markdown 이미지 파일은 import 경로와 public URL에 복사하고, 그 밖의 evidence asset은 public 아래에만 둔다. 예전 sync helper TS가 route로 컴파일되던 실패를 막았다. |
| 10 | helper 코드나 local checkout 설정이 문서 route로 노출되어 build를 깨뜨리지 않나? | Tailnet helper는 선택 page root에서 제외하고 정적 preview source와 설정을 분리했다. 두 성공 build는 고정 Rspress `2.0.22`를 사용했다. |
| 11 | 의존성 bump 또는 lockfile 변경을 문서 갱신에 끼워 넣지 않았나? | `packages/docs/package.json`, `bun.lock`의 Rspress 버전을 바꾸지 않았다. 설치된 pinned modules를 읽기 전용 symlink로 사용했다. |
| 12 | 잘못된 origin 또는 임의 사용자 경로에 선택 파일을 복사할 수 있나? | sync helper는 대상 `~/Library/Application Support/SpinonPreview`와 실행별 backup root를 고정 검증한다. 다른 경로 입력은 거부된다. |
| 13 | destination이나 backup 상위 경로의 symlink가 사용자 파일 밖으로 유도할 수 있나? | source·target·backup 경로 구성 요소와 각 destination 구성 요소의 symlink를 거부한다. selected build도 symlink를 허용하지 않는다. |
| 14 | preflight 후 source나 target이 바뀐 상태에서 오래된 판정으로 덮어쓰지 않나? | copy 단계가 source의 bytes/hash, target의 존재·bytes/hash 및 file set을 preflight와 다시 비교한다. 불일치 시 쓰기를 중단한다. |
| 15 | 정상 동기화가 기존 미리보기 파일이나 선택 범위 밖을 삭제하지 않나? | pass6·pass7의 `no_deletes`와 pass8 최종 복사의 `no_deletes`가 모두 true다. pass8에서 잘못 생성한 source-only 파일은 별도 해시·lsof 검토와 복구 backup을 거쳐 정정했으며, 서버가 열어 둔 파일은 남겼다. |
| 16 | 기존 파일 교체 전에 복구 가능한 원본 backup을 남기나? | pass6 28개·pass7 30개·pass8 최종 31개 교체 backup의 byte와 SHA를 각각 검증했다. 잘못된 pass8 source가 만든 파일 230개도 별도 recovery backup 후 정리했다. |
| 17 | 열린 파일을 강제로 닫거나 Tailnet 서버·tmux를 중지하지 않나? | pass6의 `static/js/2.1aa8a93d31.js`는 기존 서버 PID 1191이 열고 있었다. 서버를 유지하고 같은 디렉터리 temp+atomic rename을 사용했다. pass7 교체 경로에는 열린 파일이 없었다. |
| 18 | 로컬 복사 성공만으로 제공 상태를 주장하지 않았나? | pass6·pass7 각 274개와 pass8 최종 site 290개를 Tailnet URL로 요청해 HTTP 200, byte length, SHA-256을 대조했다. 각각 274/274, 274/274, 290/290 일치했다. |
| 19 | Chrome 화면·앱 Simulator 화면·실제 기기 결과를 같은 증거로 혼동하지 않나? | Agent Browser 캡처는 문서 route이며, 기존 Android AVD/iOS Simulator 앱 캡처와 분리했다. PR #357 본문 raw 이미지 4개도 실제 크기로 로드되는지 별도 확인했다. |
| 20 | Browser를 끝낸 뒤 사용자 요청과 무관한 세션까지 닫지 않나? | pass7과 pass8 모두 고유 namespace/session만 열고 닫았다. pass8 전용 daemon PID 90394와 Chrome PID 90403은 종료됐고, 사전부터 있던 daemon PID 74457은 유지됐다. |

## 발견 사항과 수정

- 선택 build 첫 시도에서 evidence README의 이미지 URL과 기존 Tailnet helper `.ts` route가 build를 막았다. 상대 이미지 경로를 고치고 Markdown 이미지 외 정적 파일을 `public/`에만 배치해 해결했다.
- pass6 Browser 확인에서 상태 대장의 141개 `../plan`, `../tests/fixtures`, `../tools` 링크가 Tailnet base 밖으로 나갔다. pass7 staging adapter가 선택 경로는 local Tailnet route로, 나머지는 GitHub `main`으로 바꾸고 실제 브라우저에서 same-origin out-of-base 링크 0개를 확인했다.
- pass8 첫 sync 명령에서 staging root를 복사 source로 잘못 지정했다. Agent Browser가 새 근거 route가 비어 있는 것을 잡았고, 기존 오류 기록을 보존했다. sanitized RSPress site output을 source로 다시 sync하고 helper에 site output 경로 검증을 추가했다.
- 잘못된 첫 복사가 추가한 source-only 231개 중 230개는 열린 handle이 없고 현재 hash가 기록과 일치함을 확인했다. 별도 recovery backup 230/230 후 제거했고, 나머지 하나는 PID 1191이 열어 둔 duplicate Simulator log라 건드리지 않았다. 정정된 pass8 site는 290/290 hash 일치, 교체 backup 31/31, 34개 HTML route 200 및 base 밖 same-origin 링크 0개다.
