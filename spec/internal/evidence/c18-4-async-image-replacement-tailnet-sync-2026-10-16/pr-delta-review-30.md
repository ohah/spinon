# C18.4 Tailnet 동기화 PR 변경 검토

검토 대상은 Tailnet C18.4 문서·자산 매니페스트, Agent Browser 제공 hash, 로드맵 카운터, 세 장의 화면 캡처, scoped build와 전체 docs build 경계 기록이다.

| # | 실패 관점 | 확인 |
| ---: | --- | --- |
| 1 | preview source가 병합 전 C18.4 branch인가 | source commit은 PR #228 병합 뒤 `main` commit `6c2c0bb5`다. |
| 2 | 문서 이미지 경로 수정이 sync source에 들어갔는가 | PR #228 수정 후 main tree에서 scoped build했다. |
| 3 | 구현 merge SHA와 docs source SHA를 혼동하는가 | PR #227 merge SHA와 PR #228 이후 source SHA를 manifest에서 분리했다. |
| 4 | 공식 상태 수를 이전 값으로 하드코딩하는가 | Agent Browser live roadmap에서 `71/202`를 읽었다. |
| 5 | C18.4 checkbox가 실제 체크됐는가 | C18.4 행 `checked=true`를 DOM에서 확인했다. |
| 6 | 전체 완료와 C18.4 한 항목 완료를 혼동하는가 | 202개 중 체크 71개로 별도 기록했다. |
| 7 | C18.4를 public image API로 확대하는가 | plan·contract가 opt-in fixture 제한과 미지원 범위를 유지한다. |
| 8 | C18/C14/U05 상위 완료를 주장하는가 | status 및 evidence에 상위 항목 미완료를 남겼다. |
| 9 | Tailnet 문서 전체를 무검토 복사하는가 | 226개 대상 경로와 8,532,397 bytes를 preflight에 고정했다. |
| 10 | 신규·교체·동일 수 합계가 전체와 다른가 | 211+10+5=226으로 일치한다. |
| 11 | 기존 파일을 삭제하는가 | manifest와 preflight 모두 삭제 0개다. |
| 12 | 교체본 이전 bytes가 보존되지 않는가 | 교체된 기존 파일 10개를 고유 backup root에 hash 검증해 보관했다. |
| 13 | backup root가 기존 백업을 덮는가 | apply 전에 root가 없음을 확인했고 실행 시 중복을 거부했다. |
| 14 | preflight 뒤 다른 프로세스가 대상 파일을 바꿨는가 | apply 시 모든 이전 hash를 다시 대조했다. |
| 15 | symbolic link를 따라 엉뚱한 경로를 쓰는가 | 대상과 부모의 symlink를 거부하는 apply guard를 사용했다. |
| 16 | `spec/` route와 flattened route가 한쪽만 갱신되는가 | 두 경로 집합을 manifest에 포함했다. |
| 17 | plan 문서 route가 잘못됐는가 | Agent Browser에서 plan route와 C18.4 본문을 열었다. |
| 18 | internal contract route가 잘못됐는가 | 계약 0067 route의 제목과 내용을 확인했다. |
| 19 | 공식 상태 HTML이 이전 C18.3에 머무는가 | `spec/STATUS.html` route에 C18.4를 확인했다. |
| 20 | 내부 README가 C18.4를 누락하는가 | 내부 인덱스에서 C18.4 링크 내용을 확인했다. |
| 21 | 구현 실행 근거 페이지가 이전 HTML인가 | Tailnet에서 C18.4 evidence route를 직접 열었다. |
| 22 | Android 화면 링크가 깨지는가 | `complete=true`, natural size `588×231`을 확인했다. |
| 23 | iOS 화면 링크가 깨지는가 | `complete=true`, natural size `1206×2622`를 확인했다. |
| 24 | Chrome broken-image oracle 링크가 깨지는가 | 병합된 `./oracle/...` 경로에서 `1280×633` 이미지를 읽었다. |
| 25 | auto-pending oracle 그림이 누락되는가 | inline data URI 이미지 `80×48`의 완료 상태를 확인했다. |
| 26 | manifest가 원본 파일 목록을 빠뜨리는가 | browser manifest의 226개와 전체 source file 검증 수가 일치한다. |
| 27 | served bytes가 local hash와 다른가 | Agent Browser Web Crypto 결과 `226/226`, 실패 0이다. |
| 28 | manifest 응답 자체가 틀리거나 깨졌는가 | HTTP 200, 96,802 bytes와 SHA-256을 기록했다. |
| 29 | 서버 재시작이나 Pages 배포를 몰래 수행하는가 | 기존 Tailnet service에 파일만 복사했고 Pages workflow는 실행하지 않았다. |
| 30 | 전체 docs build 실패를 숨기거나 scoped 성공으로 과장하는가 | 466개 dead-link 문서·unresolved import 12개와 별도 scoped build 결과를 구분 기록했다. |

세 화면, raw build logs, per-file preflight·served hash를 서로 분리해 보관했다. 전체 문서 build의 기존 한계와 C18.4 scoped build 결과는 별도 문서에 연결했다.
