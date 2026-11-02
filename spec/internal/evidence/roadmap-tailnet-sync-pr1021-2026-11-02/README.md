# PR #1021 병합 후 Tailnet roadmap 동기화 근거

## 범위

원본은 PR #1021 Rebase and merge 뒤 `main`의 source HEAD `004291f5033942f9037d3e8a0ee96658bbd7595d`다. 공개 roadmap에 C11.1 percentage-track의 PR #1019 병합 결과를 추가했다. 공식 상태 대장과 완료 수 `72/213`은 바꾸지 않았다.

기존 preview 전체를 다시 만들거나 삭제하지 않고 아래 다섯 경로만 갱신했다.

| 경로 | 결과 | SHA-256 |
|---|---:|---|
| `roadmap.html` | 기존 파일 교체 | `96a35b484e5f0ff5cf5be18a4ee8ccb26e31f7e180e30e19927e07d04e0b747c` |
| `public/roadmap.html` | 기존 파일 교체 | `96a35b484e5f0ff5cf5be18a4ee8ccb26e31f7e180e30e19927e07d04e0b747c` |
| `spec/public/roadmap.html` | 기존 파일 교체 | `96a35b484e5f0ff5cf5be18a4ee8ccb26e31f7e180e30e19927e07d04e0b747c` |
| `internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02/android-api37-avd-percentage-overflow.png` | 새 파일 추가 | `cf306c2984e60f12085c90cd33bb256a032a6e7e7d9636dad975c2ba6effb9b6` |
| `internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02/ios-26_2-simulator-percentage-overflow.png` | 새 파일 추가 | `fd3e9b2326af9ede0a7452554da19069cff072f7e7202c212af3de7186587e5f` |

병합 전 세 roadmap HTML의 SHA-256과 bytes는 [선택 route manifest](./route-publication-manifest.json)에 있다. 세 파일을 사용자 지원 백업 경로에 먼저 복사하고 해시를 대조했다. 대상 파일을 연 프로세스가 없어 temp file 작성 후 같은 디렉터리의 atomic rename으로 교체했다. 새 화면 경로 두 개를 추가했고 기존 route 삭제는 없었다. Tailscale 서비스와 preview server PID `28040`은 재시작하지 않았다.

## HTTP 및 화면 확인

- Tailscale backend는 `Running`; 기존 `tailnet-preview-server.mjs` listener는 단일 PID `28040`이었다.
- 다섯 route 각각을 localhost와 `https://macstudio.tailed42f2.ts.net/spinon/`에서 요청했다. 응답 10개 모두 HTTP 200이고 byte length 및 SHA-256이 `main` 원본과 일치한다. 상세 응답은 [served verification](./tailnet-served-verification.json)에 있다.
- Agent Browser에서 [Tailnet roadmap](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 열어 PR #1019, merge SHA, 공식 `72/213` 문구와 새 화면 링크를 확인했다. Android PNG는 1080×2400, iOS PNG는 1206×2622로 둘 다 로드됐다.
- 데스크톱 1280×800 캡처: [roadmap card](./tailnet-roadmap-percentage-desktop.png).
- 모바일 390×844 캡처에서 `documentElement.scrollWidth`는 viewport와 같은 390px이었다: [roadmap card](./tailnet-roadmap-percentage-mobile.png).
- 확인 후 `agent-browser close --all`을 실행했다. Agent Browser session은 `active=false`, `pid=null`이고 `agent-browser` PID는 없었다.

## 보존과 한계

게시 과정은 다섯 파일의 해시만 검사했으며 preview의 다른 route를 전수 재해시하지 않았다. 사이트 전체 RSPress build, 전체 route mirror, 공식 CSS 지원, C11 완료 또는 상태 집계 변경을 주장하지 않는다. 교체 전 HTML 세 개의 백업과 route별 preflight/copy 기록은 보존한다.

세부 재현 스크립트는 [`sync-selected-routes.mjs`](./sync-selected-routes.mjs)이며 `preflight`, `copy`, `verify` 단계를 제공한다. 각 단계의 source HEAD와 checkout 상태를 검사한다.

이 PR의 30개 독립 실패 관점 검토는 [`tailnet-sync-review-30.md`](./tailnet-sync-review-30.md)에 있다. 이 디렉터리의 재현 스크립트와 보고서, 캡처는 [`SHA256SUMS.txt`](./SHA256SUMS.txt)로 무결성을 확인할 수 있다.
