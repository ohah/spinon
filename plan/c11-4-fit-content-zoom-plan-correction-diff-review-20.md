# C11.4 zoomed `fit-content()` plan correction PR diff review 20

검토 대상은 #781에 병합한 plan/evidence의 Stylo→Taffy zoom 좌표 보정 diff다. Chrome baseline이나 source audit를 실제 Spinon runtime 결과로 취급하지 않는지 확인했다.

| # | 실패 관점 | 확인·조치 | 판정 |
|---:|---|---|---|
| 1 | 계획 PR의 기존 `40px` 기대를 조용히 덮어써 이력을 잃을 수 있음 | 보정 상태와 source audit 근거를 계획·README에 명시하고 이전 PR diff review 파일은 그대로 둔다. | PASS |
| 2 | WPT가 직접 assert하는 CSSOM value를 `80px`로 잘못 바꿀 수 있음 | Browser-observed, pinned WPT CSSOM expected는 `40px`으로 유지한다. | PASS |
| 3 | Taffy row를 unzoomed `40px`로 계속 검사할 수 있음 | Stylo typed `fit-content` input 80px, Taffy used row 80px로 계획 expected를 갱신했다. | PASS |
| 4 | zoomed max-content contribution이 authored 400px에 남을 수 있음 | Author child height 400px과 effective-zoom Taffy contribution 800px을 분리했다. | PASS |
| 5 | `min-height:0`이 zoom 후 0이 아닌 값이 될 수 있음 | Zero minimum은 invariant로 유지하고 Grid formula에 0을 사용한다. | PASS |
| 6 | CSS Grid formula의 argument/max-content 순서를 뒤집을 수 있음 | Grid CRD formula에서 `max(0,min(80,800))=80`을 계산한다. | PASS |
| 7 | CSS zoom이 used values와 CSSOM serialization에 미치는 효과를 섞을 수 있음 | CSS Viewport draft §4/4.1과 unscaled CSSOM/zoomed used lengths를 다른 assertion으로 명시한다. | PASS |
| 8 | Dynamic CSS Zoom draft를 immutable standard로 주장할 수 있음 | Source pins에 draft date, WIP status, access date와 section을 기록했다. | PASS |
| 9 | Stylo 버전이 바뀌어 source audit 재현이 안 될 수 있음 | Repository dependency `stylo = 0.22.0`으로 source audit version을 고정했다. | PASS |
| 10 | CSS lengths가 실제로 zoomed된다는 근거가 추측일 수 있음 | Stylo source audit는 effective-zoom absolute conversion, Spinon grid extractor는 typed length `.px()` projection을 연결한다. | PASS |
| 11 | Computed `width` input에서 root zoom을 두 번 곱할 수 있음 | Grid adapter가 이미 scaled typed CSS px를 받는 경로를 적고 second-scale를 금지한다. | PASS |
| 12 | Item/child local zoom과 inherited effective zoom이 혼동될 수 있음 | Root specified/effective `2`, item·child specified/effective `1/2`를 검증하도록 한다. | PASS |
| 13 | Width normalization이 row track/geometry를 바꿀 수 있음 | Browser original/normalized CSSOM rows와 item/child visual heights는 기존 값을 유지한다. | PASS |
| 14 | Fixed root visual height를 잘못 업데이트할 수 있음 | Author height 300px과 zoomed layout/frame height 600px을 분리한다. | PASS |
| 15 | Runtime fixture 기준이 execution proof처럼 보일 수 있음 | STATUS, public roadmap, README, plan은 source audit/expected values일 뿐 app runtime은 미실행이라고 적는다. | PASS |
| 16 | Plan review가 이전 좌표 모델을 승인한 채 남을 수 있음 | 계획 실패 관점 20개 파일을 corrected contract에 맞춰 다시 검토했다. | PASS |
| 17 | PR diff review가 계획 review를 대신할 수 있음 | 이 PR에는 별도의 correction diff review 20개를 추가한다. | PASS |
| 18 | Existing #781 diff review를 새 diff 검토로 재활용할 수 있음 | #781 historical review remains unchanged; new correction review has independent perspectives. | PASS |
| 19 | Planning note가 C11.4 completion count를 올릴 수 있음 | STATUS와 public roadmap 모두 checkbox 미완료와 `72/213`을 유지한다. | PASS |
| 20 | PNG/source evidence를 잃거나 PR 본문에서 깨질 수 있음 | Existing pinned WPT screenshot and source hashes remain linked for correction PR body. | PASS |

**결과:** correction PR diff 20/20 관점 PASS. 이는 계획 값을 바로잡은 문서 변경이며, Rust tests, app runtime, Android/iOS Simulator 결과를 뜻하지 않는다.
