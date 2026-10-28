# PR #738 C11.4 roadmap Tailnet 동기화

상태/계획/공개 roadmap PR [#739](https://github.com/ohah/spinon/pull/739)이 2026-10-28T06:13:43Z Rebase and merge된 main commit `23304539ac9d6fab0e79e2bb8cc1154e77cc8469`의 `spec/public/roadmap.html` 한 파일만 Tailnet preview로 동기화했다. C11.4 상위는 계속 미완료이며 공식 완료 수는 `72/213`이다.

## 사전 상태와 복구본

- Tailscale backend는 `Running`, Tailnet host는 online이었다.
- 대상은 `/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html` regular file이며 기존 로컬 preview server PID `28040`의 listener가 포트 `8768`에서 유지됐다. 실행 중인 요청 연결이나 대상 파일 handle은 없었다. 별도 static preview PID 33319는 포트 4174 listener-only로 관찰했고 종료하거나 재시작하지 않았다. 상세 inventory는 [`server-process-inventory.json`](server-process-inventory.json)에 보존했다.
- 기존 대상 byte 수와 hash는 `361424`, `ecb1845d51e5eff1407f863ff591317df43127479d2b6efd71518198dab86709`였다. 이는 새 roadmap 이전 main commit `bf4ab21eebc2f3739a7ae61de6511b92733b9464`의 바이트와 exact match했다.
- 복구본은 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr738-2026-10-28/pass01/roadmap.html`에 byte-exact 복사했다. 복구본은 361424 bytes이며 이전 대상과 SHA-256이 같다.

## 교체와 확인

같은 preview 디렉터리에 새 파일을 쓴 뒤 `roadmap.html`로 atomic rename했다. 다른 route를 바꾸거나 파일을 삭제하지 않았고 preview server를 재시작하지 않았다.

| 확인 | HTTP | bytes | SHA-256 |
| --- | ---: | ---: | --- |
| 변경 전 local `127.0.0.1:8768` | 200 | 361424 | `ecb1845d51e5eff1407f863ff591317df43127479d2b6efd71518198dab86709` |
| 변경 전 Tailnet | 200 | 361424 | `ecb1845d51e5eff1407f863ff591317df43127479d2b6efd71518198dab86709` |
| 변경 후 local `127.0.0.1:8768` | 200 | 363975 | `0adb06aff0f5c2733a02b20d18099ccad2c6093e060b164a4f8ccad612483ea6` |
| 변경 후 Tailnet | 200 | 363975 | `0adb06aff0f5c2733a02b20d18099ccad2c6093e060b164a4f8ccad612483ea6` |
| merged main 원본 | — | 363975 | `0adb06aff0f5c2733a02b20d18099ccad2c6093e060b164a4f8ccad612483ea6` |

Agent Browser의 실제 Tailnet 화면은 제목 `스피논 구현 로드맵`, PR #738 카드와 merge SHA `bf4ab21eebc2f3739a7ae61de6511b92733b9464`, 공식 `72/213`, 계획·evidence·Android/iOS screenshot 링크를 확인했다. 카드가 뷰포트에 들어온 상태의 1280×633 캡처는 [`tailnet-roadmap-agent-browser.png`](tailnet-roadmap-agent-browser.png)에 있다. DOM 관찰·링크·hash는 [`agent-browser-observation.json`](agent-browser-observation.json)에 기록했다. 브라우저 session을 닫고 session list가 비었으며 이 확인에서 연 Agent Browser 프로세스가 없음을 확인했다.

## 재현 및 범위

`mise exec -- node spec/internal/evidence/roadmap-tailnet-sync-pr738-2026-10-28/sync-roadmap-only.mjs`는 현재 main SHA, 직전 대상 hash, Tailnet health, preview PID/연결, source·route hash, 고유 백업 경로를 fail-closed 검사하고 한 route만 교체한다. 실행 전 결과 파일이 있으면 덮어쓰지 않는다.

이 작업은 Tailnet preview의 정적 `roadmap.html` 파일만 갱신했다. Rspress 전체 build, GitHub Pages 배포, 앱/Simulator 실행 또는 CSS 기능 범위 변경은 포함하지 않는다. 사전 상태는 [`preflight.json`](preflight.json), 교체 결과는 [`sync-result.json`](sync-result.json)에 있다.
