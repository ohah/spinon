# C19.2 Tailnet 미리보기 동기화 PR 변경 검토 30개 관점 · 2026-10-17

검토 범위는 C19.2 문서의 격리 build, Tailnet 정적 파일 복사, hash manifest, 실제 Agent Browser 화면과 동기화 근거다. 제품 코드·공식 완료 상태·Tailscale Serve 설정은 변경하지 않는다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 동기화 기준 commit이 실제 main과 다른가 | 기준 commit `9b8d74f2`와 tree `21915e49`를 기록하고 현재 branch tree와 대조한다. 통과. |
| 2 | build source 일부가 다른 revision에서 섞였는가 | 임시 source 입력 104개를 기준 tree 파일과 byte 단위로 대조해 모두 일치한다. 통과. |
| 3 | 격리 build가 Rspress 의존성을 바꾸는가 | 고정 버전 `2.0.22`를 사용하고 저장소 package/lockfile은 수정하지 않는다. 통과. |
| 4 | scoped build 성공을 전체 문서 build로 오해하는가 | 범위와 전체 build 미실행을 로그·README에 분리해 적었다. 통과. |
| 5 | dead-link 검사를 끈 상태를 전체 경로 적합성으로 주장하는가 | 임시 config에서만 비활성화했고 이 제한을 공개했다. 통과. |
| 6 | 이미지 경로가 경고 비활성화로만 통과하는가 | 여섯 PNG가 hashed `static/image` 산출물로 생성되고 실제 URL에서 로드된다. 통과. |
| 7 | 빌드 산출물에 C19 문서가 빠지는가 | road map, STATUS, 0069 계약 및 C19.2 근거 route를 실제 브라우저에서 연다. 통과. |
| 8 | Tailnet 서버가 repository를 직접 서빙한다고 오해하는가 | preview root와 복사 방식, 기존 `127.0.0.1:8768` route를 기록한다. 통과. |
| 9 | base path가 빠져 상대 이미지 URL이 깨지는가 | 모든 확인 URL은 `/spinon/` 경로 아래서 제공된다. 통과. |
| 10 | 잘못된 STATUS alias를 근거 링크로 안내하는가 | canonical `/spinon/spec/STATUS.html`을 사용한다. 통과. |
| 11 | roadmap URL은 열려도 최신 내용이 아닌가 | 실제 Tailnet roadmap 화면에서 PR #232 요약과 C19.2 한계를 확인한다. 통과. |
| 12 | 동적 count가 표시되지 않거나 stale HTML인가 | 로드맵은 `203개 표시 · 공식 완료 71/203`을 노출하고 108개 served bytes도 기준 manifest와 일치한다. 통과. |
| 13 | 71을 61 또는 62로 잘못 기록하는가 | 이전 count를 stale snapshot으로 설명하고 현재 동적 count 71/203을 기록한다. 통과. |
| 14 | 203개 전체와 상위 집계 수가 혼동되는가 | UI의 task count와 공식 완료 count를 함께 적는다. 통과. |
| 15 | C19 parent가 실수로 완료됐는가 | 실제 DOM에서 C19 체크박스는 disabled 및 unchecked다. 통과. |
| 16 | C19.2가 미완료인데 완료로 표시되는가 | 실제 DOM과 STATUS 페이지 모두 C19.2 미완료를 보인다. 통과. |
| 17 | PR 설명이 미완료 범위를 숨기는가 | hardware GPU, iOS compositor 후, image opacity group, text/contrast, hit-test를 남은 범위로 유지한다. 통과. |
| 18 | `STATUS.md`를 고치지 않고 status가 달라졌다고 쓰는가 | 이 PR은 STATUS source를 수정하지 않으며 화면 count는 현재 source에서 동적으로 온다. 통과. |
| 19 | 동기화가 파일을 조용히 삭제하는가 | manifest와 복사 검증에서 삭제 수는 0이다. 통과. |
| 20 | 교체 파일의 기존 버전을 복구할 수 없는가 | 14개 교체 대상 각각의 backup이 존재하고 hash 검증됐다. 통과. |
| 21 | created/replaced/unchanged 합계가 전체 경로와 다른가 | `81 + 14 + 13 = 108`이며 manifest count와 일치한다. 통과. |
| 22 | 바이트 합계가 manifest 파일 수와 맞지 않는가 | 108 response body 합계는 5,615,740 bytes다. 통과. |
| 23 | manifest 자체 hash를 self-reference 때문에 검증할 수 없는가 | manifest에는 self hash를 제외하고 외부 계산 SHA-256과 크기를 기록한다. 통과. |
| 24 | 응답 HTTP 200만 보고 오래된 파일을 새 파일로 오판하는가 | 각 응답의 byte length와 SHA-256을 manifest 값과 비교해 108개 모두 일치한다. 통과. |
| 25 | preview가 다른 source revision을 제공하는가 | manifest의 main commit/tree와 HTTP body 해시를 함께 확인한다. 통과. |
| 26 | Android 이미지가 iOS 캡처로 혼동되는가 | 여섯 이미지의 route, 파일명, 실제 크기와 기기 라벨이 각각 일치한다. 통과. |
| 27 | PR 화면 증거가 실제 Tailnet이 아닌 로컬 후보 화면인가 | 캡처를 실제 `macstudio.tailed42f2.ts.net`에서 Agent Browser로 저장했다. 통과. |
| 28 | screenshot이 있는데 페이지 내부 이미지가 깨졌는가 | runtime evidence 페이지의 여섯 screenshot이 모두 `complete`이고 natural size가 양수다. 통과. |
| 29 | 복사 검증 과정에서 공용 preview를 재시작하거나 Serve 설정을 바꾸는가 | 기존 Node PID 1191 및 Serve route를 확인했고 재시작·설정 변경은 없었다. 통과. |
| 30 | 작업 후 Agent Browser 프로세스가 남아 사용자 요구를 어기는가 | PR 화면 확인을 끝낸 뒤 이 세션만 닫고 browser process 종료를 확인한다. 완료 여부는 merge 뒤 README에 기록한다. |

복사 후 검증 summary는 108개 hash 일치, 14개 backup hash 일치, mismatch 0, deletion 0이다. 전체 문서 build·Pages 배포·기기 검증은 이번 PR 판정 범위에 넣지 않는다.
