# Tailnet 동기화 PR diff 검토 · 30개 관점

범위는 `spec/public/roadmap.html`의 PR #354 후속 note와 이번 Tailnet build/copy/browser evidence다. C11.4 제품 구현 diff는 PR #354에서 이미 검토·병합됐다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | roadmap note가 완료 수를 직접 고정하는가 | 통과 — 카운터는 공식 상태 대장을 읽는 기존 코드를 유지한다. |
| 2 | C11.4 체크나 C11 상위 완료를 바꾸는가 | 통과 — 둘 다 미완료다. |
| 3 | 완료 수가 실제 대장과 다른가 | 통과 — Agent Browser에서 72/212를 확인했다. |
| 4 | PR #354가 main에 병합되기 전에 병합됐다고 표시하는가 | 통과 — source base는 merge commit `08caa59b`다. |
| 5 | note의 고정 트랙 설명이 구현 범위를 과장하는가 | 통과 — single fixed track 후보 범위와 제한을 명시했다. |
| 6 | CRD 초안과 Chrome 차이를 Chrome 결함이라고 단정하는가 | 통과 — 상호 운용 차이로 표현했다. |
| 7 | Chrome row/column 관측값이 반대로 적혔는가 | 통과 — 12px row와 27px columns를 대조했다. |
| 8 | CRD에서 도출한 5.333px/12px 값을 누락하는가 | 통과 — 계획, 상태, roadmap note에 값이 있다. |
| 9 | 9-scene 결과를 새 span case 각각 9개로 오해하게 하는가 | 통과 — matrix 총계와 신규 overflow 표본을 분리해 썼다. |
| 10 | PixelCopy/Metal 표본 수를 전체 화면 검증으로 확대하는가 | 통과 — 신규 표본 각 플랫폼 3/3으로 한정했다. |
| 11 | 실제 기기 결과로 오해할 수 있는가 | 통과 — AVD와 Simulator라고 표기한다. |
| 12 | PNG texel paint를 확인했다고 주장하는가 | 통과 — 미검증으로 적었다. |
| 13 | multi-track/gutter 범위를 완료로 오해하게 하는가 | 통과 — 남은 범위에 포함했다. |
| 14 | writing mode·percentage/calc·WPT를 빠뜨리는가 | 통과 — 남은 검증으로 표기했다. |
| 15 | 상태 대장 evidence 링크가 올바른가 | 통과 — README, 구현 검토, PR diff 검토 URL이 열린다. |
| 16 | 링크가 `/spinon/` 밖으로 나가는가 | 통과 — Tailnet DOM에서 내부 링크가 모두 prefix 아래다. |
| 17 | 중첩 `/spinon/spinon/` 경로를 만들었는가 | 통과 — 서버가 prefix를 제거하고 `SpinonPreview` 루트를 사용한다. |
| 18 | preview root 확인 없이 파일을 썼는가 | 통과 — `tailscale serve status`, 서버 source, 로컬 HEAD 응답을 확인했다. |
| 19 | 동기화 과정이 preview server를 종료하는가 | 통과 — 기존 서버를 유지했다. |
| 20 | 사용하지 않은 파일을 삭제하는가 | 통과 — 삭제 0건이다. |
| 21 | 변경 전 기존 파일을 복구할 방법이 있는가 | 통과 — 변경 대상 18개와 후속 `roadmap.html`을 별도 backup으로 보존했다. |
| 22 | 덮어쓰기 도중 부분 파일을 서빙할 수 있는가 | 통과 — 대상 디렉터리 임시 파일, fsync, atomic rename을 사용했다. |
| 23 | source hash와 파일 크기를 검증했는가 | 통과 — 입력·렌더·copy manifest에 기록했다. |
| 24 | preview server 응답 바이트가 원본과 다른가 | 통과 — pass 2에서 HTTP 200 및 SHA-256 345/345가 일치한다. |
| 25 | 전체 문서 build로 오해하게 하는가 | 통과 — Markdown route 16개 선택 빌드라고 분명히 적었다. |
| 26 | dead-link/image 검사를 실행했다고 오해하게 하는가 | 통과 — 이번 선택 빌드에서 해당 옵션을 끈 사실을 기록했다. |
| 27 | 전체 문서 dead-link 검사를 통과했다고 주장하는가 | 통과 — 전체 문서 적합성은 주장하지 않는다. |
| 28 | Tailnet 확인을 Terminal Browser 결과로 표기하는가 | 통과 — Agent Browser session과 실제 Tailnet URL을 기록한다. |
| 29 | 브라우저 screenshot의 이미지 로드가 실패했는가 | 통과 — Android 1080×2400, iOS 1206×2622 natural size로 확인했다. |
| 30 | 작업 후 Agent Browser process가 남는가 | 통과 — 지정 session을 종료한 뒤 상태와 process를 별도로 확인한다. |

미해결된 PR 범위 문제는 없다. 이 검토는 전체 RSPress dead-link 검사, C11.4 적합성, 실제 기기 또는 GitHub Pages 배포를 대신하지 않는다.
