# C11.4 positive-gap auto-repeat 상한 구현/runtime 실패 관점 검토 20개

**대상:** `auto_repeat_positive_gap_preserves_named_line_at_the_ceiling_on_both_axes`, gap-aware input helper, [고정 기대값](./README.md), [실행 로그](./checks/)

| # | 실패 관점 | 판정 |
|---|---|---|
| 1 | repetition count가 gutter를 무시한다 | **통과.** exact-fit과 cap basis를 `6+5R` 독립 식으로 고정했고 두 값 모두 실행했다. |
| 2 | 반복 내부 두 track 사이 gap을 놓친다 | **통과.** 각 반복에 2개 track이 있으므로 반복당 gap 2개가 식과 input에 들어간다. |
| 3 | prefix/suffix gap 경계를 잘못 세어 반복 횟수가 달라진다 | **통과.** 전체 sequence에서 track 수 `2+2R`, gutter 수 `1+2R`를 사용한다. |
| 4 | fixed prefix 또는 suffix 크기가 반복 크기에 섞인다 | **통과.** 2px/3px 비반복 track과 1px+2px 반복 track을 기대값에서 분리했다. |
| 5 | exact-fit 값이 실제로 4,999회 repetition을 고르지 않는다 | **통과.** 25,001px 입력에서 `6+5×4,999`가 정확히 일치하고 모든 track size를 비교한다. |
| 6 | cap 요청 값이 5,000회보다 크거나 작다 | **통과.** 25,006px 입력에서 `6+5×5,000`을 확인했다. |
| 7 | exact-fit이 10,000개의 explicit track으로 끝나지 않는다 | **통과.** resolved explicit count와 gutter 제외 Initialize track 수를 각각 10,000과 비교한다. |
| 8 | cap에서 prefix가 보존되지 않는다 | **통과.** 두 scene 모두 첫 track이 2px인지 전체 size vector로 확인한다. |
| 9 | 마지막 repeated pattern이 짝수 track 경계에서만 잘린다 | **통과.** cap vector는 4,999개 완전 pair 뒤에 1px repeated track을 남긴다. |
| 10 | source-order truncation이 suffix를 남기거나 반복 track을 버린다 | **통과.** exact-fit 마지막은 3px suffix, cap 마지막은 1px repeated track이다. |
| 11 | named-line `-1`이 마지막 완전 반복과 cap 경계를 혼동한다 | **통과.** exact-fit과 cap에서 서로 다른 예상 item offset을 직접 검사한다. |
| 12 | item frame 계산이 gap을 빼고 track 크기만 누적한다 | **통과.** 기대 offset `24,993px`·`24,998px`은 선행 1px gutter를 포함하며 실행과 일치한다. |
| 13 | Columns와 Rows의 gap property가 서로 바뀐다 | **통과.** 각 axis run에 해당 축 gap만 주고 axis별 frame 좌표를 검사한다. |
| 14 | orthogonal axis의 1px fixed track 또는 item이 target axis sizing을 바꾼다 | **통과.** orthogonal axis는 고정 1px이며 target axis result를 별도로 읽는다. |
| 15 | item intrinsic contribution이 auto-repeat sizing에 영향을 준다 | **통과.** child width/height를 고정 1px로 두고 contribution 변화 경로를 만들지 않는다. |
| 16 | auto-fit collapse 동작을 auto-fill 결과로 혼동한다 | **통과.** 새 행렬은 `auto-fill`이며 모든 track의 non-collapsed 상태를 확인한다. |
| 17 | `Initialize` trace가 아닌 다른 pass의 track vector를 사용한다 | **통과.** 기존 helper가 마지막 target-axis pass에서 단일 Initialize stage를 요구한다. |
| 18 | 큰 trace가 state budget을 초과한다 | **통과.** 네 실행 모두 captured state `<100,000` 단언을 통과했다. |
| 19 | 기존 zero-gap 경로의 helper refactor가 동작을 바꾼다 | **통과.** 기존 두-track helper는 `LayoutGap::default()` wrapper를 사용하며 기존 ceiling과 전체 53 Grid sizing tests가 통과했다. |
| 20 | test-only 내부 clamp가 공개 CSS Grid 완료로 보고된다 | **통과.** README와 상태 갱신은 fixture/CSS admission/WPT 한계를 유지하고 C11.4/C11 및 `72/213`을 미승격한다. |

**판정:** 구현/runtime 실패 관점 20/20 통과. 결함은 발견되지 않았다. 계획 검토와 별도의 실제 테스트·trace·frame 비교이며 최종 PR diff 검토를 대신하지 않는다.
