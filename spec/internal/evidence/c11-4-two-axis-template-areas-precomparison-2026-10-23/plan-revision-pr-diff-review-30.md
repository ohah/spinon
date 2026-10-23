# C11.4 양 축 auto-repeat + areas cascade 계획 수정 PR diff 적대 검토 30개

**검토 대상:** `plan/c11-4-two-axis-template-areas.md`, `plan/c11-4-two-axis-template-areas-cascade-plan-review-20.md`, [`cascade-source-audit.md`](./cascade-source-audit.md).

**검토 기준:** PR #543 병합 뒤 구현 전 source trace에서 cascade guard를 새로 발견한 계획 수정이다. 기존 계획 PR의 30관점 결과를 이 PR 검토에 재사용하지 않았다. 30관점 모두 이 계획 수정 PR을 막는 결함이 없었다.

| # | PR diff 실패 관점 | 확인 결과 | 판정 |
| --- | --- | --- | --- |
| 1 | plan이 projection guard만 차단점으로 설명한다. | cascade와 projection의 두 차단점을 분리해 적었다. | 통과 |
| 2 | cascade guard가 어디서 실행되는지 불명확하다. | `computed_grid_layout`에서 두 axis를 계산한 직후 reject하는 흐름을 기록했다. | 통과 |
| 3 | `allow_auto_repeat`가 target profile 외에도 true라고 가정한다. | `element.rs` call site에서 Profile predicate를 추적했다. | 통과 |
| 4 | `allow_integer_repeat`과 `allow_auto_repeat` 인자를 하나의 권한으로 혼동한다. | 세 profile flag를 각각 기록하고 areas parsing과 auto-repeat 경계를 분리했다. | 통과 |
| 5 | cascade guard를 무조건 제거하는 구현이 계획에 들어간다. | 조건 변경은 target-only이며 TemplateV1의 auto-repeat 거부를 보존한다. | 통과 |
| 6 | projection에서 non-target profile gate까지 제거한다. | track-sizing profile gate를 유지하고 중복 two-axis-area 거부만 대상이다. | 통과 |
| 7 | target profile이 양 축 repeat metadata 하나를 빠뜨린다. | `ComputedCssGridLayout`의 양 축 repeat과 areas 동시 보존을 성공 expectation에 넣었다. | 통과 |
| 8 | cascade가 만든 area가 projection에서 다시 없어질 수 있다. | `GridContainerInput` 및 Taffy 경계까지 보존 여부를 추적한다. | 통과 |
| 9 | Taffy가 받은 두 axis result가 서로 덮어쓴다. | per-axis track array·repeat count를 독립 대조한다. | 통과 |
| 10 | same-axis multiple auto-repeat까지 확장된다. | 같은 track list의 두 번째 auto-repeat은 계속 unsupported다. | 통과 |
| 11 | no-area dual-axis repeat에 변경이 번진다. | 기존 12-scene no-area matrix를 exact regression으로 둔다. | 통과 |
| 12 | single-axis area case가 변경 중 깨진다. | 기존 13-scene matrix를 exact regression으로 둔다. | 통과 |
| 13 | CSS parse failure가 cascade guard보다 늦어져 잘못된 오류가 난다. | invalid area/repeat input은 기존 구체 오류로 fail-closed하도록 계획했다. | 통과 |
| 14 | cascade failure가 새 revision을 부분 publish한다. | snapshot/layout publication 전 오류와 revision/frame negative control을 계획했다. | 통과 |
| 15 | area matrix와 repeat count가 엮여 fill-count가 달라진다. | repeat 수는 CSS 규범 기준으로 독립 계산하고 Chrome oracle는 따로 보존한다. | 통과 |
| 16 | auto-fit empty areas가 occupancy를 만든다. | `fit-fit-empty`와 actual in-flow item occupancy를 고정했다. | 통과 |
| 17 | longhand alias 검증이 row 또는 column 한쪽만 실행된다. | `area-longhand`는 둘 다 검사한다. | 통과 |
| 18 | extra area tracks가 일반 `grid-auto-*` 문법을 열어버린다. | fixed `16px`/`12px` scene 외 새 grammar는 계획에서 제외했다. | 통과 |
| 19 | source audit이 바뀐 commit에서 실행됐다. | audit은 PR #543 merge SHA `8fb233be...`에 묶여 있다. | 통과 |
| 20 | source audit 파일을 구현 결과처럼 표현한다. | 파일과 plan에 코드 미변경·미실행 상태가 명시됐다. | 통과 |
| 21 | CSS Grid normative oracle가 다시 Chrome에서 역산된다. | 수정은 source path 설명만 바꾸며 9-scene 독립 기대/observation은 유지한다. | 통과 |
| 22 | 20관점 리뷰가 기존 표를 수정만 하고 새로운 perspective가 없다. | cascade source gate 전용 새 20관점 파일을 추가했다. | 통과 |
| 23 | 계획 PR diff 30관점이 기존 #543 evidence와 같은 파일이다. | amendment PR 전용 새 30관점 review 파일이다. | 통과 |
| 24 | 공식 C11.4 checkbox나 `77/213`이 계획 PR에서 변경된다. | `spec/STATUS.md`와 `0001` 상태를 변경하지 않았다. | 통과 |
| 25 | 내부 계약 숫자 버전이 계획에 따라 오른다. | `0059`는 계속 `0.1.0`으로 유지한다. | 통과 |
| 26 | amendment가 public CSS admission으로 잘못 게시된다. | 고정 runtime profile/fixture 한정과 WPT 미완료를 유지한다. | 통과 |
| 27 | simulator 행렬에서 실기기를 선택할 가능성이 남는다. | API 37 AVD serial과 iOS Simulator UDID를 고정하고 physical device를 배제한다. | 통과 |
| 28 | overflow가 geometry만으로 통과 처리된다. | area-added scene의 실제 PixelCopy/Metal readback points를 유지한다. | 통과 |
| 29 | Agent Browser PR image link policy가 plan 수정 중 바뀐다. | 기존 merged PR screenshot과 browser-cleanup 근거를 수정하지 않았다. | 통과 |
| 30 | 계획 수정이 구현 PR 검토를 대체한다. | 구현 뒤 새 runtime 20관점, 구현 PR diff 30관점, Tailnet sync PR diff 30관점을 유지한다. | 통과 |

## 판정

수정된 계획은 cascade와 projection 양쪽의 실제 source guard를 다룬다. 수정 PR은 문서·검토 근거만 변경하며 Rust source, STATUS counter, runtime fixture와 browser session을 변경하지 않는다. 기능 구현은 이 수정 계획이 main에 병합된 뒤 시작한다.
