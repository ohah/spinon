# C11.4 auto-repeat × negative line 계획 검토 20개

**대상:** `plan/c11-4-auto-repeat-negative-lines.md`와 고정 사전 fixture/oracle. 코드 구현 후 검토와 PR 최종 diff 검토를 대신하지 않는다.

**정정:** 이 초기 검토는 auto-repeat 분기 dispatch를 놓쳐 `track_sizing::normalize_track_sizing_grid_container`가 numeric negative line을 확장 전 길이로 해석한다는 잘못된 구현 가설을 통과시켰다. 해당 결론은 무효이며 [정정 계획 재검토 20개](./plan-correction-review-20.md) 및 [정정 PR diff 검토 30개](./pr-correction-diff-review-30.md)로 대체한다.

| # | 독립 실패 관점 | 검토와 반영 |
| ---: | --- | --- |
| 1 | fixed column size에서 repeat count 산식이 gap을 한 번 빼먹는 경우 | 고정 prefix/suffix와 repeat gap 수를 따로 전개했다. `38 + 48n`이며 134/200/230px은 2/3/4회다. 기대값 JSON validator가 동일 모델을 독립 계산한다. 통과. |
| 2 | row gap 개수를 column과 혼용해 rows 반복 count가 틀리는 경우 | `26 + 36n`을 별도 산식으로 두고 134/180/206px에서 3/4/5회를 확인한다. 통과. |
| 3 | `minmax(40px, 1fr)`에서 1fr을 definite maximum으로 오해하는 경우 | pinned CRD 규칙상 definite하지 않은 max는 fixed minimum을 반복 수 basis로 사용한다. validator가 40px/30px를 별도 basis로 쓴다. 통과. |
| 4 | auto-fit이 occupancy를 count 계산에 먼저 반영해 auto-fill과 다른 count를 내는 경우 | count와 post-placement collapse를 분리했고 18-case oracle에 empty/leading/occupied controls를 둔다. 통과. |
| 5 | negative line `-1`을 마지막 track으로 오해하는 경우 | boundary 수식 `E + 1 + k`를 사용한다. E tracks에서 `-1`은 마지막 boundary E다. fixture는 expanded line count가 다른 2/3/4 및 3/4/5 count를 확인한다. 통과. |
| 6 | authored repeat pattern 한 번만 포함한 vector 길이로 line을 계산하는 경우 | 현재 normalizer의 `columns.len()`/`rows.len()`과 Taffy 후속 repeat 계산 순서 차이를 정확히 문제 경계로 기록했다. runtime 실패는 아직 주장하지 않는다. 통과. |
| 7 | auto-repeat track이 explicit grid line 번호에 포함되지 않는다고 구현하는 경우 | fixed CSS Grid 2025 CRD를 기준으로 negative line은 explicit grid 끝에서 역산한다. auto-repeat count가 explicit track 수에 포함되는 규칙을 count table과 함께 명시했다. 통과. |
| 8 | leading implicit list를 explicit grid 뒤 방향으로 순환하는 경우 | `[31,47]`의 left-to-right leading 순서 `[31,47]`, nearest-explicit 47을 별도 규칙과 browser case로 고정했다. rows도 독립 pattern이다. 통과. |
| 9 | track 한 개와 두 개의 leading cycle이 같은 값만 검증하는 경우 | 1-track 및 2-track leading placement를 columns/rows 모두 포함하고 JSON이 첫 leading sizes를 각각 정확히 검사한다. 통과. |
| 10 | repeat-only axis를 neighbor가 있는 axis와 같은 line count로 계산하는 경우 | `columns-fill-repeat-only-leading`을 넣고 prefix/suffix count 0의 E 계산을 독립 검증한다. 통과. |
| 11 | prefix를 가로지르는 span이나 reversed line pair를 정렬하지 못하는 경우 | `-8/-5` prefix-crossing span과 `-5/-7` reversed pair를 별도 관찰한다. 경계 매핑과 normalized used-track 시작 좌표가 검사된다. 통과. |
| 12 | positive line의 기준을 negative line과 함께 잘못 이동하는 경우 | `6/7` trailing implicit control을 둔다. positive boundary는 `line - 1`이며 trailing `grid-auto-*`는 forward order로 검증한다. 통과. |
| 13 | CSS line 0이나 zero/negative span이 정상 placement로 바뀌는 경우 | line zero와 invalid span은 Rust projection/layout rejection regression으로 명시했다. browser fixture의 positive oracle와 성공 input에 넣지 않는다. 통과. |
| 14 | auto-fit empty track을 collapse하면서 repeat count도 줄이는 경우 | count는 auto-fill과 동일하고 track list의 0px slot은 남겨 둔다. validator가 repeated slots와 collapsed indices를 separately 확인한다. 통과. |
| 15 | Chrome의 collapsed gutter 결과를 normative expectation으로 오인하는 경우 | prefix 앞 gutter 차이를 발견·기록하고 known Blink divergence로 분리했다. validator는 auto-fit frame을 규범값으로 쓰지 않는다. 통과. |
| 16 | box sizing/border/padding 때문에 fixture available size가 산식과 다른 경우 | browser grid에 border/padding 없이 fixed definite content-box dimension을 사용한다. observation과 validator가 grid rect의 width/height를 확인한다. 통과. |
| 17 | columns와 rows auto-repeat count/range가 서로 덮이는 경우 | 2-axis fill/fit fixture, per-axis expected record와 독립 `GridAutoRepeat` validation을 계획했다. matrix union에서도 axis를 분리한다. 통과. |
| 18 | sparse/dense cursor 또는 hidden item이 repeat 범위에 잘못 반영되는 경우 | hidden/overlap/order/sparse/dense controls를 layout regression에 추가한다. out-of-flow/nested semantics는 scope 밖이라고 분명히 했다. 통과. |
| 19 | large negative extent에서 overflow, cap 초과 또는 partial output이 발생하는 경우 | signed arithmetic, 9,999/10,000/10,001 cap, extreme line와 allocation-failure controls 및 Spinon-owned no-partial-output 조건을 명시했다. Taffy global OOM guarantee는 제외했다. 통과. |
| 20 | simulator/review 증거가 Chrome screenshot이나 기존 matrix ID를 중복 계산하는 경우 | 18 new IDs, 기존 106 matrix와 기존 33 auto-repeat IDs exact subset, Android API 37 AVD/iOS 26.2 Simulator, PixelCopy/Metal readback 시점을 분리해 요구한다. Chrome 이미지에는 oracle label을 붙이고 border stroke 주장을 금지한다. 통과. |

## 판정

20개 관점에서 계획 누락을 수정한 뒤 재검토했다. 계획과 oracle은 구현을 시작할 준비가 됐다. 코드/runtime 결과, simulator 증거와 implementation PR diff는 아직 없다.
