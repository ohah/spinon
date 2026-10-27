# C11.4 §11.7 flexible track expansion 구현 · 실패 관점 검토 20개

**대상:** crates/spinon-layout/src/grid_track_sizing_tests/flex_factor_bound.rs의 새 Rust test와 spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/expected.json.

이 검토는 test-only 구현 경계, 입력 변환, rational 기대값 판독, trace 선택과 미지원 범위를 서로 다른 20개 관점에서 확인했다. 앱 runtime이나 simulator 실행 검토가 아니다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 포함된 expected JSON 경로가 crate source 상대 위치와 다름 | **통과.** compile-time include가 repo의 checked-in oracle JSON을 가리키고 실제 focused test가 파싱했다. |
| 2 | JSON의 다른 stage 값을 읽고도 통과 | **통과.** top-level stage가 ExpandFlexibleTracks와 정확히 일치해야 한다. |
| 3 | case 누락·추가·중복이 무시됨 | **통과.** 고정 4 ID 각각을 찾고 map 개수와 원본 배열 개수를 비교해 ID 집합과 유일성을 강제한다. |
| 4 | case ID와 축 mapping이 뒤바뀜 | **통과.** weighted/subunit은 Columns, frozen/zero-fr은 Rows로 기대 axis를 고정한다. |
| 5 | available size가 definite 200px이 아님 | **통과.** 각 JSON input을 숫자로 읽어 200px을 강제하고 그 값을 root와 viewport에 넣는다. |
| 6 | 반대 축이 indefinite이거나 root/viewport와 다름 | **통과.** 120px을 강제해 두 root dimension과 viewport dimension에 동일하게 설정한다. |
| 7 | 반대 축 gap이 대상 축 free space에 섞임 | **통과.** LayoutGap 기본값에서 시작해 axis에 해당하는 한쪽 property만 10px로 설정한다. |
| 8 | JSON의 CSS descriptor와 typed track이 불일치 | **통과.** base와 factor에서 canonical descriptor를 만들고 JSON css 문자열과 비교한 뒤 같은 값으로 typed track을 생성한다. |
| 9 | fixed 20px track을 flexible minimum으로 잘못 생성 | **통과.** factor null은 GridTrackSize::LengthPx, factor 값은 LengthPx/Fraction MinMax로 생성한다. |
| 10 | 0fr에 minimum base가 적용되지 않음 | **통과.** 0 factor와 24px base가 같은 typed MinMax input에 들어가고 trace base는 독립 기대 배열과 대조한다. |
| 11 | factor 합계 1 미만 입력이 별도 case에서 사라짐 | **통과.** subunit case의 0.5 factor와 fixed neighbor가 checked-in input에 남고 [85,20] 기대값을 검사한다. |
| 12 | rational 문자열을 float decimal로 오해하거나 분모가 0/음수 | **통과.** integer와 numerator/denominator 형식만 파싱하고 분모가 양수인지 확인한다. |
| 13 | NaN·무한대 또는 f32 변환 불가 값이 허용 | **통과.** 변환 값과 실제/기대 base 모두 finite인지 확인한다. |
| 14 | 오차 기준이 CSS px가 아니거나 적용되지 않음 | **통과.** 모든 비교는 CSS px absolute difference 1e-4 이하를 강제한다. |
| 15 | item contribution 또는 spanning이 stage 값을 바꿈 | **통과.** 입력에 root 한 node만 있고 grid_items가 비어 있음을 assert한다. |
| 16 | 다른 grid root의 trace를 검사 | **통과.** LayoutOutput에 trace 하나만 허용하고 owner NodeId를 root 1로 확인한다. |
| 17 | Rows/Columns pass 누락 또는 반복을 무시 | **통과.** 기대 axis pass가 있어야 하고 두 번째 동일 축 pass는 없어야 한다. |
| 18 | expansion snapshot을 앞/뒤 stage와 혼동하거나 중복 stage 허용 | **통과.** 고유 ExpandFlexibleTracks를 MaximizeTracks 뒤, StretchAutoTracks 앞에서 선택한다. |
| 19 | gutter 제거로 track 순서·gap·boundary sentinel 오류를 숨김 | **통과.** 전체 interleaved 배열 길이와 gutter flag 순서, collapsed boundary, finite base, 실제 internal gap 값을 검사한다. CSS track은 gutter flag로 분리해 순서대로 기대 배열과 비교한다. |
| 20 | test-only snapshot을 production/runtime 지원 또는 C11.4 완료로 확대 | **통과.** 변경은 기존 test 파일에 한정되고 growth-limit field, simulator, app, WPT, 일반 CSS admission은 판정하지 않는다. 공식 72/213도 유지한다. |

**판정:** 위 관점에서 구현 결함을 찾지 못했다. focused Rust test 1/1, cargo fmt check, package all-targets Clippy 결과는 같은 evidence directory의 실행 로그와 README에 기록한다.
