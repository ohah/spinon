# C11.1 intrinsic Grid gap 계획 PR 변경분 적대 검토 20개

대상 diff: 고정 px gap 계획, 계획 전용 20항목 리뷰, Agent Browser fixture·관찰·expected·화면과 hash manifest. 제품 구현 코드는 포함하지 않는다.

| # | PR diff에서 공격한 경계 | 판정 |
|---:|---|---|
| 1 | gap이 intrinsic base 크기에 더해지는 잘못된 oracle | **PASS** — expected는 fixed base 40/10px와 gutter를 분리한다. |
| 2 | `fr` 계산에서 gap이 남은 free space에 포함되지 않음 | **PASS** — 축별 `[40,50]`/`[10,80]`을 expected와 Chrome JSON에 고정했다. |
| 3 | row와 column gap 축이 뒤바뀜 | **PASS** — 두 축 단독 장면이 서로 다른 track/result를 만든다. |
| 4 | shorthand와 longhand 결과 불일치 | **PASS** — 양축 shorthand 장면과 단독 longhand 장면을 분리했다. |
| 5 | no-gap 기준값이 새 fixed-gap 기대와 섞임 | **PASS** — `control` scene의 `[40,60]`/`[10,90]`을 둔다. |
| 6 | CSSOM `normal`을 computed `0px`라고 기록 | **PASS** — raw observation은 `normal`, independent expected는 used zero로 분리했다. |
| 7 | browser 절대좌표가 independent expected에 복사됨 | **PASS** — raw observation은 absolute, expected는 root-relative 좌표다. |
| 8 | viewport나 DPR가 달라 관찰 재현이 흔들림 | **PASS** — HeadlessChrome UA, viewport, DPR를 JSON에 저장했다. |
| 9 | CSS-valid percentage control이 브라우저에서 invalid로 오인 | **PASS** — Chrome의 `10%` computed 값을 별도 excluded valid control에 보존했다. |
| 10 | 계획이 percentage gap도 지원하는 것처럼 확장 | **PASS** — fixed px only scope와 `%` rejection을 여러 경계에 일치시켰다. |
| 11 | zero gap computed style가 생략되어 기존 behavior 손실 | **PASS** — control은 declaration 없는 CSS `normal`과 used 0을 고정한다. |
| 12 | fixed contribution과 text/font 차이를 혼동 | **PASS** — fixture에는 텍스트 item이 없고 fixed box contribution만 있다. |
| 13 | row min-content가 상수 높이 fixed box로 잘못 산출 | **PASS** — 10px first row는 fixed-height 10px item에서 독립 도출된다. |
| 14 | percentage basis가 border-box로 오해 | **PASS** — 0 edges와 definite 100px content axis로 입력을 제한했다. |
| 15 | scene 간 상속/DOM/placement 변화가 gap 결과 교란 | **PASS** — 다섯 scene은 root offset과 gap 외 CSS/child structure를 공유한다. |
| 16 | fixture snapshot이 화면과 다른 파일 revision | **PASS** — fixture·JSON·PNG hash를 manifest로 묶었고 검증했다. |
| 17 | 화면이 실제 Agent Browser capture가 아니라 손으로 만든 그림 | **PASS** — 사용자 지정 Agent Browser에서 캡처했고 확인 후 session/process를 닫았다. |
| 18 | evidence가 제품 실행 또는 WPT pass라고 과장 | **PASS** — README는 precomparison only이며 runtime/WPT 아님을 명시한다. |
| 19 | 계획이 상위 완료 상태와 전체 완료 집계를 올림 | **PASS** — C11.1/C11/C11.4 체크와 공식 집계를 유지하도록 명시했다. |
| 20 | plan review를 plan PR diff review로 재사용 | **PASS** — 별도 파일과 20개의 diff-specific 관점을 둔다. |

**판정:** 계획 PR diff의 서로 다른 20개 실패 경로를 검토했고 blocker는 찾지 못했다. Browser capture 외 제품 test/build는 실행하지 않았다.
