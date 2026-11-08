# C11.4 병합 뒤 로드맵·Tailnet 동기화 · PR diff 적대 검토 30개

이 검토는 PR #1280 병합 후의 문서 및 선택 경로 게시 diff를 대상으로 한다. 구현 PR #1280의 별도 30개 검토를 재사용하지 않았다. `PASS`는 이 문서 변경의 상태·게시 범위 검토 결과이며 전체 CSS Grid 완료나 CI green을 뜻하지 않는다.

| # | 실패 관점 | 최종 diff에서 확인한 방어·근거 | 결과 |
|---:|---|---|---|
| 1 | C11.4 slice 병합을 상위 C11.4 완료로 승격한다 | `spec/STATUS.md`와 계획은 병합 정보를 더했지만 C11.4 체크는 미완료로 둔다. | PASS |
| 2 | C11 하위 체크를 모두 끝난 것으로 표시한다 | 공식 상태 대장의 C11 체크는 변경하지 않았고 문서는 C11 미완료를 다시 적는다. | PASS |
| 3 | 별도 상위 ID 없는 slice를 공식 완료 분모에 넣는다 | 실행 수치와 진행 세부 작업을 `78/214` 공식 완료 집계와 분리한다. | PASS |
| 4 | 완료 수가 그대로인 이유를 숨긴다 | 공개 설명은 C11.4 세부 진행과 공식 완료 수가 다른 지표임을 말한다. | PASS |
| 5 | 다른 PR 또는 잘못된 PR을 진행 카드에 연결한다 | 공개 카드와 상태 대장은 구현 PR #1280 URL을 사용한다. | PASS |
| 6 | 열린 PR 상태를 병합 상태로 기록한다 | GitHub 재조회는 PR #1280 `closed`, `merged=true`를 반환했고 문서가 Rebase and merge를 기록한다. | PASS |
| 7 | 잘못된 merge SHA를 게시한다 | PR 정보의 `b1b27b02491f1614ded260fd3b66f50184c5bb80`와 상태·계획·로드맵 본문이 일치한다. | PASS |
| 8 | UTC 병합 시각을 임의로 기록한다 | GitHub `merged_at` `2026-11-08T04:28:20Z`가 세 문서의 시간과 일치한다. | PASS |
| 9 | 계획 문서가 PR 생성 전 상태에 머문다 | 계획 상태 문구의 “PR 생성과 merge 상태 확인은 남아 있다”를 실제 병합 기록으로 교체했다. | PASS |
| 10 | 공개 진행 카드가 PR 생성 중으로 남는다 | 새 Tailnet 화면의 첫 C11.4 문단은 `#1280 Rebase and merge`와 SHA를 표시한다. | PASS |
| 11 | 현재 진행 카드를 완료 칸으로 옮긴다 | 칸반은 `진행 중 1/2`와 `C11.4 · 진행 중`을 유지한다. | PASS |
| 12 | 진행 중인 C11.4가 카드 제목·보드에서 식별되지 않는다 | 칸반 제목 옆 `진행 중: C11.4 Grid implicit 행 목록`과 `Grid 24순서 구현` 카드를 Agent Browser로 확인했다. | PASS |
| 13 | 과거의 Layout `11/11`을 현재 candidate 수치로 남긴다 | 현재 진행 문단은 Layout `12/12`를 표시한다. 이전 역사적 PR 문단은 별도 기록으로 남는다. | PASS |
| 14 | typed projection 조합 수가 불명확하다 | 공개 카드와 실행 근거가 24순서 × 네 cap form `96` typed cases를 기록한다. | PASS |
| 15 | RuntimeSession 순서·alignment matrix가 축약된다 | 현재 카드가 24순서 × px/% × start/stretch `96/96`을 기록한다. | PASS |
| 16 | fit-content cycle overflow를 일반 geometry로 뭉갠다 | 현재 카드가 네 순환 위치의 px/% cap overflow `48/48`을 별도로 기록한다. | PASS |
| 17 | 경계·거부·복구를 지원으로 과장한다 | `0%`·`120%`, calc, 거부, 복구를 검증 범위로 나열하고 임의 문법 parity 주장은 하지 않는다. | PASS |
| 18 | Android simulator screenshot을 PixelCopy로 주장한다 | 공개 카드 및 PR 근거가 OS screenshot visual smoke와 PixelCopy 부재를 구분한다. | PASS |
| 19 | iOS simulator screenshot을 Metal readback으로 주장한다 | iOS 26.2 Simulator OS screenshot과 Metal surface readback 부재를 구분한다. | PASS |
| 20 | 시뮬레이터 근거를 실기기 또는 hardware GPU로 일반화한다 | 공개 카드와 실행 README는 AVD/Simulator만 기록하고 실기기·hardware GPU를 주장하지 않는다. | PASS |
| 21 | 빈 GitHub rollup을 CI 통과라고 기록한다 | PR workflow/status 조회가 빈 목록이며 상태·계획·로드맵이 CI green을 부정한다. | PASS |
| 22 | 내부 Grid contract 버전을 올린다 | contract `0.1.0`을 보존하고 기존 계약 숫자를 바꾸지 않았다. | PASS |
| 23 | 카드 링크가 현재 계획 문서가 아닌 과거 plan을 가리킨다 | 10개 HTTP 검증 대상에서 현재 four-member plan route가 200이고 게시 SHA와 일치한다. | PASS |
| 24 | 카드 링크가 낡은 실행 근거를 가리킨다 | current precomparison/runtime README·review route 5개를 다시 빌드해 게시하고 각 route의 SHA를 대조했다. | PASS |
| 25 | HTML 원본을 RSPress 결과로 오인하거나 반대로 제공한다 | staging은 선택 문서 원본과 RSPress generated routes를 구별하고 `roadmap.html` source SHA를 따로 기록한다. | PASS |
| 26 | RSPress dependency를 예고 없이 올린다 | 고정된 `@rspress/core` `2.0.22`로 선택 root를 빌드했다. | PASS |
| 27 | preview 경로 전체를 지우거나 덮어쓰기 실패에서 기존 파일을 잃는다 | 276개 route만 추가/동일 확인/교체했고 20개 교체 파일은 별도 backup 후 같은 디렉터리 원자 rename했다. `noDeletes=true`다. | PASS |
| 28 | 열려 있는 preview 파일을 교체하거나 서버를 재시작한다 | preflight에서 교체 대상 열린 reader는 0개였다. PID 28040, cwd, Tailscale `Running`을 확인했고 재시작하지 않았다. | PASS |
| 29 | 앱 데이터 snapshot을 Tailnet으로 함께 게시한다 | 게시 manifest의 `.tar.gz` route는 0개이며 `bootstrap-original`도 selected root에서 제외했다. | PASS |
| 30 | 모바일에서 진행 카드를 못 보거나 문서가 가로로 넘친다 | Agent Browser 390×844에서 active card를 보이게 scroller를 이동했고 문서 폭 390px·진행 카드 visible을 확인했다. PR #1281 본문의 데스크톱/모바일 이미지도 각각 1440×1000, 390×844로 로드됐다. 두 확인 session 종료 상태는 lifecycle JSON에 남겼다. | PASS |

## 판정

30개 관점 모두 문서 diff와 게시 근거에서 방어를 확인했다. Tailnet의 10개 선택 route는 HTTP 200 및 게시 SHA와 일치했다. 전체 사이트 dead-link 검사는 하지 않았고, 이 변경은 C11.4/C11 구현 완료를 뜻하지 않는다.
