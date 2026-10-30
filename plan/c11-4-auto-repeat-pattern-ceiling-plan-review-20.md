# C11.4 auto-repeat 두-track 상한 계획 실패 관점 검토 20개

**검토 대상:** [계획](./c11-4-auto-repeat-pattern-ceiling.md), [구현 전 비교 기준](../spec/internal/evidence/c11-4-auto-repeat-pattern-ceiling-precomparison-2026-10-30/README.md), `GRID_MAX_TRACKS`와 Taffy `0.15.0` auto-repeat source-order 경계. 계획 구현 전 입력·계산·named-line·증거·상태 범위를 서로 다른 관점으로 확인했다.

| # | 실패 관점 | 판정 및 방어 경계 |
| ---: | --- | --- |
| 1 | implicit-grid cap 규칙을 explicit auto-repeat cap의 규범 oracle로 오해한다 | **통과.** 계획과 precomparison은 §5.4가 implicit grid에 관한 규칙임을 구분하고, explicit track 10,000 cap을 고정 Taffy 내부 경계로만 시험한다. |
| 2 | 가용 크기에서 prefix 또는 suffix의 크기를 빼지 않아 반복 횟수가 하나 어긋난다 | **통과.** 두 fixed neighbor 합 5px와 반복당 3px를 명시해 15,002/15,005px가 각각 4,999/5,000회가 되도록 고정한다. |
| 3 | exact-fit case가 cap에 닿지 않아 source-order suffix 보존을 검사하지 못한다 | **통과.** 15,002px은 prefix + 9,998 repeated tracks + suffix로 정확히 10,000개를 만든다. |
| 4 | overflow case가 전체 pattern 개수만 보아 중간 절단을 놓친다 | **통과.** 15,005px은 반복 10,000 tracks를 요청하지만 prefix 뒤 9,999개만 남아 마지막 `[1px, 2px]`에서 `1px`만 유지된다. |
| 5 | track 크기 순서를 검사하지 않아 pattern이 잘못된 subtrack에서 잘린다 | **통과.** 전체 Initialize track sequence, prefix, 반복 pattern, suffix 유무를 확인한다. |
| 6 | prefix/suffix를 생략해 cap의 source-order 효과를 놓친다 | **통과.** 두 neighbor를 포함한다. exact-fit은 suffix를 보존하고 cap 초과는 suffix를 버린다. |
| 7 | repeat metadata의 line group 개수가 pattern track 수와 맞지 않는다 | **통과.** 두 repeated tracks에는 앞·중간·끝의 세 line-name group을 지정한다. |
| 8 | pattern 시작 이름을 반복 바깥 line에 중복해 경계 occurrence가 두 번 센다 | **통과.** CSS 입력과 `GridAutoRepeat.line_names`는 반복 pattern 내부의 세 group만 사용하며 외부 line name은 비운다. |
| 9 | 음수 named-line occurrence가 explicit grid 끝에서 센다는 기준을 놓친다 | **통과.** §8.3 의미와 Taffy `-1` 배치를 사용하고 고정 기대 frame을 비교한다. |
| 10 | 중간 이름이 마지막 partial pattern 뒤에 남지 않아 named item이 이전 track으로 이동한다 | **통과.** cap 초과 case에서 `pattern-start -1 / pattern-middle -1` child가 마지막 보존된 `1px` track에 있어야 한다. |
| 11 | exact-fit에서 마지막 완전 pattern의 named line을 cap case 좌표와 혼동한다 | **통과.** exact-fit과 overflow에 서로 다른 offset `14,996px`·`14,999px`를 지정한다. |
| 12 | 빈 track을 collapse하는 auto-fit을 써서 repeat 계산과 명명 경계를 혼합한다 | **통과.** auto-fill만 사용하고 기존 auto-fit 행렬과 범위를 분리한다. |
| 13 | child의 intrinsic/minimum size가 fixed tracks 또는 auto-repeat 개수를 바꾼다 | **통과.** item은 두 축 모두 고정 1px이고, 사용 track은 고정 1/2px이며 gap은 0이다. |
| 14 | Columns와 Rows에서 root basis 또는 trace 축을 뒤바꾼다 | **통과.** 각 실행에서 target 축에만 15,002/15,005px basis를 주고 다른 축은 고정 1px로 둔다. |
| 15 | border, padding 또는 viewport 변환이 content basis를 바꾼다 | **통과.** 고정 root와 viewport를 같은 target basis로 두고 border/padding/gap은 0이다. |
| 16 | 입력 하나에서 두 축 모두 10,000 tracks로 만들어 trace 예산을 과도하게 쓴다 | **통과.** 최대 크기 축을 한 번에 하나만 실행하고 100,000 captured-state 상한을 단언한다. |
| 17 | resolved explicit count만 검사해 stage trace가 다른 track 수를 갖는다 | **통과.** `used_*.explicit_track_count`, 마지막 sizing pass의 단일 Initialize stage, gutter 제외 track 수를 함께 비교한다. |
| 18 | 화면 screenshot이 없는 테스트의 증거 공백을 시각적으로 메울 수 있다고 가정한다 | **통과.** 여기서는 앱 화면이 무관하며 exact Rust trace와 item frame을 직접 검사한다고 한계를 기록한다. |
| 19 | test-only 결과를 앱 runtime, CSS admission 또는 WPT 완료로 확대한다 | **통과.** 구현·증거·공식 상태에서 앱·WPT·일반 author CSS 지원을 제외한다. |
| 20 | 세부 경계 통과를 C11.4/C11 완료나 전체 완료 수 증가로 기록한다 | **통과.** C11.4/C11 체크와 `72/213`을 유지한다고 명시한다. |

**판정:** 계획 실패 관점 20/20 통과. 검토 중 §5.4가 explicit auto-repeat cap의 근거가 아님을 더 분명히 적고 중복된 pattern 시작 이름 표기를 제거한 뒤 관점 1·8을 다시 확인했다. 이 검토는 계획의 경계·기대값 검토이며 구현/runtime 검토나 계획 PR diff 검토를 대신하지 않는다.
