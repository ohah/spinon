# C11.1 Root HTML named-area 계획 PR · 변경분 적대 검토 30개

**검토 대상:** 계획 전용 변경분: `plan/c11-root-html-grid-template-areas.md`, 계획 실패 관점 기록, 고정 Chrome fixture/expected/observation/screenshot, C11 상태·상위 계획·공개 roadmap의 계획 표시. 구현 source와 simulator 결과는 아직 없다.

| # | 변경분 실패 관점 | 판정 |
|---:|---|---|
| 1 | 계획 문서에 제품 Rust/Java/Swift/FFI 구현이 섞임 | **통과.** 변경 파일의 product source에는 layout 동작 변경이 없다. |
| 2 | 계획 review 20개가 PR diff review 30개를 대신함 | **통과.** 이 표는 별도로 계획·evidence·상태 연결을 검토한다. |
| 3 | PR 계획을 구현 완료로 기록함 | **통과.** plan, STATUS와 public roadmap이 구현 미시작이라고 구분한다. |
| 4 | C11.1 checkbox가 제한 후보만으로 체크됨 | **통과.** checkbox는 계속 미완료다. |
| 5 | C11.3.3/C11.4/C11 parent가 바뀜 | **통과.** 모든 상위 항목은 미완료로 남는다. |
| 6 | 공식 완료 `72/213` 집계가 변경됨 | **통과.** 상태 문구가 수치를 그대로 고정한다. |
| 7 | C11.1 기존 #1000/#1002 범위가 named area 지원으로 소급 확대됨 | **통과.** 이 plan은 별도 다음 후보로 기록했다. |
| 8 | 범위가 기본 parsed Root HTML route가 아닌 opt-in fixture route로 새어 나감 | **통과.** 계획은 `new_runtime_gpu_with_root_html()`만 목표로 명시한다. |
| 9 | existing C11.3 typed area profile이 검증 근거로 재사용됨 | **통과.** C11.3은 typed machinery 참고일 뿐 기본 pipeline은 새로 연결·검증해야 한다. |
| 10 | “CSS Grid area 지원”으로 범위를 과장함 | **통과.** 단일 고정 rectangular 2×2 profile과 세 area만 지원 후보로 둔다. |
| 11 | standard source draft 상태를 숨김 | **통과.** 계획은 W3C 2025 CRD를 기준으로 이름 붙이고 제안 구현 slice로 한정한다. |
| 12 | normative frame을 Chrome 결과에서 역산함 | **통과.** expected geometry는 fixed track sum과 cell indices에서 계산했다. |
| 13 | Chrome observation이 독립 기대값인 것처럼 사용됨 | **통과.** Browser observation은 교차 검사라고 분리한다. |
| 14 | Chrome binary/viewport/DPR를 잃음 | **통과.** HeadlessChrome 156, 1280×633 CSS px, DPR 1을 JSON에 고정했다. |
| 15 | browser observation이 다른 fixture와 혼합됨 | **통과.** local source URI가 expected/README에 동일하게 기록되어 있다. |
| 16 | header가 두 columns를 덮지 못하거나 row를 혼동함 | **통과.** 기대 root/item frames와 area cell range가 독립 JSON에 고정돼 있다. |
| 17 | DOM reverse가 실제 Chrome에서 관찰되지 않음 | **통과.** 원본과 reversed semantic frames를 각각 JSON에 보존했다. |
| 18 | source order reverse가 semantic owner를 DOM index에 종속시킴 | **통과.** item은 `data-area` semantic label로 측정한다. |
| 19 | duplicate, missing 또는 unknown area item을 성공 처리함 | **통과.** 계획의 negative matrix에서 scene publication 전 reject하도록 요구한다. |
| 20 | area template와 explicit tracks dimension이 다를 때 partial output이 남음 | **통과.** dimensions 일치 검사와 fail-closed 동작을 acceptance에 넣었다. |
| 21 | supported area가 implicit track을 만들어 낼 수 있음 | **통과.** 정해진 두 행·두 열 밖 placement와 implicit tracks를 제외한다. |
| 22 | authored numeric/span placement나 non-default flow가 이 slice에 섞임 | **통과.** 세 named area의 `grid-area`만 허용하도록 한정했다. |
| 23 | text/image/nested content가 빈 item profile을 넘어옴 | **통과.** empty direct HTML block item만 계획 대상으로 적었다. |
| 24 | invalid CSS declaration drop과 valid unsupported value가 같은 오류로 취급됨 | **통과.** cascade invalid와 computed-value rejection controls를 구분한다. |
| 25 | stale style/layout revision이나 prior scene cache가 새 성공으로 재사용됨 | **통과.** same-revision 확인과 stale-revision negative를 별도로 요구한다. |
| 26 | overflow clip/scroll 지원으로 오해함 | **통과.** visible overflow 기존 조건과 clipping/scrolling 미지원이 분리돼 있다. |
| 27 | screenshot이 Spinon 결과나 pixel equality로 보임 | **통과.** screenshot을 Chrome precomparison으로만 표기했다. |
| 28 | simulator/device 실행을 이미 통과했다고 오해함 | **통과.** plan, STATUS와 roadmap이 simulator 검증 전이라고 쓴다. |
| 29 | 새 Markdown 경로가 이번 docs build dead-link 오류에 추가됨 | **통과.** local fixture/PNG 링크를 code path/Markdown image로 바꾸고 plan link를 GitHub main URL로 바꿨다. 전체 docs build는 기존 1,223 dead-link groups로 실패했으며 log 검색에서 새 plan/evidence 경로는 나오지 않았다. |
| 30 | evidence integrity/hash와 Agent Browser cleanup이 누락됨 | **통과.** manifest는 6 input/evidence files를 검증했고 browser close 뒤 process scan 결과를 README와 JSON에 기록했다. |

**검토 후 보정:** 최초 expected JSON 초안에 arithmetic expression을 JSON value처럼 넣은 점을 parser가 잡아 결과 숫자로 고쳤다. 첫 docs build는 local fixture/PNG hyperlink를 dead route로 분류해 Markdown image/code path로 정정했고, plan link는 GitHub URL로 바꿨다. 최종 기계 verifier와 SHA-256 manifest는 통과했다. 프로젝트 전체 docs build는 새 경로와 무관한 기존 dead-link 문제 1,223개 그룹 때문에 통과하지 못했다.

**판정:** 계획 PR diff에서 미해결 blocker를 찾지 못했다. 이 검토는 구현 승인이나 browser/Android/iOS runtime pass가 아니다.
