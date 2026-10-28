# PR #741 Tailnet 단일 경로 동기화 변경 검토 30관점

대상은 Tailnet 정적 미리보기의 `roadmap.html` 한 파일과 이 동기화의 재현 근거다. 기능 구현이나 C11.4 완료 판정은 변경하지 않는다.

| # | 실패 가설 / 독립 관점 | 결과 |
|---:|---|---|
| 1 | main 소스가 PR #741 병합 후의 커밋이 아니다. | 통과 — source commit `974296be9744681b42ea9c79a512e183f2d836a2` 기록 |
| 2 | source 바이트가 main 파일과 다르다. | 통과 — source SHA `22d32405f27f7b67b51745e26c8d3a7d915b901a0556e3dac6c32a9e01217cba` 기록 |
| 3 | 사전 target이 확인 없이 덮어써진다. | 통과 — 알려진 이전 SHA와 exact match 뒤 진행 |
| 4 | target 경로가 예상 밖의 symlink나 비정규 파일이다. | 통과 — regular file이며 symlink가 아님을 확인 |
| 5 | 대상 파일을 사용하는 열린 프로세스가 있다. | 통과 — 교체 직전 `lsof`에서 target handle 없음 |
| 6 | 백업 경로가 기존 데이터를 덮어쓴다. | 통과 — 고유 pass 경로가 없음을 확인하고 생성 |
| 7 | 백업이 대상의 바이트를 그대로 보존하지 않는다. | 통과 — 백업 바이트와 이전 SHA가 원본과 일치 |
| 8 | 동일 파일 시스템 밖의 복사로 교체가 부분 노출된다. | 통과 — 같은 디렉터리의 임시 파일에서 atomic replace |
| 9 | 원본 파일 권한이 바뀐다. | 통과 — target mode를 임시 파일에 보존 |
| 10 | sync가 roadmap 이외 파일까지 바꾼다. | 통과 — target 하나만 교체 |
| 11 | local route가 새 파일을 serving하지 않는다. | 통과 — 교체 후 HTTP 200, bytes와 SHA가 source와 일치 |
| 12 | Tailnet route가 새 파일을 serving하지 않는다. | 통과 — 교체 후 HTTP 200, bytes와 SHA가 source와 일치 |
| 13 | 두 경로 중 하나에서 캐시된 예전 바이트가 반환된다. | 통과 — 두 응답 모두 새 SHA, `Cache-Control: no-cache` |
| 14 | 경로 응답 크기가 source와 다르다. | 통과 — 둘 다 364,401 bytes |
| 15 | Tailscale이 offline인데 과거 연결만 보고 성공 처리한다. | 통과 — backend `Running`, self `Online` |
| 16 | 기존 main preview server를 재시작해 이용자 세션을 끊는다. | 통과 — PID 28040, port 8768 그대로 유지 |
| 17 | 별도 port 4174 listener를 종료하거나 덮어쓴다. | 통과 — PID 33319를 그대로 두고 변경하지 않음 |
| 18 | Tailnet host/경로를 잘못 확인한다. | 통과 — 요청된 `macstudio.tailed42f2.ts.net/spinon/roadmap.html` 응답 확인 |
| 19 | 브라우저가 오래된 캐시를 화면으로 보여준다. | 통과 — 새 query key와 no-cache route로 Agent Browser 확인 |
| 20 | Tailnet 화면에서 PR #738 카드가 누락된다. | 통과 — Agent Browser 화면과 DOM에서 카드 확인 |
| 21 | 카드의 공식 완료 수가 부정확해진다. | 통과 — `72/213`, C11.4 미완료 표기 유지 |
| 22 | 카드의 구현 결과를 CI 통과로 과장한다. | 통과 — check rollup이 비었다는 원문 유지 |
| 23 | 카드 링크 중 옛 Tailnet 상대 경로가 남는다. | 통과 — 같은 origin 링크 0개 |
| 24 | 문서 링크가 GitHub 이외 origin으로 잘못 연결된다. | 통과 — 11개 destination 모두 GitHub 또는 raw GitHub origin |
| 25 | PNG가 mutable `main`의 최신 이미지로 바뀔 수 있다. | 통과 — 이미지 링크는 구현 merge SHA로 고정 |
| 26 | Android 화면 링크가 잘못된 플랫폼 산출물이다. | 통과 — 브라우저 natural size 1080×2400 |
| 27 | iOS 화면 링크가 잘못된 플랫폼 산출물이다. | 통과 — 브라우저 natural size 1206×2622 |
| 28 | 화면 기록에서 카드가 viewport 밖에 있다. | 통과 — 스크린샷 1280×633에서 #738 카드가 보임 |
| 29 | 브라우저를 종료하지 않아 사용자가 요청한 프로세스가 남는다. | 통과 — 닫은 뒤 활성 session 0, browser process 없음 |
| 30 | 동기화 결과를 runtime 검증이나 공식 완료로 오해한다. | 통과 — 근거 문서가 범위를 정적 링크 동기화로 제한 |

이 검토는 문서 배포와 화면 표시만 다룬다. Android/iOS runtime 테스트를 다시 수행한 것으로 간주하지 않는다.
