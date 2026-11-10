# C11.4 three-fit row-span 구현·런타임 검토 20관점

검토일: 2026-11-10
검토 범위: 세 fit track의 six-member row-span admission, automatic minimum, typed style projection, trace rounding, calc authored-value preservation 및 RuntimeSession 반례·복구 경로.

각 항목은 서로 다른 입력 경계나 불변 조건을 확인한다. 구현 전 계획 검토 20관점과 계획 PR diff 30관점은 별도 자료이며 이 목록에 합산하지 않았다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | fit track 수와 나머지 intrinsic track 수가 바뀌는 입력 | 정확히 3 fit + min-content 1 + max-content 1 + auto 1만 허용한다. four-fit·중복 tail은 거부한다. |
| 2 | 나머지 세 track의 authored 순서가 특정 순서만 허용되는지 | 여섯 tail 순열을 모두 허용하며 typed style/layout predicate가 같은 조합을 검증한다. |
| 3 | fit position이 1-based 행 번호와 다른 인덱스로 해석되는지 | 위치 1–6의 20개 오름차순 triple을 검사하고 중복·역순·0·7을 거부한다. |
| 4 | Style projection과 layout의 admission 집합이 달라지는지 | 두 계층 모두 six-member list, tail multiplicity, three-fit 위치 조건을 동일하게 제한한다. |
| 5 | px와 percentage cap의 typed value가 혼동되는지 | direct px 및 percentage cap을 220px definite basis에서 전체 비교한다. authored 위치별 cap assignment도 확인한다. |
| 6 | 음수·비유한 cap이 intrinsic floor로 유입되는지 | layout 단위 테스트에서 음수 px, 무한 px, NaN percentage를 거부한다. |
| 7 | Stylo가 `calc(0px)`를 px로 단순화해 calc 금지 규칙을 우회하는지 | winning specified declaration의 `FitContent(Calc)`를 보존하고 `calc(0px)` 및 `calc(10px + 10%)`를 RuntimeSession에서 거부한다. |
| 8 | percentage cap이 indefinite basis에서 허용되는지 | definite 220px height를 고정하고 `height:auto` 반례를 거부한다. |
| 9 | 새 span predicate가 three-fit auto-placement까지 넓히는지 | layout/style predicate 단위 테스트와 RuntimeSession mutation이 three-fit auto-placement을 닫힌 상태로 확인한다. |
| 10 | 임의의 row-span topology나 일곱 번째 implicit row가 통과하는지 | 지정된 span-2 다섯 item topology와 span-2/span-3 네 item topology만 matrix에 포함하며 임의 span mutation과 추가 implicit row를 거부한다. |
| 11 | column placement, flow, alignment 또는 overflow 변형이 profile에 섞이는지 | fixed first column·sparse row flow·start/stretch·visible overflow를 검사하고 column mutation 및 scrollable overflow를 거부한다. |
| 12 | 추가 Grid owner가 trace를 다른 Grid에 결합하는지 | 한 parsed Grid owner와 한 layout trace를 요구하며 candidate가 한 owner로 제한됨을 확인한다. |
| 13 | explicit empty row control이 auto minimum 적용 대상이 되는지 | 첫 explicit row item은 `min-height:0`을 유지하고 auto minimum 대상에서 제외한다. |
| 14 | visible text item 일부에만 auto minimum이 적용되는지 | 전 text item의 uniform mode를 요구하고 dark media query로 한 item만 바꾼 반례를 atomic rejection한다. |
| 15 | unsupported child, text grammar 또는 writing context가 제한 profile에 진입하는지 | 직접 text payload·고정 font/line-height·지원 writing context를 요구하고 비ASCII payload mutation을 거부한다. |
| 16 | span item이 실제 두 행 또는 지원되는 네 item topology를 벗어나는지 | 모든 item placement, source order, line count와 span geometry를 oracle에 대조한다. |
| 17 | stale/missing revision이나 다른 owner의 track trace가 결과에 붙는지 | layout/cascade key, trace node, item input owner, revision 일치를 직접 검증한다. |
| 18 | automatic minimum이 텍스트 내용 floor보다 낮아지는지 | eligible span의 traced automatic minimum을 line count×10px와 비교하고 최종 spanned row 합이 floor 이상인지 확인한다. |
| 19 | 1/64px 잔여 분배가 세 fit track에서 잘못된 행으로 가는지 | 3-fit auto-minimum에 `UpLaterTrack`을 적용하고 전체 11,520 scene의 Chrome row values와 0.01px 허용오차로 비교한다. |
| 20 | DOM 순서 반전이나 `align-content:stretch`가 unrelated track geometry를 바꾸는지 | 5,760 source-order pair의 행/item frame 동등성과 5,760 start/stretch pair에서 auto row 외 행 불변을 matrix가 검사한다. |

## 실행 근거

- 11,520 scene RuntimeSession 대 Chrome 비교: `parsed_root_html_three_fit_row_span_matches_all_11520_chrome_scenes` 통과, test 686.80초.
- one-fit row-span 5,760 scene: `parsed_root_html_one_fit_row_span_auto_minimum_matches_all_2880_chrome_pairs` 통과.
- one-fit later positions 4–6: `parsed_root_html_later_fit_content_positions_four_through_six_match_all_4320_chrome_scenes` 통과.
- two-fit row-span 3,840 scene: `parsed_root_html_two_fit_content_row_spans_match_all_3840_chrome_scenes` 통과.
- 기존 repeated-text row-span 960 scene 회귀: 오래된 `rows`/`rowValues` 필드와 unused D/E CSS 원문 비교를 보정한 뒤 `parsed_root_html_repeated_text_row_spans_match_all_960_chrome_scenes` 통과.
- atomic rejection, same-session recovery, caps/owner/automatic-minimum trace: `three_fit_auto_minimum/rejections.rs`, `trace.rs`의 focused tests 통과.
- two-fit `min-content` floor control과 four-fit mutation rejection/same-session recovery 통과. 기존 rejection이 새로 허용된 three-fit row-span을 거부하던 문제를 four-fit 입력으로 교체했다.
- Android API 37.0 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator light/dark visual smoke 통과. 정확한 fixture SHA, `layout=ready`, draw/present, OS color-scheme 전환에 따른 `dark=true` 및 environment revision 증가가 앱 로그와 캡처에 연결되어 있다.
- `cargo fmt --all -- --check`와 `git diff --check` 통과. Layout/style/style-to-layout Clippy는 `-D warnings` 통과. Runtime-only Clippy는 기존 두 helper의 `too_many_arguments` lint만 명령에서 제외해 통과했다. 여러 package를 합친 Clippy 명령은 변경하지 않은 `spinon-style` test helper에서 `fixture_with_child_style`와 `Fixture.child`가 정의되지 않은 compile 오류로 실패했다.

이 검토는 고정 profile의 구현 범위에만 해당하며 일반 CSS Grid 지원, WPT 전체 적합성, 실기기 GPU parity 또는 C11.4/C11 완료 판정이 아니다.
