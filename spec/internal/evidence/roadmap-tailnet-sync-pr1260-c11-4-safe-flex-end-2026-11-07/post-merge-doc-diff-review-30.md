# C11.4 flex-end 병합 후 상태·로드맵 문서 diff 검토 30개

병합된 PR #1260의 공식 상태, 작업 계획, 내부 계약, 공개 roadmap, Tailnet 제공본과 Agent Browser 증거를 서로 대조했다. 구현/runtime 검토 및 PR #1260 코드 diff 검토와 별도의 문서 동기화 변경 검토다.

| # | 독립 검토 관점 | 판정 |
|---:|---|---|
| 1 | PR 상태가 open이나 draft로 남아 있는가 | GitHub PR #1260은 closed/merged이며 `merged_at`은 `2026-11-07T08:20:03Z`다. 통과. |
| 2 | merge SHA가 상태 대장·계획·계약·roadmap에서 같은가 | 네 문서는 `57f5b9103e919a8dc7f9378dc78f984b3101f1f1`을 기록한다. 통과. |
| 3 | 병합 날짜와 시간대가 혼동되는가 | ISO 표기는 UTC `Z`를 유지하며 Tailnet status note의 로컬 시각은 별도로 확인했다. 통과. |
| 4 | 체크가 없는데 CI 성공이라고 주장하는가 | PR head combined status와 workflow run 목록이 모두 비어 있다. 어느 문서도 CI green을 주장하지 않는다. 통과. |
| 5 | 병합한 safe flex-end가 아직 진행 중으로 표시되는가 | 공개 history에는 PR #1260 병합으로 기록하고 칸반 active 카드는 후속 계획 단계로 옮겼다. 통과. |
| 6 | 후속 계획 카드가 새 기능 구현 완료/진행으로 과장되는가 | “후속 slice 계획 중”으로 표시하고 새 구현이 이미 시작된 뜻이 아님을 evidence README에 명시했다. 통과. |
| 7 | 상위 작업 총계가 바뀌었는가 | 상위 작업 78개 표시는 그대로다. 통과. |
| 8 | 하위 ID 총계가 잘못 합산됐는가 | 134 세부 ID와 ID 없는 하위 체크 2개 표시는 동기화 note와 일치한다. 통과. |
| 9 | 공식 완료 수가 이 bounded slice 때문에 증가했는가 | 완료 수 78/214를 유지한다. 통과. |
| 10 | C11.4를 완료 체크로 잘못 승격했는가 | `spec/STATUS.md`의 C11.4 checkbox는 미완료 상태로 남는다. 통과. |
| 11 | 상위 C11을 완료 처리했는가 | C11 checkbox 및 roadmap의 미완료 문구를 유지한다. 통과. |
| 12 | safe/unsafe flex-end가 일반 CSS 지원으로 표현되는가 | 이 문서들은 기존 fixed profile의 제한 구현으로만 연결하고 public CSS 지원을 선언하지 않는다. 통과. |
| 13 | 선행 safe flex-start merge 정보가 변경되거나 덮였는가 | PR #1256과 merge SHA `0c3fa3f94e9a4af1db2f3e22be012df2282e6a67`를 보존하고 #1260을 후속으로 기록했다. 통과. |
| 14 | PR 본문 스크린샷 개수가 과장됐는가 | PR #1260 body에 positive·safe overflow·unsafe overflow의 Android/iOS 화면 총 6장이 있고 Agent Browser에서 모두 로드됐다. 통과. |
| 15 | 시뮬레이터 스크린샷을 surface pixel equality로 오해하게 쓰는가 | 시각 bounds만 증거로 쓰며 PixelCopy, Metal drawable readback과 compositor 이후 일치를 명시적으로 제외했다. 통과. |
| 16 | Chrome 관측값과 독립 기대값이 섞이거나 횟수가 바뀌었는가 | 6 scene·72 coordinates·0 CSS px 차이라는 source report의 수치를 유지한다. 통과. |
| 17 | focused tests를 전체 workspace green으로 확대했는가 | Layout/style projection/RuntimeSession focused 2/2 및 회귀 filter 9/9·6/6·9/9로 범위를 한정했다. 통과. |
| 18 | Android/iOS 런타임 장면 수가 플랫폼별인지 총합인지 모호한가 | 양 플랫폼에서 positive·safe overflow·unsafe overflow 각 세 장면을 실행했다고 썼다. 통과. |
| 19 | 실행 뒤 기존 앱·asset 복원을 누락했는가 | 상태 문서와 runtime README link가 실행 및 restore evidence로 연결된다. 통과. |
| 20 | 작업 계획이 아직 구현 중으로 표시되는가 | 계획 상단 상태를 실제 PR #1260 merge 결과와 limitations로 갱신했다. 통과. |
| 21 | 내부 계약 숫자 버전이 임의로 증가했는가 | 계약 `0.1.0`과 기존 fixed-profile 경계를 유지한다. 통과. |
| 22 | Tailnet에서 생성되지 않은 상대 문서 페이지를 새 링크로 넣었는가 | 새 history entry의 계획·runtime·review 링크는 Tailnet에 없는 local `.html` 대신 GitHub main의 Markdown 경로를 사용한다. 통과. |
| 23 | 새 PR·merge 정보 링크가 올바른 저장소/PR을 가리키는가 | PR link는 `ohah/spinon/pull/1260`이며 Agent Browser DOM에서도 같은 href를 확인했다. 통과. |
| 24 | source hash와 Tailnet served hash가 다른가 | 최종 622,905 bytes source·target·HTTP response 모두 SHA-256 `980c92d9360b39a5ccc319aede900108c68ceb1fcc519c4936fae528b78d2494`다. 통과. |
| 25 | 교체 전 제공본을 백업하지 않았거나 다른 파일을 덮었는가 | pass03은 이전 제공본 hash `a291392b…`, pass04는 중간본 `28ad80d6…`를 각기 보존한다. 통과. |
| 26 | 백업 경로가 기존 pass와 충돌하는가 | 새 pass03/pass04 폴더를 별도로 만들고 두 backup hash를 copy record와 대조했다. 통과. |
| 27 | Tailnet 서버를 중단하거나 재시작했는가 | preflight/copy record 모두 PID 28040과 `serverRestarted:false`를 기록한다. 통과. |
| 28 | localhost 또는 Tailnet 경로가 오류 응답/다른 본문을 제공하는가 | `127.0.0.1:8768`과 Tailnet HTTPS 모두 200 및 동일한 최종 hash를 반환했다. 통과. |
| 29 | 데스크톱 화면에서 card가 보이거나 문서 폭이 깨지는가 | Agent Browser 1440×1000에서 C11.4 active planning card를 보았고 document/body scroll width가 1440이다. 통과. |
| 30 | 모바일 폭, 최종 스크린샷 또는 Agent Browser 종료를 누락했는가 | 390×844에서 document/body width가 390이고 active card를 확인했다. final PNG hash/dimensions를 남겼으며 종료 뒤 일치하는 browser process가 없다. 통과. |

30개 항목에서 문서 동기화 blocker를 찾지 못했다. Tailnet source·served hashes, 기존 제공본 백업, desktop/mobile DOM·screenshots 및 browser lifecycle은 같은 evidence directory에 보존했다. GitHub Pages 배포는 하지 않았다.
