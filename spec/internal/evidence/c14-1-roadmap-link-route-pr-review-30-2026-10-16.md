# C14.1 Tailnet 근거 링크 경로 PR 적대 검토 · 30개 관점

- 검토일: 2026-10-16
- 변경 대상: `spec/public/roadmap.html`의 C14.1 실행 근거 링크
- 재현 URL: `/spinon/spec/internal/evidence/c14-1-live-v8-image-measurement-2026-10-16/README.html`
- 정상 Rspress URL: `/spinon/internal/evidence/c14-1-live-v8-image-measurement-2026-10-16/README.html`
- 판정: 실제 Agent Browser에서 기존 URL의 hydration 404와 정상 route 표시를 확인했다. 새 링크는 생성된 Rspress route로 맞춘다. 미해결 P1/P2 없음.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | roadmap의 새 C14.1 링크가 Tailnet에서 실제로 표시되는가 | Agent Browser에서 기존 `/spec/internal/...` URL은 hydration 뒤 404가 됐고, `/internal/...` URL은 “C14.1 live V8 문서 → PNG 자연 크기 측정 실행 근거” 제목으로 열렸다. 통과. |
| 2 | 변경 경로가 Rspress의 `base=/spinon/` 설정과 맞는가 | generated route prefix `/spinon/internal/...`를 유지하며 base를 중복 삽입하지 않는다. 통과. |
| 3 | `spec/` 접두어가 route 안에 잘못 남는가 | 기존 링크에서만 `/spec/`를 제거한다. generated page route는 `/internal/`이다. 통과. |
| 4 | `spinon` base segment를 실수로 제거하는가 | 링크는 `/spinon/internal/...`를 유지한다. 통과. |
| 5 | evidence 디렉터리 이름의 철자·날짜가 실제 산출물과 일치하는가 | `c14-1-live-v8-image-measurement-2026-10-16`이 build output 경로와 정확히 같다. 통과. |
| 6 | 대상 파일명 대소문자나 확장자가 실제 산출물과 다른가 | `README.html` route 파일명을 scoped build 산출물과 대조했다. 통과. |
| 7 | 링크가 Markdown source 파일로 가 browser router에서 재처리되는가 | href는 생성된 `.html` route를 가리켜 raw `.md`와 route를 혼동하지 않는다. 통과. |
| 8 | 브라우저가 링크를 상대 경로로 잘못 해석하는가 | root-relative URL이라 현재 페이지의 깊이에 영향을 받지 않는다. 통과. |
| 9 | 올바른 C14.1 evidence가 아닌 다른 날짜의 페이지로 연결되는가 | 동일 날짜의 live V8 execution README에 도달함을 Agent Browser title로 확인했다. 통과. |
| 10 | 링크 텍스트가 변경되어 문서의 뜻이 바뀌는가 | “실행·화면·artifact 근거” 텍스트는 유지한다. 통과. |
| 11 | giant single-line HTML 편집이 주변 요약 문장을 바꾸는가 | diff와 before/after anchor를 확인했다. 의도한 href 한 곳만 변경한다. 통과. |
| 12 | C14.1 설명이 synthetic evidence를 live V8 결과로 오인하게 되는가 | link 수정은 설명 문장을 건드리지 않으며, 본문은 live V8 경계와 남은 paint 범위를 유지한다. 통과. |
| 13 | C14.1 완료 checkbox를 바꾸는가 | 공식 상태 대장 체크는 변경하지 않는다. 통과. |
| 14 | 전체 완료 카운터가 영향을 받는가 | 체크 상태 변경이 없어 기대값은 65/197이다. Tailnet Agent Browser에서 65 checked / 197 total을 확인했다. 통과. |
| 15 | 이미지·PNG 링크를 evidence page route로 잘못 치환하는가 | 수정 대상은 README HTML page href 하나뿐이며 screenshot/PNG 경로는 바뀌지 않는다. 통과. |
| 16 | 기존 C13/R05 evidence URL을 광범위 replace로 손상시키는가 | 다른 `/spinon/spec/internal/...` URL은 그대로 둔다. 통과. |
| 17 | GitHub main 상태 링크를 Tailnet route로 바꾸는가 | 공식 상태 링크와 동적 fetch URL은 변경하지 않는다. 통과. |
| 18 | public roadmap의 본문 요약이나 C14.1의 미완료 설명이 사라지는가 | 변경은 anchor destination만 바꾼다. 통과. |
| 19 | 변경한 href가 Tailnet 정적 root의 존재하는 산출물로 연결되는가 | scoped build가 만든 `internal/evidence/.../README.html`을 sync manifest 대상에 포함했다. 통과. |
| 20 | 정상 route가 page title뿐 아니라 내용을 보여주는가 | Agent Browser에서 generated page title과 README 본문이 렌더링됨을 확인했다. 통과. |
| 21 | nested route가 HTTP 200이라 표시도 성공했다고 오판하는가 | nested path를 Agent Browser에서 열어 hydration 404를 관찰했다. HTTP 상태만으로 성공 처리하지 않았다. 통과. |
| 22 | 새 정상 링크가 앱 route 대신 서버 404 문서로 연결되는가 | root route `/spinon/internal/...`에서 Rspress client가 페이지를 렌더링함을 확인했다. 통과. |
| 23 | build output을 만들기 위해 저장소 Rspress 설정을 바꾸는가 | route 선택 build는 임시 source/config에서 수행하고 저장소 config를 바꾸지 않았다. 통과. |
| 24 | 변경이 GitHub Pages 배포나 preview server 재시작을 요구하는가 | 이 PR은 static roadmap href 수정만 다룬다. Pages workflow 실행이나 서비스 재시작은 하지 않는다. 통과. |
| 25 | Tailnet 정적 사본이 merge 뒤 source와 다른 채 남는가 | 병합 후 동일 source HTML을 root·public·spec/public 세 사본에 해시 확인해 sync할 계획이다. 통과. |
| 26 | PR 변경이 app behavior 또는 simulator 결과를 새로 주장하는가 | 앱 코드와 실행 evidence는 변경하지 않고 기존 C14.1 경계만 링크한다. 통과. |
| 27 | Android/iOS Simulator screenshots를 새 링크 검증 근거로 과장하는가 | 이번 검증은 Agent Browser route 표시이며 앱 화면/paint 근거로 쓰지 않는다. 통과. |
| 28 | 별도 preview path를 터미널 브라우저나 전역 browser 종료로 처리하는가 | Agent Browser 전용 세션에서 URL을 확인한다. terminal-browser/tmux 조작은 없다. 통과. |
| 29 | 고정된 PR diff가 whitespace 또는 우발 변경을 포함하는가 | `git diff --check`와 파일별 diff를 확인하고 href 변경 외 수정이 없다. 통과. |
| 30 | merge 후 link regression을 판정할 수 있는 관찰 조건이 빠져 있는가 | Agent Browser에서 roadmap link를 열어 C14.1 evidence heading/body를 확인하고 65/197 count와 404 이미지 여부를 함께 기록한다. 통과. |

## 검증 경계

- 이 수정은 Tailnet roadmap에서 C14.1 evidence route로 가는 링크만 바로잡는다.
- official status는 65/197이며 C14.1과 C14는 미완료다.
- merged page 이후 Tailnet preview를 다시 동기화하고 Agent Browser에서 최종 link를 확인한다.
