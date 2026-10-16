# C18.4 Chrome oracle 이미지 경로 문서 PR 검토

두 Markdown 이미지 참조의 Rspress 상대 경로 처리를 검토했다. scoped build 전에 `oracle/...`가 MDX module 요청으로 해석되어 빌드 실패하는 것을 재현했다. `./oracle/...`로 수정한 뒤 같은 빌드를 다시 실행한다.

| # | 실패 관점 | 확인 |
| ---: | --- | --- |
| 1 | 현재 문서 폴더 기준 상대 경로인가 | 두 경로 모두 `./oracle/`로 시작한다. |
| 2 | 첫 PNG가 실제 저장소에 있는가 | `oracle/chrome155-broken.png`가 존재한다. |
| 3 | 두 번째 PNG가 실제 저장소에 있는가 | `oracle/chrome155-auto-pending-visible.png`가 존재한다. |
| 4 | 경로 대소문자 불일치가 있는가 | 경로 component와 저장 파일명이 일치한다. |
| 5 | 문서 위치 변경 없이 해결되는가 | README 위치는 그대로고 하위 `oracle/` 경로를 유지한다. |
| 6 | MDX가 module import로 잘못 읽는가 | 수정 전 scoped build가 `Can't resolve 'oracle/...'`로 실패했다. |
| 7 | 수정 뒤 Rspress가 두 파일을 렌더하는가 | 첫 PNG는 hashed `static/image` asset, 80×48 PNG는 inline data URI로 생성 HTML에 들어갔다. |
| 8 | 첫 그림 alt text가 유실되는가 | 한국어 alt text는 변경하지 않았다. |
| 9 | 두 번째 그림 alt text가 유실되는가 | 한국어 alt text는 변경하지 않았다. |
| 10 | 손상 PNG fixture 바이트가 바뀌는가 | 이미지 파일은 수정하지 않았다. |
| 11 | auto-pending fixture 이미지가 바뀌는가 | 이미지 파일은 수정하지 않았다. |
| 12 | 파일 hash 표와 이미지 hash가 달라지는가 | 문서의 표와 PNG bytes는 변경하지 않았다. |
| 13 | 그림과 인접 설명의 순서가 바뀌는가 | 두 링크의 앞뒤 문단 순서는 그대로다. |
| 14 | 이미지가 외부 URL로 바뀌는가 | 두 경로 모두 로컬 상대 경로다. |
| 15 | site base path에서 경로가 깨지는가 | 상대 경로라 `/spinon/` base에 의존하지 않는다. |
| 16 | preview route와 이미지 route가 다른 origin을 쓰는가 | Rspress가 같은 origin의 hashed image asset으로 번들한다. |
| 17 | README direct Markdown route가 파일을 찾는가 | Tailnet 동기화에는 원본 README와 하위 `oracle/` 파일을 함께 포함한다. |
| 18 | `spec/` prefix flattening이 이미지 경로를 바꾸는가 | 문서와 `oracle/` 디렉터리를 동일 상대 구조로 복사한다. |
| 19 | 렌더 HTML이 이전 이미지로 남는가 | 새 scoped output의 두 이미지 `src`와 alt text를 검사했다. |
| 20 | 한 화면만 수정되어 다른 화면이 빠지는가 | 두 이미지 경로를 함께 수정하고 모두 build output에서 확인한다. |
| 21 | 렌더된 이미지가 완전하게 로드되는가 | Agent Browser 0.31.1에서 첫 이미지 1280×633, 두 번째 80×48, 둘 다 `complete=true`였다. |
| 22 | 이미지 응답이 실패하는가 | 로컬 scoped preview에서 hashed PNG HTTP 응답을 브라우저가 성공적으로 표시했다. |
| 23 | 이미지가 잘리거나 빈 표면으로 나타나는가 | [Agent Browser 캡처](./c18-4-async-image-oracle-scoped-build-agent-browser.png)에서 두 oracle 화면이 렌더됐다. |
| 24 | 저장소 전체 문서 빌드 결과로 과장하는가 | 검증 범위는 이 작업에 한정한 scoped build다. |
| 25 | C18.4 oracle을 제품 동작으로 오해하는가 | oracle 설명과 제한 profile 문구는 수정하지 않았다. |
| 26 | 공개 지원 범위가 넓어지는가 | 계약·상태 대장의 완료 범위는 바꾸지 않았다. |
| 27 | C18.4 runtime 화면과 Chrome oracle 화면을 혼동하는가 | 파일 이름·alt text·각 README의 역할을 유지한다. |
| 28 | 다른 문서 링크나 이미지 참조를 함께 손상시키는가 | diff에는 두 이미지 href만 변경된다. |
| 29 | 공백 오류가 diff에 들어가는가 | PR 전 `git diff --check`를 실행한다. |
| 30 | 병합 후 main revision을 검증과 혼동하는가 | Tailnet sync는 이 수정 PR 병합 뒤의 main commit을 source로 삼는다. |

Tailnet 자체의 URL·HTTP·served hash 확인은 후속 동기화 근거에 따로 기록하며 이 로컬 build 결과와 혼동하지 않는다.
