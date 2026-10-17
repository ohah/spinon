# C13.3 API 29 Tailnet 근거 route PR 변경 실패 관점 검토 30개

검토 대상은 `spec/public/roadmap.html`, `plan/c13-3-native-wheel-input.md`의 API 29 링크와 Agent Browser route 근거다.

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 로드맵 링크가 이전 404 alias를 계속 가리키는가 | canonical `/spinon/internal/.../README.html`로 바꿨다. 통과. |
| 2 | 실제 route에 문서 파일이 없는가 | build output의 `internal/evidence/.../README.html`과 Tailnet canonical page가 있다. 통과. |
| 3 | HTTP 200만 보고 브라우저 렌더를 오판했는가 | alias는 HTTP 200이어도 Agent Browser에서 404였고 canonical route는 정상 렌더됐다. 통과. |
| 4 | 404 관찰과 성공 관찰이 같은 preview host인가 | 두 URL 모두 동일 Tailnet host에서 열었다. 통과. |
| 5 | 404 관찰과 성공 관찰의 문서 본문이 다른가 | 두 파일의 HTTP body가 같은 30,259 bytes·SHA-256이다. 통과. |
| 6 | 고친 링크가 Rspress route alias에 의존하는가 | route table의 `/internal/...` 경로를 사용한다. 통과. |
| 7 | roadmap에서 API 29 근거가 빠졌는가 | API 29 실행 근거 텍스트와 URL을 보존했다. 통과. |
| 8 | C13.3 계획의 첫 API 29 실행 링크가 고장 경로를 가리키는가 | canonical route에 맞는 상대 `../internal/.../README.html`로 바꿨다. 통과. |
| 9 | 계획의 반복 API 29 링크만 옛 경로에 남는가 | 하단 링크까지 함께 바꿨다. 통과. |
| 10 | 구현 검토 링크가 raw Markdown 경로로 남는가 | `implementation-review-20.html` route를 쓴다. 통과. |
| 11 | PR 변경 검토 링크가 raw Markdown 경로로 남는가 | `pr-delta-review-30.html` route를 쓴다. 통과. |
| 12 | `README.html`과 계획의 상대 route 깊이가 잘못됐는가 | 계획이 `/plan/` route이므로 `../internal/`이 `/internal/`로 해석된다. 통과. |
| 13 | HTML roadmap 링크의 base path가 빠졌는가 | 기존 custom roadmap 규칙대로 `/spinon/internal/...`을 사용한다. 통과. |
| 14 | `main` 사이트 base와 Tailnet base가 다르게 적용되는가 | 두 preview가 모두 `/spinon/` base를 쓴다. 통과. |
| 15 | 같은 파일명의 다른 evidence route로 연결되는가 | C13.3 API 29 evidence ID와 경로를 대조했다. 통과. |
| 16 | 링크를 바꾸며 API 29 실행 요약 문장을 삭제했는가 | 기존 수치·제한 설명은 유지했다. 통과. |
| 17 | Android API 29 AVD를 실기기로 확대 표현했는가 | AVD 경계 문구를 유지했다. 통과. |
| 18 | virtual mouse를 물리 trackpad로 오인하는가 | 기존 virtual-input·물리 입력 미검증 설명을 유지했다. 통과. |
| 19 | iOS callback/sign이 확인된 것처럼 보이는가 | 미관찰·미완료 문구를 유지했다. 통과. |
| 20 | C13.3 공식 완료 checkbox를 변경했는가 | `spec/STATUS.md`는 건드리지 않았다. 통과. |
| 21 | 공식 완료 카운트가 바뀌는가 | STATUS 집계 변경이 없다. 통과. |
| 22 | 이전 alias 경로 자체를 삭제하거나 리다이렉트했다고 주장하는가 | alias는 유지하고 문서 링크만 canonical route로 옮겼다. 통과. |
| 23 | raw source Markdown 링크와 렌더 페이지 링크를 혼동하는가 | 새 API 29 근거 링크는 브라우저 렌더 `.html` route를 가리킨다. 통과. |
| 24 | Agent Browser screenshot을 제품 실행 화면으로 오인하는가 | README에서 route 화면으로만 분류한다. 통과. |
| 25 | screenshot 중 하나가 API 29 AVD runtime screenshot으로 위장되는가 | README 캡처는 Tailnet Agent Browser 화면이며 source labels로 구별된다. 통과. |
| 26 | AVD 화면 이미지가 새로 로드되지 않는 route를 증명하는가 | canonical 페이지에서 두 이미지 natural size와 `complete` 상태를 검사했다. 통과. |
| 27 | 고친 route가 C13.3 API 29 이미지 증거를 누락하는가 | evidence `README.html`에서 두 PNG를 확인했다. 통과. |
| 28 | 이미 확인된 원본 page body hash와 기록이 다르게 적혔는가 | local·Tailnet HTTP body에서 SHA-256을 대조했다. 통과. |
| 29 | 문서 변경이 Chrome/Stylo 색상 판정에 영향을 주는가 | C19 문구와 링크는 변경하지 않았다. 통과. |
| 30 | route 수정 결과를 제품 CSS 적합성으로 확대하는가 | link/render 수정만 다루며 기능 호환 주장은 추가하지 않았다. 통과. |

## 결론

발견된 문제는 `/spec/internal` alias가 HTTP 200 뒤 Rspress 404를 표시하는 route 불일치였다. 사용자 링크를 실제 렌더링되는 canonical route로 수정했다. 다른 결함은 발견되지 않았다.
