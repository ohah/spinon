# C04.12 Tailnet 동기화 근거 PR 변경 적대 검토 30개

이 문서는 동기화 스크립트/근거 변경, Tailnet 미리보기 상태, Rspress 선택 산출물, 해시·백업 기록 및 브라우저 화면의 서로 다른 실패 경로를 검토한다.

| # | 실패 관점 | 실행 결과 |
| --- | --- | --- |
| 1 | PR #272 병합 전 source commit을 미리보기 결과라고 기록하는가 | `54a39a3e7403dd7fc9178de17daeafec4f00ec88`에서 빌드·동기화했다. 통과. |
| 2 | PR 번호나 merge SHA가 원본 링크와 맞지 않는가 | PR #272 상태와 merge SHA가 같은 `origin/main`에 있는 것을 확인했다. 통과. |
| 3 | Tailnet base URL이 prior listener 경로와 다른가 | 기존 `https://macstudio.tailed42f2.ts.net/spinon/` 경로를 사용했다. 통과. |
| 4 | 다른 preview root를 덮어쓰는가 | `/Users/yoonhb/Library/Application Support/SpinonPreview`를 preflight에 고정했다. 통과. |
| 5 | root 문서 빌드가 repository 전체를 대상으로 도는가 | 병합 source의 선택 문서 트리에서 22개 HTML route만 렌더했다. 통과. |
| 6 | subset 빌드를 전체 dead-link 검사 통과로 표현하는가 | 전체 `docs:build`의 기존 dead image·Shiki 오류와 선택 빌드 범위를 나눠 기록했다. 통과. |
| 7 | Rspress 버전이 pinned 버전에서 바뀌는가 | Rspress `2.0.22`를 사용했다. 통과. |
| 8 | 출력 경고가 Tailnet 목적지 전체를 비우는가 | 고유한 `/private/tmp` build output을 쓰고 목적지 파일 삭제는 0건이다. 통과. |
| 9 | 새 Tailnet README route가 build에 빠지는가 | `internal/evidence/...tailnet-sync.../README.html`이 22개 route에 포함됐다. 통과. |
| 10 | STATUS C04.12가 완료로 잘못 바뀌는가 | Agent Browser에서 상태 대장 체크박스 `false`를 확인했다. 통과. |
| 11 | 공개 roadmap에서 예전 고정 counter가 되살아나는가 | 화면의 공식 집계는 `205개 표시 · 공식 완료 71/205`였다. 통과. |
| 12 | viewport/scheme 미완료가 지원 완료로 표현되는가 | 로드맵은 page scale, OS preference, CSS 효과와 HTML paint를 미완료로 표시한다. 통과. |
| 13 | build asset URL base가 Tailnet 경로와 어긋나는가 | build config base와 preview 경로가 모두 `/spinon/`이다. 통과. |
| 14 | HTML만 복사하고 static JS/CSS를 빠뜨리는가 | 최종 Rspress output 56개 모두 manifest 대상에 포함했다. 통과. |
| 15 | 수정된 README route가 이전 static chunk를 잘못 참조하는가 | 최종 output-v7 page와 static chunk를 재복사하고 88개 경로의 hash를 대조했다. 통과. |
| 16 | 기존 chunk를 제거해 다른 route를 깨뜨리는가 | Tailnet 파일 삭제는 0건이고 종전 경로를 보존했다. 통과. |
| 17 | source와 destination 경로·hash를 혼동하는가 | manifest가 각각 경로, byte 수, SHA-256, 이전 상태를 기록한다. 통과. |
| 18 | 기존 대상 백업 전에 교체하는가 | 첫 단계 26개와 재렌더 후 24개 교체 파일을 별도 백업하고 hash를 확인했다. 통과. |
| 19 | 새 대상과 기존 교체 경로를 같은 상태로 기록하는가 | preflight는 새 경로 복사·기존 경로 교체·동일 경로를 구분한다. 통과. |
| 20 | backup 경로가 공개 preview root 안에 놓이는가 | 백업은 별도 `SpinonPreviewBackups/c04-12-root-html-default-app-tailnet-sync-2026-10-17` 아래다. 통과. |
| 21 | 부분 복사 오류가 성공으로 기록되는가 | 로컬 파일 88개 모두 기대 byte 수·SHA-256과 일치했다. 통과. |
| 22 | HTTP 200이어도 내용이 오래된 경우를 놓치는가 | local/Tailnet 88개 요청 모두 HTTP 200과 source hash가 일치했다. 통과. |
| 23 | 캐시된 이전 응답을 검증하지 않는가 | `Cache-Control: no-cache` 요청 body를 직접 해시했고 응답이 일치했다. 통과. |
| 24 | app screenshot을 HTML paint로 잘못 표현하는가 | 실행 README는 Android/iOS Simulator 캡처가 빈 native host임을 표시한다. 통과. |
| 25 | terminal-browser로 웹 화면을 확인하는가 | Tailnet UI는 Agent Browser 0.31.1에서 확인했다. 통과. |
| 26 | 캡처가 병합 전 branch preview를 가리키는가 | 병합 후 Tailnet `/roadmap.html`·`/STATUS.html`에서 캡처했다. 통과. |
| 27 | 캡처 후 Agent Browser 프로세스가 남는가 | close 뒤 session `active=false`, `pid=null`을 확인했다. 통과. |
| 28 | 기존 Tailnet listener나 tmux를 바꾸는가 | listener PID 1191은 계속 127.0.0.1:8768에 있었고 tmux는 건드리지 않았다. 통과. |
| 29 | Simulator 기록을 실기기로 과장하는가 | 근거는 Android API 37 AVD와 iOS 26.2 Simulator뿐이며 실기기는 사용하지 않았다. 통과. |
| 30 | 재검토에 필요한 기록이 누락되는가 | build log, manifest, backup/copy/served verification, browser screenshot과 SHA256SUMS를 추가했다. 통과. |

이 PR은 이미 병합된 feature의 Tailnet 문서 반영 및 재현 근거만 추가한다. C04.12 체크·완료 수·제품 지원 상태는 바꾸지 않는다.
