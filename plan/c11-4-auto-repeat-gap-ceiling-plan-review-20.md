# C11.4 positive-gap auto-repeat ceiling 계획 실패 관점 검토 20개

**대상:** [positive-gap 상한 계획](./c11-4-auto-repeat-gap-ceiling.md), 고정 Taffy `0.15.0` 계산 경로, 기존 [단일/두-track ceiling 계획](./c11-4-auto-repeat-pattern-ceiling.md)

| # | 실패 관점 | 판정 |
|---|---|---|
| 1 | 반복 횟수에 gutter를 빼먹어 exact-fit basis를 잘못 계산한다 | **통과.** 총 공간을 prefix/suffix `5px`, 반복 `3R`, 전체 gutter `1+2R`로 분리해 `6+5R`로 고정했다. |
| 2 | 반복 사이에만 gap을 넣고 패턴 내부 두 track 사이 gap을 누락한다 | **통과.** 반복 track 수 `2R`와 total track count 기반 gutter 수 `1+2R`를 명시했다. |
| 3 | prefix와 suffix를 auto-repeat 반복 크기에 포함해 반복 횟수가 바뀐다 | **통과.** 비반복 크기 `2px+3px`와 반복 크기 `3R`를 독립 계산한다. |
| 4 | `25,001px` exact-fit이 실제로 정수 repetition 수를 만들지 않는다 | **통과.** `6+5×4,999=25,001`이며 다음 반복은 5px 더 필요하다. |
| 5 | `25,006px` 입력에서 cap 전 5,000회가 요청되지 않는다 | **통과.** `6+5×5,000=25,006`으로 다음 경계를 고정했다. |
| 6 | prefix/suffix 포함 track 수를 잘못 세어 cap 결과가 10,001개가 된다 | **통과.** exact input은 `1+2×4,999+1=10,000`; cap input은 요청 10,002개다. |
| 7 | cap이 repeated pattern을 짝수 track 경계에서만 자른다고 가정한다 | **통과.** 남은 9,999 repeated slots가 마지막 `[1px,2px]` pattern을 첫 track 뒤에서 자르는지 전체 순서로 검사한다. |
| 8 | suffix가 잘리지 않고 retained track으로 들어간다 | **통과.** cap 기대 sequence는 prefix, 4,999개 완전 pair, 마지막 1px이며 3px suffix는 없다. |
| 9 | auto-repeat 계산의 1px UA floor가 authored 1px·2px 값을 바꾸는 사례다 | **통과.** 반복 track 모두 floor 이상이며 계획은 계산용 UA floor를 authored sizing 변경이라고 주장하지 않는다. |
| 10 | target axis gap을 반대 축에 넣거나 두 축에 모두 넣는다 | **통과.** 각 실행은 Columns 또는 Rows 한 축에만 1px gap, 다른 축에는 0px gap을 둔다. |
| 11 | gutter가 없는 trace에서 item offset이 우연히 맞는다 | **통과.** 0-gap 기존 예상값과 다른 `24,993px`·`24,998px`을 지정하고 frame으로 누적 gap을 확인한다. |
| 12 | negative named-line occurrence가 두 축 또는 source order에 따라 잘못 해석된다 | **통과.** 동일한 `pattern-start -1 / pattern-middle -1` frame을 Columns와 Rows 각각 검사한다. |
| 13 | item의 intrinsic 크기가 auto-repeat 계산에 영향을 준다 | **통과.** child는 두 축 모두 1px fixed이고 반복 track은 고정 길이다. |
| 14 | outer line-name metadata 길이가 반복 확장 규칙과 맞지 않는다 | **통과.** 세 pattern group과 `tracks.len() - repeated_track_count + 2 = 4` outer 빈 group을 명시했다. |
| 15 | basis가 border-box 또는 viewport 크기와 혼동된다 | **통과.** root content axis와 viewport를 동일한 definite basis로 두고 border/padding을 만들지 않는다. |
| 16 | 각 테스트가 두 축을 동시에 최대로 만들어 trace budget을 초과한다 | **통과.** 한 번에 target 축만 10,000 track으로 만들고 captured state `<100,000`을 검사한다. |
| 17 | resolved count만 맞고 `Initialize` stage에 다른 track sequence가 들어간다 | **통과.** resolved count, 단일 최종 Initialize stage, gutter 제외 수, 전체 size 순서를 모두 확인한다. |
| 18 | 앱 screenshot을 algorithm trace의 증거로 대체하거나 요구한다 | **통과.** 앱 surface가 없는 test-only slice라 simulator capture를 범위 밖으로 둔다. |
| 19 | 제한된 Taffy clamp 결과를 공개 CSS 지원 또는 WPT 적합성으로 확대한다 | **통과.** 범위와 제외 항목에서 CSS admission, 전체 WPT, 일반 지원을 분리한다. |
| 20 | 작은 cap 경계가 공식 C11.4 완료 수를 올린다 | **통과.** C11.4/C11 체크와 공식 `72/213`을 유지한다. |

**판정:** 계획 실패 관점 20/20 통과. 구현 전 독립 기대값을 고정했으며, 이 표는 구현/runtime 검토나 PR diff 검토를 대신하지 않는다.
