# C11.4 행 축 overlapping-span 구현·runtime 실패 관점 검토 20회

**대상:** 고정 Rows scene builder, Android/iOS strict matcher, capture runner와 정적 FFI archive scanner
**판정:** 수정된 최종 source에서 새 20개 관점을 확인했다. 발견한 입력·검증기·증거 경계 문제는 수정 후 해당 관점을 다시 확인했다.

| # | 실패 관점 | 코드·실행 대조와 판정 |
| ---: | --- | --- |
| 1 | 등록되지 않은 비슷한 scene ID가 fixture source로 흘러감 | Rust dispatcher는 네 고정 ID를 exact match하고 알 수 없는 ID에 `None`을 반환한다. focused Rust test가 유사·임의 ID를 거부한다. **통과** |
| 2 | row-axis 입력이 실제로는 column sizing scene이 됨 | root는 고정 Columns 220px, Rows 네 개 `minmax(0,max-content)`다. 네 Android/iOS run 모두 Rows pass를 확인했다. **통과** |
| 3 | 자동 column placement가 겹친 row item을 implicit column으로 이동 | 첫 Android run에서 B가 x=220으로 이동하는 입력 오류를 발견했다. builder를 oracle와 같이 `grid-column:1 / 2`로 수정하고 네 scene의 전체 frame을 양쪽 플랫폼에서 재확인했다. **수정 후 통과** |
| 4 | row line이 잘못되거나 끝 경계를 넘음 | 각 scene의 `grid-row` start/end와 높이를 고정 builder test가 검사한다. runtime matcher는 semantic item별 span contribution과 frame을 함께 검사한다. **통과** |
| 5 | forward/reversed 이름만 바뀌고 실제 DOM 순서는 바뀌지 않음 | Rust test가 item 선언 순서를 검사하고, manifest의 node frames와 contribution item index가 scene별 authored 순서를 반영한다. **통과** |
| 6 | item index를 semantic item과 반대로 연결 | 네 scene의 A/B/C→runtime node mapping을 Android·iOS matcher가 각자 고정하고 전체 frame·contribution 목록에서 대조한다. **통과** |
| 7 | batch 처리 순서가 authored item order에 종속됨 | span-2-only는 `[2]`, 두 span 그룹은 forward/reversed 모두 `[2,3]` Rows stage를 갖는다. Android/iOS 모두 stage sequence와 개수를 검사한다. **통과** |
| 8 | 최종 track 크기만 맞고 중간 intrinsic stage가 틀림 | 두 matcher가 Initialize, span batch, IntrinsicSizingComplete, MaximizeTracks, ExpandFlexibleTracks, StretchAutoTracks의 이름·순서·각 snapshot을 확인한다. **통과** |
| 9 | Columns 대조군이 생략되거나 intrinsic contribution을 잘못 받음 | trace에는 `[Columns, Rows]` 두 pass만 허용한다. Columns의 Initialize/FixedTrackEarlyReturn, 220px track, 빈 contribution 배열을 검사한다. **통과** |
| 10 | CSS track 수가 같아도 내부 gutter 누락·추가가 통과 | Rows stage마다 내부 entry 9개, 교대 index, 네 CSS track과 다섯 gutter를 확인한다. 더 많거나 적은 entry는 실패한다. **통과** |
| 11 | 중간 gutter가 collapsed되거나 경계 gutter가 살아 있음 | 각 snapshot에서 첫·마지막 gutter만 collapsed이고 모든 gutter base/growth size는 0인지 검사한다. **통과** |
| 12 | 값이 맞더라도 growth limit이 잘못된 시점에 닫힘 | 배치 전 null, span-2/3 batch별 finite limit, IntrinsicSizingComplete 이후 최종 limit 및 base size를 따로 비교한다. **통과** |
| 13 | 비유한 trace 값이 tolerance 비교를 우회 | Android JSON number matcher와 Swift generic matcher가 finite 숫자만 허용한다. NaN·Infinity 문자열과 값은 성공하지 않는다. **통과** |
| 14 | JSON boolean이 숫자 0/1로 coercion됨 | Android helper는 Java `Boolean` 타입을 요구하고 iOS helper는 `CFBoolean` identity를 확인한다. 타입이 맞지 않는 flag는 거부된다. **통과** |
| 15 | `null` growth limit을 +∞ 증거로 과장 | 구현은 기대 시점의 JSON null 여부만 비교한다. 문서도 serializer의 null만 기록하며 무한대의 독립 증명으로 확대하지 않는다. **통과** |
| 16 | 일부 node frame 또는 중복 marker가 전체 통과를 만듦 | 양 matcher는 정확히 한 frame marker, 예상 개수·순서·node ID를 요구한다. root와 모든 child 좌표·크기를 검사한다. **통과** |
| 17 | NaN·Infinity 또는 잘린 frame 숫자가 허용오차 비교를 통과 | frame parser는 모든 좌표와 크기를 finite로 확인하고 frame `1/64 CSS px` tolerance를 trace의 `1e-4 CSS px`와 분리한다. **통과** |
| 18 | 미등록 scene 또는 다른 profile 선택자가 신규 scene을 우회 실행 | Android·iOS capture는 미등록 scene과 C11.2.1 profile selector 충돌을 각각 실행하고 argument-error 로그를 검증한다. **통과** |
| 19 | 로그가 잘리거나 이전 process의 결과를 재사용 | runner는 고유 run ID와 process ID를 사용하고 Android trace chunk 집합 및 iOS process-filtered summary/draw 로그를 확인한 뒤 manifest와 원본 로그를 쓴다. **통과** |
| 20 | fixture 코드가 product archive에 포함되거나 scanner가 다른 tree를 증명 | scanner는 `Cargo.toml`·`spec/` marker로 저장소 루트를 찾고 staged/worktree 일치 및 stage 누락 source를 검사한다. product archive는 marker 0/4·ABI/source symbol 없음, opt-in control은 marker 4/4·ABI/source symbol 있음을 보였다. 정적 FFI archive 한정이며 APK/IPA 제외를 주장하지 않는다. **통과** |

## 재검토 근거

- Rust 고정 입력: `exposes_only_fixed_overlapping_row_span_source_order_scenes` 1/1 통과.
- 최종 source Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator 각각 네 scene 전부 통과. unknown-scene/profile-conflict negative route도 양쪽에서 거부됐다.
- 두 simulator debug app build, `cargo fmt --all -- --check`, final capture source/artifact hash 일치 검사는 통과했다.
- 스크린샷은 장면 표시와 배치의 보조 증거다. 화면 전체 pixel equality, Android 실기기/hardware GPU, iOS compositor 이후 표면은 검사하지 않았다.
