# C11.4 anonymous Grid `text-indent` flags 구현 실패 관점 검토 20개

**범위:** 고정 C11.4 anonymous direct-text Grid fixture의 `hanging`·`each-line` 처리, Parley text layout/intrinsic sizing, Stylo 0.22 호환 경계, Android/iOS fixture adapter와 simulator capture runner. 계획 문서 20개 검토를 재사용하지 않았다. 이 검토는 최종 PR diff 30개 검토를 대신하지 않는다.

| # | 실패 관점 | 확인한 경계와 결과 |
| ---: | --- | --- |
| 1 | 기존 호출부의 기본 `text-indent` 동작이 바뀌는가 | `ParagraphStyle::default()`가 길이 0, 두 flag false를 유지한다. 기존 positive 18-scene matrix가 두 플랫폼에서 통과했다. |
| 2 | NaN/무한대 길이가 layout에 들어가는가 | `ParagraphStyle::validate()`가 non-finite `text_indent`를 거부하고 새 line metric 검사도 indent finite 여부를 확인한다. |
| 3 | 고정 profile 밖 길이와 잘못된 keyword 순서를 허용하는가 | parser는 `px`와 `[-10, 10]` 범위, 고정 token 순서만 허용하며 percentage, 중복/역순 keyword, NaN, `-0px`를 거부한다. |
| 4 | 등록되지 않은 scene ID가 heuristic으로 수용되는가 | 50개 고유 ID만 table에 등록한다. 목록 밖 negative scene은 layout publication 전에 거부된다. |
| 5 | computed property와 authored source가 서로 달라도 적용되는가 | 값·flags가 장면의 고정 serialization과 맞지 않으면 거부한다. `-7px`, `-30px`, `%`, 다른 flags와 잘못된 양수값 mutation이 오류 경로를 확인한다. |
| 6 | `each-line`이 soft wrap 뒤에도 잘못 적용되는가 | Parley 0.12.0 `IndentOptions`와 Ahem soft-wrap 20-scene oracle을 대조했다. soft continuation에는 `hanging`에 따른 선택만 반영된다. |
| 7 | `hanging`과 `each-line` 조합이 line 집합을 잘못 뒤집는가 | forced-break 및 soft-wrap 각 4개 flag 조합에서 line별 indent를 별도 기대값과 대조했고 Rust/runtime·simulator 행렬이 통과했다. |
| 8 | indent가 shaped advance에 중복 포함되는가 | Parley `LineMetrics` 계약상 indent는 `advance` 밖이며 정렬 offset에 반영된다. snapshot은 advance와 indent를 분리하고 glyph origin도 검사한다. |
| 9 | min-content가 첫 line만 보거나 indent를 두 번 더하는가 | 각 실제 line의 visible advance에서 inline box 폭을 분리한 뒤 그 line의 `metrics.indent`를 한 번 합산한다. Ahem min-content 대조와 track trace가 통과했다. |
| 10 | max-content가 forced-break 이후 line indent를 놓치는가 | `break_all_lines(None)` 결과의 모든 line을 순회해 각 indent를 합산한다. forced-break max-content 10개 scene이 줄별 값과 track 폭을 통과했다. |
| 11 | 음수 indent 회귀 또는 음수 폭이 생기는가 | 기존 -5/-10px 12개와 18개 positive controls를 함께 Android/iOS에서 30/30 통과했다. intrinsic contribution은 기존처럼 0 아래로 내려가지 않도록 제한한다. |
| 12 | forced break가 row 높이와 root frame을 어긋나게 하는가 | `pre-line` 두 줄의 min/max content 높이는 20px로 기대값을 고정했다. Android/iOS trace와 root frame이 일치한다. |
| 13 | whitespace 처리 중 LF가 사라지거나 soft-wrap와 혼동되는가 | flags forced scene은 `preserve-breaks`/`PreLine`, soft scene은 collapse/`Normal`로 각각 고정하며 원문 text와 line count를 검사한다. |
| 14 | Stylo가 flags를 버릴 때 무관한 `0px`를 authored 값으로 오인하는가 | 리뷰에서 예상 진단이 없어도 fallback이 허용됨을 발견했다. 이제 fallback 인자는 exact single diagnostic gate에서만 true가 되며 false 입력 회귀 검사가 추가됐다. |
| 15 | 호환 진단 예외가 다른 CSS 오류까지 숨기는가 | 진단 수 1, source `inline:<node>`, node ID, 선언 line/UTF-16 column, 정확한 메시지를 모두 확인한다. 추가 진단·다른 source·메시지·위치는 거부한다. |
| 16 | 이전 문서의 computed style이 재사용되는가 | generation, document revision, render-tree revision을 비교한다. trace와 runtime test는 HostDocument revision까지 확인한다. |
| 17 | anonymous item 이외의 tree가 동일 scene ID로 통과하는가 | root tag/ID/profile, direct child 개수·소유권·text bytes, Grid style/tracks를 검증한다. 변형된 직접 자식과 scene mismatch는 실패한다. |
| 18 | 현재 범위 밖 writing mode/profile에 flags가 새어 나가는가 | vertical intrinsic 전용 경로는 flags 기본값만 허용한다. 구현 계획의 LTR·horizontal-tb 외 동작을 완료로 주장하지 않는다. |
| 19 | trace가 layout과 다른 flags를 기록하거나 fixture 코드가 제품 archive에 섞이는가 | trace에 typed flags·line indent를 기록하고 revision을 대조한다. 기존 archive scanner는 product archive 0/20 scene ID·0/2 export, fixture control 20/20·2/2를 확인했다. |
| 20 | 플랫폼 host가 서로 다른 기대값으로 통과하거나 capture가 present 전에 끝나는가 | 동일 Ahem 기대값으로 Android/iOS adapter를 대조했다. 최종 build hash에 묶인 flags 20/20, positive 18/18, negative 포함 30/30을 양쪽에서 실행했고 모두 예상 root의 draw/present, 양수 surface texture를 기록했다. 전체 136개 scene의 superseded retry는 0이다. |

## 수정 후 확인

- Stylo fallback 경계 수정 후 `cargo test --locked -p spinon-runtime --lib --features c11-4-grid-track-sizing-fixture-test text_indent`: 9/9 통과.
- `cargo test --locked -p spinon-text-layout ahem_text_indent_flags_follow_soft_and_forced_break_line_sets`: 1/1 통과.
- 수정 후 최종 Android/iOS fixture app을 다시 빌드하고 여섯 simulator matrix를 재실행했다. 실행 폴더와 build/install hash는 [runtime evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/README.md), [positive evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-runtime-2026-11-01/README.md), [negative evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-runtime-2026-11-01/README.md)에 있다.
- Rust fmt, capture runner syntax, C11.4 Node contract test, `git diff --check`가 통과했다.
- 후속 feature-scoped Clippy에서 새 text-layout test의 range-loop 경고, scene constructor의 인자 수, FFI test의 `FLAG_SCENES` import 누락을 찾았다. iterator와 typed indent structs로 정리하고 import를 추가한 뒤 text-layout/runtime/FFI Clippy `-D warnings`가 모두 통과했다. 영향을 받은 focused test는 각각 1/1, 9/9, 2/2로 다시 통과했고 최종 여섯 simulator matrix와 Android/iOS app build도 새 hash로 재검증했다.

## 남은 경계

이 검토와 simulator draw/present는 glyph pixel 동등성, Android 실기기/hardware GPU, iOS compositor 이후 화면, 일반 CSS admission, full WPT 또는 C11/C11.4 완료를 증명하지 않는다. 구현 PR 전 30개 서로 다른 diff 실패 관점 검토와 PR의 required checks/final merge 확인이 남아 있다.
