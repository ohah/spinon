# PR #481 공개 roadmap 반영 회고적 diff 검토 30개

PR #481의 네 문서 변경을 병합 후 점검했다. PR 생성·병합 당시 별도의 30개 diff 검토 파일을 남기지 못했으므로, 이 회고 검토는 해당 사전 절차를 충족했다고 주장하지 않는다. 저장소 이력을 보완하고 현재 공개 결과의 사실성과 링크를 확인한다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | C11.4가 하위 slice 통과만으로 완료 처리됨 | 상태 대장 상위 checkbox는 계속 미완료다. |
| 2 | 공식 완료 집계가 증가함 | 상태 및 public roadmap 모두 `72/213`을 유지한다. |
| 3 | 잘못된 PR을 merge 근거로 연결 | 링크는 구현 PR #480을 가리킨다. |
| 4 | merge SHA 오타·대상 혼동 | 공개 SHA `e8c7eb06c66fc7424002eaca76af081ecd72a24f`는 GitHub PR #480의 merge commit과 일치한다. |
| 5 | merge timestamp 시간대 혼동 | `22:54:20 UTC`는 GitHub의 mergedAt과 일치한다. |
| 6 | squash/merge commit을 Rebase and merge로 잘못 표기 | PR #480의 실제 merge method는 Rebase and merge다. |
| 7 | 빈 check rollup을 통과한 CI로 표시 | status와 public note는 rollup이 비었다고 쓴다. |
| 8 | main required status check 문구가 과장됨 | GitHub required-status-check endpoint는 `Required status checks not enabled`를 반환했고 적용 branch ruleset도 없었다. |
| 9 | state entry가 구현과 spec boundary를 과장함 | 네 fixed scene과 opt-in runtime fixture 범위로 한정한다. |
| 10 | 상태 기록이 집중 계획으로 연결되지 않음 | `plan/c11-4-cyclic-calc-track-sizing.md` 링크가 존재하고 빌드 route에서도 열렸다. |
| 11 | 상위 계획의 기존 PR 이력이 대체·삭제됨 | 신규 구현 문단만 뒤에 추가했고 기존 항목은 유지됐다. |
| 12 | 계획 문서에 실제 기대값과 결과가 바뀜 | CRD 기반 `[16]/[24]/[16]/[24]`, `[16]/[10]/[10]/[12]` 기대값이 유지됐다. |
| 13 | runtime evidence 링크가 잘못된 날짜 디렉터리로 연결 | README route와 네 scene 하위 증거 경로가 selected build에 포함됐다. |
| 14 | screenshot 링크가 존재하지 않는 Android 파일을 가리킴 | Android two-axis PNG가 source와 preview에 존재하며 `1080×2400`으로 열렸다. |
| 15 | screenshot 링크가 존재하지 않는 iOS 파일을 가리킴 | iOS two-axis PNG가 source와 preview에 존재하며 `1206×2622`로 열렸다. |
| 16 | Android AVD 결과를 Android 실기기 결과로 표기 | API 37 ARM64 AVD라고 명시한다. |
| 17 | iOS Metal drawable을 compositor 이후 화면으로 표기 | compositor 제출 전 sample임을 명시한다. |
| 18 | border paint 또는 root 밖 overflow까지 확인했다고 과장 | evidence가 확인한 track 경계 표본으로 범위를 제한한다. |
| 19 | 일반 CSS admission을 지원한다고 읽힘 | 일반 CSS author admission이 미완료라고 명시한다. |
| 20 | WPT 완료를 암시 | WPT 미검증을 명시한다. |
| 21 | C11.4 checkbox를 하위 scene 결과와 함께 체크 | 상위 checkbox 및 public roadmap 미완료 문구를 재확인했다. |
| 22 | 새 note가 HTML 문단 구조를 깨거나 기존 note 안에 중첩 | 새 `<p class="kanban-note">`는 기존 note 앞에 독립 삽입됐고 Rspress 페이지가 렌더링됐다. |
| 23 | CSS math markup이 HTML에서 Markdown backtick으로 노출 | public roadmap은 expression을 `<code>` 태그로 표시한다. |
| 24 | 계획 URL이 잘못된 문서 extension을 사용 | `/spinon/plan/c11-4-cyclic-calc-track-sizing.html`이 실제 preview route다. |
| 25 | 내부 evidence URL이 base path를 잃음 | 계획·사전 비교·실행 근거와 PNG URL이 `/spinon/` 아래에서 성공했다. |
| 26 | 코드 변경이 없는 docs PR을 runtime feature로 표현 | PR #481은 status·계획·public HTML만 갱신했으며 runtime 구현은 PR #480에 귀속한다. |
| 27 | PR 라벨 유형 또는 영역 누락 | `documentation`, `area: docs`, `area: layout` 라벨이 붙었다. |
| 28 | main 반영 전 후보를 live Tailnet 결과라고 표기 | 별도 pass41 build는 PR #481 merge 뒤 origin/main tree와 일치함을 확인했다. |
| 29 | preview hash 검증을 production release proof로 확대 | README는 Tailnet 미리보기 동기화만 보고하며 production 배포를 주장하지 않는다. |
| 30 | 문서 변경 검토 절차 누락을 숨김 | 이 파일을 회고 검토로 분명히 표시하고, PR #481의 사전 30개 gate를 충족한 것처럼 서술하지 않는다. |

## 판정

현재 공개 상태와 merge 근거는 일치한다. 30개 항목 검토는 merge 후에 수행됐으므로 PR #481의 사전 diff review 절차 누락을 소급해 해소하지 않는다.
