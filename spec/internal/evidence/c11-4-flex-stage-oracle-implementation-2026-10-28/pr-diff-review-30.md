# C11.4 §11.7 flexible track expansion 구현 PR · 변경분 검토 30개

**대상:** Rust test-only 변경, 계획·상태 요약, 실행 로그와 구현 evidence. 계획 20개 및 구현 20개 검토와 별도로 최종 PR diff의 범위·입력 연결·보고 경계를 확인한다.

| # | PR diff 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 제품 알고리즘이나 adapter가 의도치 않게 바뀜 | **통과.** Rust 변경은 grid_track_sizing_tests/flex_factor_bound.rs의 test-only 영역이다. |
| 2 | Cargo manifest/lock 또는 dependency가 불필요하게 바뀜 | **통과.** 이미 쓰는 serde_json dev dependency를 재사용하고 manifest 변경은 없다. |
| 3 | 공개 API·FFI·runtime 동작이 바뀜 | **통과.** 제품 코드, ABI, FFI allowlist와 runtime 경로는 diff에 없다. |
| 4 | implementation이 계획된 단일 stage 외 algorithm을 주장 | **통과.** 구현은 definite empty-grid ExpandFlexibleTracks stage의 최종 base size만 비교한다. |
| 5 | 계획 PR #716의 순서 조건을 놓침 | **통과.** #716은 구현 PR 전 Rebase and merge됐고 현재 main base에 포함되어 있다. |
| 6 | test가 Chrome screenshot을 Rust expected로 사용 | **통과.** Rust 기대값은 CSS Grid 2025 CRD 독립 Fraction JSON이며 Chrome 관측은 control로 분리한다. |
| 7 | JSON input 수정으로 다른 stage나 case 집합이 조용히 통과 | **통과.** stage 이름, exact 네 ID, 고유성, 케이스별 축과 고정 sizing inputs를 검사한다. |
| 8 | target axis가 Rows/Columns 반대로 생성 | **통과.** 각 고정 case ID의 기대 축을 별도로 검사한 뒤 해당 축에 track list를 놓는다. |
| 9 | available size가 definite root/viewport와 다름 | **통과.** 200px과 반대축 120px을 강제하고 root와 viewport 두 dimension을 동일하게 맞춘다. |
| 10 | 반대 축 gap이 free-space 산술에 더해짐 | **통과.** 기본 zero gap에서 시작해 target axis property만 10px로 설정한다. |
| 11 | JSON track descriptor가 실제 typed GridTrackSize와 다름 | **통과.** base/factor로 canonical CSS descriptor를 재구성해 JSON 표시와 맞춘 뒤 같은 값으로 typed input을 만든다. |
| 12 | fixed 20px neighbor가 flexible track으로 변환 | **통과.** null factor는 LengthPx, factor는 MinMax LengthPx/Fraction으로 구분한다. |
| 13 | zero-fr minimum/base floor가 입력에서 사라짐 | **통과.** 0 factor와 24px base를 JSON descriptor로 구성하고 해당 CSS track trace를 기대값과 비교한다. |
| 14 | sub-unit 0.5fr 사례가 weighted 사례와 혼동 | **통과.** subunit-columns의 고정 ID와 axis, input descriptor, 독립 기대 배열이 별도다. |
| 15 | 기대 track 배열의 길이 또는 순서가 달라도 통과 | **통과.** 각 입력에 두 CSS track과 두 rational 기대값을 강제하고 trace 순서대로 비교한다. |
| 16 | 정수/유리수 형식의 일부가 부정확하게 처리 | **통과.** JSON integer와 integer 또는 numerator/denominator 문자열을 처리한다. |
| 17 | 0/음수 denominator 또는 f32 overflow가 수용 | **통과.** denominator 양수와 변환 결과 finite 조건을 검사한다. |
| 18 | tolerance가 적용되지 않거나 수치 차원을 잘못 사용 | **통과.** CSS px의 finite absolute difference 1e-4 이하를 모든 base 비교에 적용한다. |
| 19 | item contribution/span이 빈-grid oracle에 섞임 | **통과.** root node 하나와 빈 grid_items를 강제한다. |
| 20 | trace가 다른 grid node 소유 | **통과.** output trace 하나만 허용하고 root ID를 검사한다. |
| 21 | 대상 axis pass 누락 또는 중복 | **통과.** 해당 axis pass가 하나여야 한다. |
| 22 | 대상 expansion stage가 중복되거나 누락 | **통과.** ExpandFlexibleTracks stage를 유일하게 찾고 요구 순서도 검사한다. |
| 23 | Maximize/Stretch와 다른 시점의 snapshot을 비교 | **통과.** MaximizeTracks < ExpandFlexibleTracks < StretchAutoTracks를 확인한다. |
| 24 | boundary gutter가 CSS track으로 세어지거나 사라짐 | **통과.** 두 CSS track, internal gap, 양 boundary entry를 포함한 interleaved 길이와 flags를 검사한다. |
| 25 | collapsed sentinel이 모든 gutter 또는 CSS track에 잘못 적용 | **통과.** 첫/마지막 gutter만 collapsed이고 CSS track은 collapsed가 아닌지 확인한다. |
| 26 | internal gap 크기나 대상 축 연결이 달라짐 | **통과.** 유일한 non-boundary gutter base를 target-axis gap 10px와 비교한다. |
| 27 | gutter를 거른 뒤 track identity/order를 바꿈 | **통과.** trace flags로 track을 추리고 DOM/spec 순서대로 rational 배열과 대조한다. |
| 28 | 다른 trace field를 normative CSS contract로 확대 | **통과.** growth_limit, infinitely_growable, chosen fr fraction과 leftover는 expected assertion으로 주장하지 않는다. |
| 29 | green status를 full suite, simulator, CI 통과로 과장 | **통과.** focused test와 fmt/Clippy 로그만 보고하고 simulator, app runtime, WPT와 CI 상태는 별도/미검증으로 둔다. |
| 30 | roadmap에서 C11.4 또는 공식 완료 수가 잘못 바뀜 | **통과.** checkbox는 미완료고 공식 집계 72/213을 유지하며 계획·구현 evidence 경로를 연결한다. |

**판정:** 변경 범위, 오라클 입력, trace gate, local validation과 status 표현에서 추가 수정이 필요한 결함을 발견하지 못했다. 이 검토는 PR diff에 대한 기록이며 CI 또는 simulator 검증을 대신하지 않는다.
