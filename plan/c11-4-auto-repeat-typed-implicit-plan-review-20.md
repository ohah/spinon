# C11.4 auto-repeat typed implicit tracks 계획 적대 검토 20개

**검토 대상:** `c11-4-auto-repeat-typed-implicit-tracks.md`와 2026-10-20 Chrome 155 사전 비교
**검토 기준:** 구현 전 계획 실패 관점 20개. 구현 결과나 전체 CSS Grid 적합성에 대한 판정은 아니다.
**결과:** 20개 관점 검토 완료. 계획 단계에서 수정한 두 결함은 #1과 #9이며, 나머지는 구현 수용 기준으로 명시했다.

| # | 공격 관점 | 깨질 수 있는 경우와 계획의 통과 기준 | 판정 |
|---:|---|---|---|
| 1 | fixed length 경계값 | 계획이 `0px`를 음수·비유한 값과 함께 거부하면 CSS의 유효한 0-size track을 깨뜨린다. 계획은 `0px` 허용, 음수·비유한 길이 거부로 수정했다. | 수정 완료 |
| 2 | track-size typed 변환 | adapter가 `LengthPx` 외 값을 거부하거나 scalar 경로만 유지하면 percentage·fraction·intrinsic·`minmax()`·`fit-content()`가 auto-repeat 경로에서 다시 실패한다. 모든 기존 `GridTrackSize` 변형을 공용 converter에 연결한다. | 수용 기준 명시 |
| 3 | auto-repeat 사전 검증 | `validate_auto_repeat_auto_track_list`가 고정 px 전용으로 남으면 변환 전에 typed 목록을 계속 거부한다. C11.4만 넓히고 기존 profile 경계는 유지한다. | 수용 기준 명시 |
| 4 | 빈 목록 | 빈 `grid-auto-*` 목록이 Taffy에 빈 pattern으로 전달되면 cycle이 잘못되거나 기본값처럼 보일 수 있다. 입력 검증에서 계속 구체적으로 거부한다. | 수용 기준 명시 |
| 5 | authored cycle 방향 | leading tracks가 list의 첫 값부터 시작하면 explicit 경계에서 역방향 cycle이 틀린다. 마지막 authored 값이 explicit 경계에 오고 앞쪽으로 역순 반복되는지 확인한다. | 수용 기준 명시 |
| 6 | trailing cycle 방향 | trailing tracks가 역순으로 적용되면 explicit 뒤 pattern이 틀린다. 첫 authored 값이 explicit 다음 track에 오고 순방향 반복되는지 trace와 used-size로 확인한다. | 수용 기준 명시 |
| 7 | negative line 기준 | repeat expansion 전후에 `-1` 기준을 혼동하면 leading implicit count가 한 track씩 어긋난다. 5-track repeat에서 `-8/-7`은 leading 2, `-9/-8`은 leading 3임을 앱과 Rust 양쪽에서 확인한다. | 수용 기준 명시 |
| 8 | positive line 기준 | leading implicit tracks가 양수 line 번호를 이동시키면 `8/9` trailing 결과가 달라진다. 기준 fixture의 leading 2, explicit 5, trailing 3 경계를 분리 검증한다. | 수용 기준 명시 |
| 9 | Calc handle 소유권 | adapter의 `grid_math_property`가 auto property를 모르는 상태로 새 변환을 호출하면 panic/unreachable이 나거나 다른 axis 계산식을 쓸 수 있다. 계획에 auto property→기존 axis identity 연결과 auto-list Calc 참조 검증을 추가했다. | 수정 완료 |
| 10 | 잘못 묶인 Calc ID | 존재하지만 다른 node·다른 axis의 Calc ID가 handle map에서 우연히 발견되면 잘못된 식이 실행된다. `validate_css_math_references`가 auto list의 top-level, `minmax`, `fit-content` 참조를 같은 node와 axis에 결합하는지 확인한다. | 수용 기준 명시 |
| 11 | definite percentage basis | rows가 columns 폭으로 resolve되거나 반대로 계산되면 percentage/calc 결과가 달라진다. fixture에서 두 축을 독립 definite size로 두고 axis-specific expected result를 검사한다. | 수용 기준 명시 |
| 12 | indefinite/cyclic percentage | 제한된 definite fixture 결과를 indefinite sizing까지 일반화하면 지원 경계를 과장한다. 이 slice에서는 indefinite/cyclic percentage를 거부하고 범위 밖으로 문서화한다. | 수용 기준 명시 |
| 13 | intrinsic contribution | empty implicit track의 0값을 text-bearing min-content track에 일반화하면 intrinsic sizing이 틀린다. content-bearing item, empty adjacent track, source contribution과 trace를 분리한다. | 수용 기준 명시 |
| 14 | `fit-content()` floor | cap만 적용하고 auto minimum을 보존하지 않으면 min-content floor보다 track이 작아질 수 있다. formula와 item contribution 및 final used track을 따로 검사한다. | 수용 기준 명시 |
| 15 | flex free space | definite area에 positive free space가 없는데 `1fr`를 늘리거나 negative free space를 양수처럼 나누면 잘못된다. 110px fixed fixture에서 base-size 합과 flex outcome을 확인한다. | 수용 기준 명시 |
| 16 | `minmax()` 조합 | CSS에서 허용하지 않는 min/max 조합이나 음수 최소값이 converter에 도달하면 Taffy가 비정상 style을 받는다. computed-style 및 direct layout 입력 검증에서 기존 validator가 거부하는지 확인한다. | 수용 기준 명시 |
| 17 | auto-fit 점유 판정 | leading/trailing implicit item을 auto-repeat 내부 점유로 세거나 hidden/empty item을 무조건 점유로 취급하면 collapse count가 틀린다. repeat 안의 control 점유 1/0과 4/5 collapse를 별도 검사한다. | 수용 기준 명시 |
| 18 | gutter collapse | track 크기만 맞아도 collapsed repeat 주변 gutter가 남으면 item frame과 repeat-count 결과가 달라진다. 132px, 8px gap의 14번째 scene에서 count·gap·frame을 보존하고 브라우저 관찰과 규범 기대를 나눈다. | 수용 기준 명시 |
| 19 | profile 및 실패 경계 | typed list 확장이 C11.2.2·C11.2.3·TemplateGrid 등으로 새면 현재 fail-closed 계약이 바뀐다. 같은 input을 각 profile에 넣어 기존 오류가 유지되는지 확인하고 부분 `LayoutOutput`이 없어야 한다. | 수용 기준 명시 |
| 20 | 한계와 증거 과장 | 138 simulator scenes 통과를 WPT·실기기·hardware GPU·compositor 이후 pixel·전체 C11.4 완료라고 표시하면 증거가 과장된다. AVD, iOS Simulator, PixelCopy/Metal drawable 경계를 나눠 기록하고 roadmap 완료 체크는 유지한다. | 수용 기준 명시 |

## 수정 후 계획 반영

- `0px`는 허용하고 negative/non-finite fixed lengths는 거부한다고 명확히 했다.
- auto track Calc 변환의 axis property mapping 및 참조 무결성 검증을 implementation path에 추가했다.
- fixed-px-only adapter validator, 10,000 track 상한, 기존 profile boundary를 계획의 직접 변경 지점으로 기록했다.
- 14번째 nonzero-gutter auto-fit scene을 기대값과 138-scene runtime matrix에 추가했다.
