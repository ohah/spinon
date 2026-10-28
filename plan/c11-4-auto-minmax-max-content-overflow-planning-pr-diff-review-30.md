# C11.4 `minmax(auto, max-content)` 계획 PR 변경분 검토 · 실패 관점 30개

| # | 변경분에서 확인한 실패 경로 | 판정 |
|---:|---|---|
| 1 | 새 세부 계획이 C11.4 상위 계획에서 연결되지 않음 | 통과 — 상위 계획 첫 항목에 10-case 사전 비교와 세부 계획 링크가 추가됐다. |
| 2 | 공식 상태 대장에 다음 후보가 누락됨 | 통과 — `spec/STATUS.md`에 계획 상태와 기대 범위가 기록됐다. |
| 3 | 공개 로드맵이 계획을 구현 결과로 표현함 | 통과 — Chrome baseline과 규범 계산만 고정한 다음 계획으로 표시한다. |
| 4 | C11.4 체크박스 또는 공식 완료 수가 올라감 | 통과 — 미완료와 `72/213`을 유지한다. |
| 5 | 병합된 #749/#750의 범위가 과장됨 | 통과 — 기존 `minmax(auto, 1fr)` test-only 범위와 #750의 앱 경로 미변경을 구분한다. |
| 6 | 이전 PR의 빈 체크 rollup을 CI 통과로 표현함 | 통과 — GitHub check rollup이 비어 있음을 명시한다. |
| 7 | 새 계획이 #750과 동일한 track maximum을 시험한다고 오해됨 | 통과 — 새 입력은 `minmax(auto, max-content)`이고 기존 `1fr` 입력과 분리한다. |
| 8 | CSS Grid §6.6의 automatic minimum 조건을 잘못 요약함 | 통과 — computed overflow, `auto` 최소 track, 여러 track span 시 flexible track 조건을 적었다. |
| 9 | `overflow: clip`을 scrollable 값으로 취급함 | 통과 — `visible`과 함께 content-based control로 둔다. |
| 10 | `overflow: hidden`을 non-scrollable로 잘못 분류함 | 통과 — `hidden`, `auto`, `scroll` 모두 0 automatic minimum으로 둔다. |
| 11 | Minimum contribution과 최종 track 크기를 혼동함 | 통과 — 두 값을 별도로 검사하도록 요구한다. |
| 12 | Max-content maximum의 기여도가 계획에서 빠짐 | 통과 — 모든 입력에서 MaxContent contribution 값을 검사한다. |
| 13 | scrollable 값에서 minimum이 0이면 max-content 기여도도 사라진다고 가정함 | 통과 — 기대 기여도는 Columns 100px, Rows 10px로 유지한다. |
| 14 | Columns 수치가 Chrome observation에서만 복사됨 | 통과 — Ahem 글자 폭과 definite 60px root를 별도 산술 근거로 기록한다. |
| 15 | Rows transferred size가 불명확함 | 통과 — natural image 24×16px와 CSS width 15px에서 block size 10px을 도출한다. |
| 16 | 행의 item이 stretch되어 5px track과 같아짐 | 통과 — `align-self:start`와 15×10px item frame을 고정한다. |
| 17 | Rows 기대값이 text layout 높이에 좌우됨 | 통과 — replaced-image snapshot 경로로 제한하고 text height는 명시적으로 제외한다. |
| 18 | overflow 값이 Grid root에 지정되어 item minimum을 시험하지 못함 | 통과 — 각 값은 Grid item에 지정되고 computed value도 관측한다. |
| 19 | root의 해당 축 크기가 indefinite가 됨 | 통과 — Columns 60px, Rows 5px로 definite axis를 고정한다. |
| 20 | 명시 min-size가 automatic minimum을 덮음 | 통과 — 기본 `auto` minimum과 computed minimum 관측을 요구한다. |
| 21 | browser scroll size가 border-box item 크기로 오인됨 | 통과 — used track과 item frame을 기대값으로 삼고 scroll size는 진단 정보로만 둔다. |
| 22 | 선언된 track과 computed track의 차이를 놓침 | 통과 — Chrome 원 관측에 computed track과 root/item rect가 함께 보존된다. |
| 23 | 폰트 또는 PNG decode 전에 값을 수집함 | 통과 — Ahem 준비와 모든 image decode를 기다린 뒤 수집한다. |
| 24 | viewport 좌표를 root-relative 좌표처럼 비교함 | 통과 — 원 관측과 기대 JSON에서 root-relative frame을 분리한다. |
| 25 | Chrome screenshot을 Spinon runtime 결과로 오인함 | 통과 — README와 로드맵에서 Chrome 기준 화면이라고 명시한다. |
| 26 | test-only 변경에 Android/iOS simulator 통과를 요구하거나 주장함 | 통과 — 앱 route 변경이 없어 simulator 검증 대상이 아니라고 기록한다. |
| 27 | 계획의 10개 case 중 overflow 값이나 축이 빠짐 | 통과 — 두 축 모두 다섯 overflow 값을 JSON, 계획, fixture에서 확인한다. |
| 28 | README·계획·STATUS·로드맵의 100/60px 및 10/5px 결과가 다름 | 통과 — 네 문서가 같은 두 그룹의 track 수치를 사용한다. |
| 29 | 계획 PR 자체의 diff 검토와 향후 구현 PR 검토가 하나로 합쳐짐 | 통과 — 계획 PR 30개 검토는 별도 파일이고, 향후 구현/runtime 20개 검토는 별도 gate다. |
| 30 | 좁은 matrix를 전체 Grid 자동 최소값, CSS admission 또는 C11.4 완료로 확대함 | 통과 — 제외 범위와 공식 미완료 상태를 계획·상태·로드맵에서 반복해 한정한다. |

계획 변경은 사전 비교, 범위, 검증 기준 및 공식 상태 연결에 한정된다. 제품 코드나 앱 runtime은 변경하지 않으며, 이 계획 PR 검토를 이후 구현 검토의 대체물로 사용하지 않는다.
