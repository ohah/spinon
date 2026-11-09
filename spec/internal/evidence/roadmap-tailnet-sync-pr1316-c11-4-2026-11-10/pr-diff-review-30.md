# PR #1317 Tailnet 동기화 evidence diff · 적대적 검토 30개

대상은 이 PR의 최종 변경 diff다. 20개 게시 절차 검토는 [별도 기록](./tailnet-operation-review-20.md)에 남겼다. 각 항목은 상태·계획·roadmap 또는 Tailnet evidence 경계 하나를 점검한다.

| # | 실패 관점 | 확인과 결과 | 판정 |
|---:|---|---|---|
| 1 | 잘못된 PR merge metadata | PR #1316의 merged 상태, `83c1c57e9c448202040f8efe4e12ff8f3d3b8d8c` SHA와 GitHub UTC 시각이 STATUS·계획·evidence에서 일치한다. | 통과 |
| 2 | 다른 source commit의 화면 게시 | 선택 source commit `91179a34089c1fa2f3f964a83c4c4b28d424d9f1`과 tree `684837d3d678de664920e47b080509e64d9fd406`이 manifests에 연결된다. | 통과 |
| 3 | 계획 PR을 구현 PR로 표시 | #1316은 계획 PR로 명시되고 #1314는 마지막 병합 구현으로 유지된다. | 통과 |
| 4 | C11.4 완료로 오인 | STATUS·roadmap·Tailnet 화면 모두 C11.4/C11 미완료를 유지한다. | 통과 |
| 5 | 공식 완료 수 변경 | manifest와 화면에 `78/214`를 유지한다. | 통과 |
| 6 | 내부 계약 버전 과장 | 내부 계약 숫자 버전 `0.1.0`을 변경하지 않는다. | 통과 |
| 7 | 다음 카드의 링크 target 오류 | roadmap href, publication route와 Agent Browser에서 열린 plan URL이 정확히 일치한다. | 통과 |
| 8 | 계획 merge metadata가 표시되지 않음 | kanban heading에 `계획 PR #1316 Rebase and merge`가 나타난다. | 통과 |
| 9 | 이전 구현 카드가 사라짐 | PR #1314 implementation 카드와 세 기존 local links를 보존한다. | 통과 |
| 10 | 계획 본문을 게시했지만 검토 파일 누락 | plan, 계획 review-20, PR diff review-30 파일이 selected publication에 포함된다. | 통과 |
| 11 | Chrome evidence page 누락 | auto-minimum precomparison README route와 연결된 PNG asset을 포함한다. | 통과 |
| 12 | desktop screenshot을 mobile 증거로 착각 | desktop artifact는 1440×1000 PNG이고 관측 viewport와 일치한다. | 통과 |
| 13 | mobile screenshot 크기 불일치 | mobile artifact는 390×844 PNG이고 관측 viewport와 일치한다. | 통과 |
| 14 | 다른 서버에서 찍은 화면 | lifecycle·verification은 지정한 `macstudio.tailed42f2.ts.net` URL의 Agent Browser 세션을 기록한다. | 통과 |
| 15 | 캡처 당시 새 plan link 미반영 | screenshot, JSON 관측과 HTML source 모두 같은 `/spinon/plan/...auto-minimum.html` href다. | 통과 |
| 16 | 모바일 body의 가로 잘림 | 390px viewport에서 document와 body가 각각 390px이며 Kanban은 자체 362px 폭 안에서 스크롤된다. | 통과 |
| 17 | 화면상 공식 counter와 source 차이 | Agent Browser note, source STATUS와 JSON이 모두 78/214를 보고한다. | 통과 |
| 18 | server가 다른 PID로 바뀌거나 내려감 | preflight·verification에서 PID `90711`, port `8768`, server restart false를 기록했다. | 통과 |
| 19 | 일부 route의 오래된 content | local과 HTTPS의 117개 SHA가 모두 일치한다. | 통과 |
| 20 | 교체 파일 backup 누락 | 25개 교체 대상을 백업했고 사후 hash 검증도 25/25다. | 통과 |
| 21 | 안전 검사 없는 덮어쓰기 | 교체 전 open handle count 0이며 manifest/preflight/copy 단계의 순서를 보존했다. | 통과 |
| 22 | 기존 시작 페이지 또는 404 제거 | 두 route를 제외했으며 `noDeletes=true`다. | 통과 |
| 23 | 전부 새로 복사했다고 잘못 집계 | `47 create + 25 replace + 45 same = 117`을 보고한다. | 통과 |
| 24 | 전체 docs link check를 통과했다고 주장 | 선택 RSPress build와 전체 `docs:build`를 구분하고 1,518 dead-link source 실패를 명시한다. | 통과 |
| 25 | version drift | 선택 빌드는 RSPress `2.0.22`를 사용한다. | 통과 |
| 26 | build artifact를 보존해 불필요한 용량 누적 | 임시 selected build `35 MB`를 사용 완료 후 제거하고 manifests·로그·screenshot을 evidence에 남겼다. | 통과 |
| 27 | preview rollback 경로 불명 | 25개 backup의 사용자 지원 경로를 README와 manifests에 명시한다. | 통과 |
| 28 | task browser가 남아 리소스를 점유 | 전용 Agent Browser 세션을 닫고 `active:false`, `pid:null`을 확인했다. | 통과 |
| 29 | Terminal Browser 또는 simulator 결과 혼입 | 해당 도구와 simulator를 사용하지 않았고 lifecycle JSON에 false로 기록한다. | 통과 |
| 30 | 기록 파일 자체의 누락·중복 | 검토표 행 수 30, 작업 검토표 행 수 20, 모든 증거 파일의 SHA256 목록을 독립 확인한다. | 통과 |

**결과:** PR diff에 대한 서로 다른 실패 관점 30개를 확인했다. Tailnet 화면은 공개 preview의 상태 표현 검증이며 Spinon Android/iOS 앱 실행 또는 C11.4 구현 완료 증거가 아니다.
