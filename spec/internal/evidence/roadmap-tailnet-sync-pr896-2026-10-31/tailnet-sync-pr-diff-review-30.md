# Tailnet PR #896 source publication sync · 변경 실패 관점 30개

이 검토는 PR #895/#896이 main에 병합된 뒤 Tailnet selected publication을 갱신한 경계만 확인한다. CSS 구현 적합성, 전체 문서 링크 검사 또는 C11 완료를 대신하지 않는다.

| # | 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | publication이 PR #896 병합 전 source를 쓰는가 | **PASS** — source HEAD는 merge 뒤 `2e8ab8cf5211635920430d69ff4ce34dd09f5ca4`다. |
| 2 | selected root tree hash가 다른 revision인가 | **PASS** — manifest는 `08c94be8181a778a8d4c041462d566bbb10e0747`이다. |
| 3 | untracked 작업 산출물이 게시되는가 | **PASS** — root 구성은 source HEAD의 `git ls-tree`와 `git show`만 사용한다. |
| 4 | dirty tracked file을 main source로 섞는가 | **PASS** — source preparation이 HEAD/tree와 tracked diff를 일치시켰다. |
| 5 | symlink 또는 특수 파일이 root에 들어가는가 | **PASS** — tracked mode는 regular blob만 허용한다. |
| 6 | 절대경로·`..` route가 생기는가 | **PASS** — staging은 각 route segment를 검사한다. |
| 7 | raw roadmap route가 병합 source와 다르게 게시되는가 | **PASS** — `roadmap.html` bytes는 선택한 raw `spec/public/roadmap.html`과 동일하다. |
| 8 | 최신 roadmap entry가 PR #893을 가리키지 않는가 | **PASS** — PR 번호와 Rebase merge 문구를 publication marker로 검사했다. |
| 9 | merge SHA가 틀리는가 | **PASS** — `ae292fff135a925ef30a3e2437082e96c2386e9c`가 렌더된 entry에 있다. |
| 10 | 140/140 값이 플랫폼 하나에만 적용되는가 | **PASS** — 문구는 Android와 iOS 각각 140/140을 표시한다. |
| 11 | 시뮬레이터 결과가 실기기 검증으로 오해되는가 | **PASS** — Android AVD와 iOS Simulator로 명시한다. |
| 12 | 빈 check rollup이 CI 통과로 표시되는가 | **PASS** — roadmap 문구는 CI 통과로 세지 않는다고 명시한다. |
| 13 | 제한 fixture 결과를 CSS 전체 지원으로 표현하는가 | **PASS** — public CSS, authored CSS admission과 전체 WPT는 미완료로 남아 있다. |
| 14 | C11 공식 완료 수가 바뀌는가 | **PASS** — source와 rendered page 모두 `72/213`을 유지한다. |
| 15 | C11/C11.4가 완료 상태로 표시되는가 | **PASS** — 최신 entry는 두 항목의 미완료를 명시한다. |
| 16 | PR #893 Android 이미지 route가 publication에서 빠지는가 | **PASS** — required route 및 HTTPS hash 결과에 포함됐다. |
| 17 | PR #893 iOS 이미지 route가 publication에서 빠지는가 | **PASS** — required route 및 HTTPS hash 결과에 포함됐다. |
| 18 | RSPress가 README 이미지 URL을 package specifier로 처리하는가 | **PASS** — PR #896의 `./` 수정 뒤 build가 성공했고 PNG asset을 output에 포함했다. |
| 19 | 원래 RSPress image-resolution 실패를 감추는가 | **PASS** — 수정 전 실패 log를 별도 보존했다. |
| 20 | 잘못된 required-route guard를 성공으로 세는가 | **PASS** — pass01은 기존 산출물로 보존하고 올바른 route name의 pass02 검증으로 완결했다. |
| 21 | source selection이 문서 전체 link validation을 수행했다고 과장하는가 | **PASS** — README와 manifest에서 전체 링크 검사가 아님을 밝힌다. |
| 22 | required C11 routes가 누락되는가 | **PASS** — plan, 0059 contract, STATUS, 실행 README·reviews 및 두 PNG를 요구하고 확인했다. |
| 23 | 중복 publication route에서 서로 다른 bytes가 덮이는가 | **PASS** — duplicate route는 SHA-256이 다르면 staging을 중지한다. |
| 24 | preflight 없이 preview target을 덮는가 | **PASS** — copy가 먼저 저장된 source/target hash 사전검사를 요구한다. |
| 25 | 기존 target bytes를 backup 없이 교체하는가 | **PASS** — replace 67개를 rename 전에 각 byte/hash로 백업했다. |
| 26 | 기존 preview route가 의도치 않게 삭제되는가 | **PASS** — sync 결과는 `no_deletes: true`이고 삭제 작업은 없다. |
| 27 | 다른 서버나 public listener를 수정하는가 | **PASS** — PID command와 단일 `127.0.0.1:8768` listener를 검사했다. |
| 28 | 부분 write가 최종 target 경로로 보이는가 | **PASS** — temp file을 fsync한 뒤 atomic rename한다. |
| 29 | local target이나 HTTPS가 오래된 bytes를 제공하는가 | **PASS** — local 2,257/2,257 및 HTTPS 2,257/2,257 SHA-256이 source와 일치한다. |
| 30 | browser 증거가 terminal browser 또는 남은 process를 사용했는가 | **PASS** — Agent Browser에서 page와 양 PNG를 확인하고 종료 뒤 executable process 0개를 확인했다. |

**경계:** Agent Browser page와 selected asset의 표시를 확인했지만 전체 문서 링크 집합, glyph pixels, iOS compositor 이후 pixels, hardware GPU 또는 C11 완료는 판정하지 않았다.
