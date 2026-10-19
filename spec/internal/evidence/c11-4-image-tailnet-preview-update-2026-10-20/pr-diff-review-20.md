# C11.4 Tailnet preview 갱신 PR diff 검토 20개

**검토 범위:** C11.4 roadmap·STATUS·계획 링크 변경, 선택 문서 staging/link 변환, RSPress sync 설정과 도구, pass6–pass9 기록, Tailnet 제공물, Agent Browser 캡처. 각 관점은 독립 실패 경로를 확인한다.

| # | 적대 질문 | 확인 결과 |
| ---: | --- | --- |
| 1 | PR #357 merge 이전의 stale branch를 근거로 현재 상태를 갱신하지 않았나? | 작업 HEAD와 `origin/main`은 `62c86a1c056c0cd7bcdb9ba65fea35fba934d7e9`이며 PR #357의 rebase merge SHA와 일치한다. |
| 2 | 좁은 fixed-gutter 구현을 C11.4 전체 완료로 올려 쓰지 않았나? | 집중 계획·상태 대장·공개 roadmap 모두 C11.4 체크와 완료 수를 그대로 두고 percentage/calc, WPT, text/image 조합, 전체 Grid 범위를 남겼다. |
| 3 | 플랫폼 상태나 출시 상태를 문서 미리보기 결과로 오해하게 하지 않나? | 공식 상태 대장의 미출시·현재 제품 지원 완료 없음·내부 숫자 버전 `0.1.0`을 유지했다. |
| 4 | PR #357이 실제 main에 병합됐다는 표현에 근거가 있나? | GitHub PR #357 merge 상태와 main commit `62c86a1c...`를 확인하고 계획·상태·구현 근거 문서에 연결했다. |
| 5 | 로드맵의 PR #357 항목에서 고정 track과 gutter를 혼동하지 않나? | 공개 roadmap 설명은 `3px + 3px + 4px → 10px` clamp와 `track 7px / gutter 3px` trace를 구분한다. |
| 6 | 계획·근거 문서의 변경이 기존 Chrome·Simulator 수치를 과장하지 않나? | Android는 API 37 AVD, iOS는 iPhone 17 Pro/iOS 26.2 Simulator로 표기하고, iOS surface 표본이 compositor 전 Metal drawable임을 유지했다. |
| 7 | 이전 계획의 “PR 생성·merge 대기” 문구가 남아 있지 않나? | 집중 계획과 구현 README를 PR #357 merge 상태로 갱신했고 C11.4 미완료 문구는 보존했다. |
| 8 | relative screenshot URL이 repository root 기준 import로 오인돼 RSPress build를 깨뜨리지 않나? | 구현 README 이미지 URL에 `./`을 명시했다. 고정 RSPress 2.0.22 선택 build가 pass9에서 성공했다. |
| 9 | 로그·JSON·helper가 Markdown 문서로 해석돼 build route를 깨뜨리지 않나? | evidence asset은 `public/`으로 복사하고 Markdown 이미지만 문서 import와 public URL 양쪽에 배치한다. 선택 build 결과 34개 HTML route를 확인했다. |
| 10 | source root와 sanitized output을 바꿔 넣어 raw Markdown을 공개 preview에 복사할 수 있나? | pass8 첫 오지정이 Browser에서 드러나자 sync helper가 정확한 `build/spinon/c11-4-image-tailnet-site-${pass}`만 허용하도록 가드를 추가했다. pass9도 이 경로에서 생성했다. |
| 11 | relative link 재작성기가 selected subset의 링크를 잘못된 `/spinon/` 경로로 바꾸지 않나? | 선택 source와 destination mapping으로 local HTML route를 만들고, subset 밖 repository 문서는 GitHub `main`으로 보낸다. pass9에서 HTML route 34개를 모두 검사했다. |
| 12 | scheme·절대·hash 링크나 query/fragment suffix를 훼손하지 않나? | URL scheme, leading slash, fragment-only 링크는 원문을 유지하고 relative path suffix를 분리해 route rewrite 뒤 보존한다. Browser에서 전체 선택 HTML의 out-of-base same-origin 링크는 0개다. |
| 13 | Tailnet base가 `/spinon/`인 환경에서 root-relative link가 호스트 루트로 빠지지 않나? | 34개 HTML route 모두 HTTP 200이고 각 렌더 결과에서 same-origin out-of-base link가 없다. |
| 14 | 기존 preview 파일이 사전 비교 뒤 바뀌거나 symlink로 우회될 수 있나? | sync helper는 source·target·backup 구성 요소의 symlink를 거부하고 preflight 후 bytes, SHA, file set, 기존 대상 식별을 다시 비교한다. |
| 15 | 복사 중 열려 있는 파일을 강제로 닫거나 부분 파일을 제공하지 않나? | 교체는 같은 디렉터리 temporary file 후 atomic rename을 사용한다. pass9 교체 33개 중 열린 파일은 없었고 서버와 tmux는 유지했다. |
| 16 | 기존 파일 backup을 확인하지 않고 교체하지 않나? | pass9 교체 파일 33개 모두 backup bytes/SHA를 대조했고 33/33 검증했다. |
| 17 | 복사 결과만 확인하고 실제 Tailnet 응답을 추정하지 않나? | pass9 selected file 304개를 Tailnet URL로 요청해 HTTP 200·byte count·SHA-256이 304/304 일치했다. |
| 18 | 잘못된 첫 pass8 복사가 남긴 파일을 무기록·무복구로 정리하지 않았나? | 231개 후보를 original hash와 대조하고 열린 handle을 확인했다. 닫힌 230개를 별도 backup 230/230 후 제거했고 PID 1191이 열어 둔 duplicate Simulator log는 남겼다. 첫 실행 기록도 보존했다. |
| 19 | Agent Browser 문서 화면을 Android/iOS 앱 실행 근거로 잘못 표시하지 않나? | Tailnet 화면 5개와 앱 Simulator 화면을 분리했다. 기존 Android AVD 2장과 iOS Simulator 2장 모두 실제 크기로 로드됐으며 치수는 각각 1080×2400, 1206×2622다. |
| 20 | Browser를 닫으면서 사용자 기존 daemon을 종료하지 않나? | 전용 pass9 daemon PID 5376과 Chrome PID 5377만 닫았고, 사전부터 있던 Agent Browser PID 74457은 종료 후에도 남았다. |

## 발견 및 조치

- pass6에서 선택 build가 이미지 상대 경로와 helper route 때문에 실패해 근거를 남기고 source staging을 고쳤다.
- pass6 Browser가 STATUS 상대 링크 141개의 base 이탈을 찾았다. pass7·pass8에서 전체 선택 문서 링크 변환으로 넓히고 pass9에서 34개 HTML route 모두 200 및 same-origin base 이탈 0개를 확인했다.
- pass8 최초 복사에는 staging source 경로를 잘못 넣었다. Browser에서 새 근거 페이지가 비어 있음을 확인한 직후 sanitized RSPress output으로 다시 복사했다. 잘못 추가한 source-only 파일은 hash·lsof 검사와 복구 backup을 거쳐 정리했으며 열린 로그 하나는 서버를 건드리지 않고 유지한다.
- GitHub Pages 전체 배포, 전체 문서 site build, 앱 실행, WPT 또는 실기기 검증은 이 문서 PR에서 수행하지 않았다.
