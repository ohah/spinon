# Tailnet 동기화 PR 변경 실패 관점 30개

기준 원본은 병합된 PR #261 이후의 `740b64032a68cd24ae258d341cefcfc2a3fe8984`다. 각 항목은 문서 렌더, copy-only 전파, route와 실제 served response에 별도로 적용한다.

| # | 실패 관점 | 판정 기준 | 결과 |
|---:|---|---|---|
| 1 | sync 입력이 PR #261 병합 전 commit으로 돌아감 | 빌드 입력 HEAD가 main `740b64032a68cd24ae258d341cefcfc2a3fe8984`와 일치 | 통과: 고정 source revision 기록과 일치 |
| 2 | custom roadmap만 갱신되고 두 경로 중 하나가 오래됨 | `/roadmap.html`, `/public/roadmap.html` bytes가 source `spec/public/roadmap.html`과 일치 | 통과: 두 route source SHA가 `fa59fd5c…113d979`로 같음 |
| 3 | official status에서 완료 수가 잘못 바뀜 | Tailnet STATUS가 `61/197`과 C15.3 미완료를 표시 | 통과: 최신 STATUS 본문과 roadmap에 그대로 표시 |
| 4 | 제한 구현이 C15 전체 완료로 승격됨 | roadmap·STATUS·계약·evidence에 C15.3/C15 미완료 경계 유지 | 통과: 네 route 모두 범위 한계를 명시 |
| 5 | 최신 C15 plan route가 누락되거나 이전 내용임 | `plan/c15-inline-text-layout.html` hash가 선택 build와 일치 | 통과: local/Tailnet served SHA가 manifest와 같음 |
| 6 | 내부 색인의 0074 연결이 이전 계약으로 남음 | `internal/README.html`에 0074와 구현 근거 링크 존재 | 통과: Agent Browser snapshot에서 두 link 확인 |
| 7 | 0074 route의 실제 페이지가 누락됨 | `/internal/0074-c15-3-runtime-glyph-scroll-transform.html` 응답 200 | 통과: title, reset sequence, 완료 경계 본문 확인 |
| 8 | 기존 source mirror가 낡아 직접 경로가 다름 | `/spec/internal/0074-...html`이 flat route와 같은 bytes | 통과: mirror와 flat route 모두 manifest SHA 일치 |
| 9 | Chrome oracle와 구현 근거가 서로 바뀜 | precomparison README는 계획 oracle, runtime README는 구현 표면 결과로 분리 | 통과: 두 경로와 제목을 개별 확인 |
| 10 | Android 표면을 physical GPU 증거로 확대 해석 | 페이지에 API 37 ARM64 AVD/software 경계 유지 | 통과: 실행 README와 evidence 경계에서 명시 |
| 11 | iOS drawable readback을 compositor 이후 출력으로 표현 | 페이지에 Metal drawable의 compositor 전 한계 명시 | 통과: 실행 README와 계약에서 제출 전으로 한정 |
| 12 | DOM text admission 또는 일반 CSS 지원을 암시 | C15.3/C15 제외 범위가 route에 보임 | 통과: 일반 DOM admission·glyph hit 등 제외 명시 |
| 13 | README의 상대 PNG 링크가 preview에서 깨짐 | 각 로컬 asset 경로를 target inventory와 HTTP 응답으로 확인 | 통과: Android/iOS PNG 200, length/SHA exact |
| 14 | README의 log 링크가 route에서 404 | Android/iOS 실행 로그와 환경·검증 로그가 200 | 통과: 선택한 evidence asset 응답 모두 200/hash exact |
| 15 | markdown rendering이 asset을 다른 hashed path로 이동 | 생성된 HTML의 image/src와 실제 asset SHA를 대조 | 통과: hashed Chrome PNG가 static route로 제공됨 |
| 16 | base path가 `/spinon/spinon/`으로 중복됨 | HTML의 CSS·JS·navigation URL이 `/spinon/` 아래 한 번만 구성 | 통과: Agent Browser에서 base URL 및 route 이동 정상 |
| 17 | static Rspress CSS/JS가 선택 build에는 있으나 preview에서 누락 | HTML에서 참조하는 각 asset이 preview HTTP 200 | 통과: 133개 local/Tailnet URL 전체 200/hash exact |
| 18 | stale search index가 0074 문서를 감춤 | 생성 search index가 0074 또는 runtime scroll 용어를 포함 | 통과: 새 선택 build search index 포함 및 HTTP SHA 일치 |
| 19 | Rspress가 preview root를 정리해 기존 파일을 삭제 | copy manifest는 명시적 파일만 기록하고 delete 목록은 비어 있음 | 통과: preflight·manifest 모두 `deletions: []` |
| 20 | 다른 내용의 기존 파일을 backup 없이 덮음 | 변경 target은 copy 전 SHA와 backup SHA가 같음 | 통과: 교체 19개 전부 backup hash가 이전 hash와 같음 |
| 21 | backup을 새 preview route로 노출해 페이지를 오염 | backup 경로는 preview root 밖이거나 static route 대상에서 제외 | 통과: backup은 `SpinonPreviewBackups`에 저장 |
| 22 | source bytes가 preflight 이후 달라진 채 복사됨 | copy 직전 source SHA가 preflight SHA와 일치 | 통과: copy script 불일치 시 중단, 최종 hash 일치 |
| 23 | 로컬 target 복사 완료와 HTTP 제공 성공을 혼동 | local HTTP response SHA와 bytes가 manifest와 일치 | 통과: 133/133 response body exact |
| 24 | Tailnet Funnel 캐시/host가 다른 응답을 제공 | public Tailnet HTTP status·content length·SHA를 다시 확인 | 통과: 133/133 public response body exact |
| 25 | HTTP 200 error page를 정상 route로 오판 | title·고유 C15.3 heading·0074 text가 응답 본문에 존재 | 통과: roadmap와 0074 고유 본문을 browser에서 확인 |
| 26 | browser 이미지가 깨져도 route가 열린 것으로 처리 | Agent Browser에서 image `complete`와 `naturalWidth/Height` 확인 | 통과: Android 1080×2400, iOS 1206×2622 |
| 27 | Agent Browser 캡처를 제품 앱 화면으로 오인 | README는 캡처를 Tailnet 문서 화면 자료라고 표시 | 통과: 정적 페이지 캡처와 앱 표면 결과를 분리 |
| 28 | 전용 serial/실기기를 연결하거나 사용했다고 과장 | sync 작업은 시뮬레이터 실행을 다시 수행하지 않고 기존 한계만 인용 | 통과: 새 native/device 실행 없음, 기존 경계 유지 |
| 29 | preview listener 또는 tmux를 cleanup 중 종료 | PID 1191 listener와 tmux 상태를 보존 | 통과: listener 응답 유지, tmux는 조작하지 않음 |
| 30 | browser cleanup이 실제 Tailnet listener까지 종료 | Agent Browser 프로세스만 종료하고 8768 listener HTTP 200 유지 | 통과: page 확인 뒤 Agent Browser 세션을 닫았고 해당 Chrome process group이 사라졌으며 listener는 HTTP 200으로 남았다 |

Rspress 선택 build는 저장소 전체 docs validation으로 세지 않는다.
