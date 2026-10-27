# C11.4 auto-repeat intrinsic-size 재계산 계획 · 실패 관점 20개 검토

검토 대상은 [집중 계획](./c11-4-auto-repeat-intrinsic-recompute.md)과 Chrome/WPT 사전 비교 evidence다. 검토는 구현 전에 계획의 범위·재현성·판정 기준을 대상으로 했다.

| # | 별도 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | WPT 결과를 Spinon 결과로 오해할 수 있음 | **PASS.** WPT harness 8/8은 Chrome oracle로 한정했고 Spinon WPT pass를 명시적으로 부인한다. |
| 2 | WPT 소스나 지원 CSS가 upstream에서 바뀌면 기대값 근거가 흔들림 | **PASS.** WPT revision, test path와 각 파일 hash를 manifest에 고정했다. |
| 3 | Chrome 좌표를 독립 CSS 기대값으로 복사했을 수 있음 | **PASS.** observation과 expected JSON을 분리하고 auto-repeat·track sizing·intrinsic contribution 계산을 별도 산출로 설명한다. |
| 4 | auto-repeat이 track width와 gutter 계산 때문에 off-by-one될 수 있음 | **PASS.** 반복 수·각 track 폭·probe geometry를 상태별로 고정하고 gap 없는 기준 입력도 명시했다. |
| 5 | max-content와 min-content 상태가 다른 입력으로 혼동될 수 있음 | **PASS.** 동일 Grid에서 처음 네 폭은 max-content, 다음 네 폭은 min-content 순서로 실행한다. |
| 6 | 8개 독립 DOM을 계산해도 동적 재계산을 주장할 수 있음 | **PASS.** runtime sequence는 같은 HostDocument와 node handle을 요구한다. Simulator의 독립 scene matrix는 앱 projection 확인에만 쓰고 동적 invalidation 증거로 보지 않는다. |
| 7 | DocumentRevision이 진행하지 않아 이전 source 결과를 재사용할 수 있음 | **PASS.** 각 mutation에서 source document revision 진행을 acceptance gate로 둔다. |
| 8 | StyleRevision이 입력 소유자와 동기화되지 않을 수 있음 | **PASS.** 모든 단계에서 style revision 증가와 환경 revision 고정을 각각 검사한다. |
| 9 | 스타일 변경 invalidation이 실제 변경 노드를 포함하지 않을 수 있음 | **PASS.** 각 mutation의 invalidation 결과가 변경을 포함하고 이전 revision 재사용이 없음을 확인한다. |
| 10 | 사용 CSS 값은 같지만 WPT item mutation 순서가 달라질 수 있음 | **PASS.** `100 → 80 → 15 → 120` 순서를 두 intrinsic mode에서 반복하도록 고정했다. |
| 11 | root border/padding이 item/probe 좌표에 잘못 더해질 수 있음 | **PASS.** root-relative WPT coordinates와 absolute layout frame을 분리해 기록한다. |
| 12 | absolute probes가 in-flow contribution으로 sizing을 오염시킬 수 있음 | **PASS.** 네 probe를 absolute로 고정하고 sizing에서 제외되며 geometry에만 포함되는 경계를 명시했다. |
| 13 | probe가 다른 containing block을 참조할 수 있음 | **PASS.** root의 `position:relative`와 containing-block geometry를 고정 입력에 포함했다. |
| 14 | probe가 없는 track line 또는 잘못된 line pair를 검사할 수 있음 | **PASS.** probe별 `1/2`, `2/3`, `3/4`, `4/5` placement를 계획 범위로 고정했다. |
| 15 | frame만 일치하고 실제 used tracks 또는 auto-repeat count가 틀릴 수 있음 | **PASS.** typed track list와 runtime repeat trace/count를 별도로 검사한다. |
| 16 | 이전 layout 결과가 화면에 남아도 새 계산 결과로 오인할 수 있음 | **PASS.** 각 단계의 revision stamp·trace·draw/present를 함께 기록한다. |
| 17 | opt-in fixture가 임의 CSS 입력을 넓게 admission할 수 있음 | **PASS.** root identity, topology, profile와 정확한 CSS declaration에 대한 allowlist를 둔다. |
| 18 | 전체 WPT나 C11.4 완료로 결과가 과장될 수 있음 | **PASS.** 복수 grammar/일반 admission/전체 WPT/C11.4를 제외 범위로 분명히 적었다. |
| 19 | 앱 시뮬레이터 결과가 실기기나 hardware GPU 결과로 보고될 수 있음 | **PASS.** Android/iOS simulator만 표기하고 실제 하드웨어 증거와 구분한다. 독립 scene matrix를 같은-document sequence로 부르지 않는 한계도 계획에 적었다. |
| 20 | 테스트 전용 slice 때문에 내부 계약 버전이나 공식 완료 수를 올릴 수 있음 | **PASS.** 공개 API 없음, 내부 계약 `0.1.0`, C11.4 unchecked, `72/213` 유지 조건을 명시했다. |

**판정:** 계획 관점 20/20 PASS. 본 계획은 8개 동적 상태 전체를 같은 문서·node identity·revision sequence로 검증하는 조건을 유지한다. 구현은 이 gate를 만족하는 runtime route를 만들 수 있을 때만 시작한다. 구현 후에는 별도의 새 20개 관점 검토를 수행한다.
