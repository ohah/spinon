# C11.3.3 template-area typed auto track 계획 실패 관점 검토 20개

**검토 대상:** [`plan/c11-3-3-template-area-auto-track-sizing.md`](../../../plan/c11-3-3-template-area-auto-track-sizing.md)

**검토 기준:** CSS Grid Layout Module Level 1 2025-03-26 CRD §§7–8, 11; Chrome 156 고정 precomparison; 기존 C11.3.3 area track-list cycling/auto sizing과 C11.4 TrackSizing 계약.

**결과:** PASS 20 · PARTIAL 0 · FAIL 0. 구현 전 계획 검토이며 코드/runtime/PR 검토를 대신하지 않는다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | area가 없는데 typed `grid-auto-*`가 template profile 전체에 열린다 | PASS — projection과 layout 모두 area-backed `TemplateGrid`/`TemplateGridImplicitTracks`로 범위를 한정하고 no-area negative control을 둔다. |
| 2 | area matrix가 authored rows보다 작을 때 기존 trailing explicit track이 삭제된다 | PASS — authored track 보존을 계약에 두고 long authored list 회귀를 요구한다. |
| 3 | area matrix가 explicit tracks보다 클 때 누락 rows만큼 잘못 생성된다 | PASS — area dimension과 authored count 차이로 missing 개수를 독립 산출하고 양 축을 검사한다. |
| 4 | columns/rows 속성 또는 dimension이 서로 바뀐다 | PASS — axis별 Chrome used track 및 item frame, Rust projection 테스트를 완료 조건에 둔다. |
| 5 | pattern list 반복이 한 번의 extension 뒤 종료된다 | PASS — missing count에 대한 순환과 fixed-cycle 관찰을 비교한다. |
| 6 | area 생성 뒤 다음 positive implicit line이 이미 소비한 pattern entry를 재사용한다 | PASS — consumed count만큼 phase 이동 후 named-line 확장을 검증한다. |
| 7 | intrinsic list 허용으로 signed leading/negative track 배치가 의도치 않게 열린다 | PASS — signed mode를 explicit 제외 범위와 negative control로 고정한다. |
| 8 | numeric positive line placement가 named-area implicit mode를 우회해 지원된다 | PASS — independent numeric implicit placement를 명시적으로 제외한다. |
| 9 | empty track list가 modulo-by-zero 또는 무한 반복을 유발한다 | PASS — 빈 목록을 projection/layout 양쪽에서 거부하고 별도 negative control을 둔다. |
| 10 | unsupported enum 값이 typed track으로 잘못 변환된다 | PASS — 기존 `project_tracks`/`validate_tracks` 경로 재사용과 unsupported-value 거부를 요구한다. |
| 11 | `NaN`, infinity, 음수 fraction/percentage가 sizing 입력에 들어간다 | PASS — malformed component negative controls와 기존 validation 재사용을 명시한다. |
| 12 | CSS `calc()`의 length/percentage basis가 사라지거나 다른 축에 적용된다 | PASS — 기존 checked CSS math projector와 definite basis를 양 축에서 확인한다. |
| 13 | percentage sizing을 indefinite basis에서 확정값으로 오판한다 | PASS — definite basis test와 invalid/missing basis rejection을 계획에 둔다. |
| 14 | min-content/max-content contribution이 fixed-size 경로에서 사라진다 | PASS — intrinsic item contribution을 이용해 track 결과와 regression을 확인한다. |
| 15 | `fit-content()` screenshot에서 min-content가 limit보다 커 cap 효과를 검증하지 못한다 | PASS — 현재 fixture의 한계를 명시하고 min-content ≤ limit < max-content 입력을 새로 요구한다. |
| 16 | root의 leftover space 또는 explicit row를 잘못 셈해 fr 예상값이 달라진다 | PASS — 90px root, 10px explicit row, 두 implicit rows, gap 0에서 40px씩 독립 계산한다. |
| 17 | area-generated implicit track 뒤 metadata line-group count가 track count와 어긋난다 | PASS — 기존 `N+1` line metadata reservation과 track quota/error 경계를 regression 대상으로 둔다. |
| 18 | track quota, arithmetic overflow, allocation 오류 뒤 partial frames가 남는다 | PASS — 기존 checked/fallible path와 no-partial-output을 negative gate로 고정한다. |
| 19 | Chrome 결과만 복제해 normative sizing과 fixture capture의 불일치가 숨겨진다 | PASS — CSS 계산을 판정 기준으로 두고 Chrome은 독립 교차 확인으로 한정한다. |
| 20 | AVD/iOS Simulator 근거가 device, hardware GPU, compositor 이후 pixel, C11 완료로 과장된다 | PASS — simulator renderer/readback 한계, WPT 제외, C11 및 공식 `72/213` 유지가 계획에 적혀 있다. |

이 계획은 구현을 시작하기 전의 검토 기록이다. 계획 단계의 PASS는 코드 동작, simulator 결과, WPT 적합성 또는 PR diff 검토의 PASS가 아니다.

## Cascade 경계 발견 후 계획 보완 재검토

첫 실제 CSS 통합 입력에서 `RuntimeBlockFlexGridTemplateV1`이 `minmax()`를 adapter에 보내기 전에 `Unsupported`로 분류했다. 구현 전 계획 검토는 style projection/runtime validation만 보고 Stylo computed extraction gate를 빠뜨렸다. 계획의 1번 구현 단계를 cascade gate까지 확장하고, 아래에서 서로 다른 20개 경계를 새로 공격했다. 이 재검토는 첫 표와 구현/runtime·PR diff 검토를 대체하지 않는다.

**결과:** PASS 20 · PARTIAL 0 · FAIL 0. 이 보완 표는 남은 구현 범위 전에 갱신한 계획을 검토하며 이미 실행된 code test 결과를 계획 PASS로 세지 않는다.

| # | 새 독립 실패 관점 | 계획 보완 판정 |
|---:|---|---|
| 1 | template profile에서 area가 없는데 cascade가 C11.4 parser를 먼저 연다 | PASS — `template_areas.is_some()`와 template profile 조건을 cascade 및 projection에 함께 요구한다. |
| 2 | invalid `grid-template-areas`가 cascade fallback 뒤에도 typed auto sizing을 남긴다 | PASS — 이미 cascade된 유효 area metadata만 허용 신호로 사용하고 invalid declaration은 기존 fallback을 유지한다. |
| 3 | cascade의 typed parser 적용으로 명시 `grid-template-rows/columns` intrinsic grammar가 열린다 | PASS — 추가 parser flag는 auto-track 변환에만 전달하고 명시 template track parse options는 바꾸지 않는다. |
| 4 | 비-area template profile의 multi-value px list 지원까지 사라진다 | PASS — 기존 `allow_fixed_track_lists` 경로는 유지하고 typed parser만 area 조건에 추가한다. |
| 5 | area profile의 단일 `1fr`이 기존 single-length special case로 거부된다 | PASS — typed parser 선택은 list 길이보다 area/profile 조건이 우선한다. |
| 6 | mixed fixed/intrinsic list에서 일부만 변환되고 순서가 달라진다 | PASS — 전체 list에 기존 `track_size(..., true)`를 순서대로 적용하며 unsupported 항목은 전체 값을 거부한다. |
| 7 | cascade computed `calc()`가 이전 revision math node를 가리킨다 | PASS — 같은 `CssMathProjector` revision에 연결하고 style-to-layout integration gate에서 계산 결과를 검사한다. |
| 8 | row/column이 서로 다른 cascade 허용 flag를 사용한다 | PASS — 같은 area/profile 조건으로 auto rows와 columns 변환 양쪽에 flag를 전달한다. |
| 9 | `grid-template-areas:none` 또는 초기값이 synthetic area로 취급된다 | PASS — 초기 area는 `None`이며 computed metadata의 실제 존재만 신호로 쓴다. |
| 10 | all-null area matrix가 영역이 있다고 오판되어 feature gate를 넓힌다 | PASS — dimensions와 computed `Some` 상태를 구분하고, all-null dimensions도 같은 narrow area-backed contract 안에서 검사한다. |
| 11 | area extension 뒤 `minmax` pattern이 동일한 track value identity를 잃는다 | PASS — computed track 값을 clone/copy하는 기존 typed converter와 `extend_template_area_tracks`를 그대로 연결한다. |
| 12 | mixed area/auto-repeat 경로에 의도치 않은 typed tracks가 허용된다 | PASS — template profile의 area 확장 조건과 auto-repeat profile을 분리하고 auto-repeat+area 조합은 제외한다. |
| 13 | CSS cascade unsupported 값을 adapter error로 오인해 자동 fallback이 사라진다 | PASS — Stylo의 기존 computed Unsupported/fallback 동작을 보존하고, 새 parser는 이미 computed된 area 경로에만 쓴다. |
| 14 | allocation failpoint이 new typed list branch에서 우회된다 | PASS — existing `reserve_computed_tracks`/reservation site를 재사용하고 fallible reservation을 완료 gate에 둔다. |
| 15 | parser `allow_intrinsic_tracks`를 template 전체에 true로 넘겨 auto-repeat과 explicit tracks를 바꾼다 | PASS — 별도 `allow_template_area_track_sizing` boolean만 auto row/column conversion에 전달한다. |
| 16 | non-area typed list가 area 조건에서 불변인 negative control로 검증되지 않는다 | PASS — cascade fixture에는 area 없는 `15px 1fr` 거부 대조군이 남아 있다. |
| 17 | author stylesheet와 inline style의 cascade order가 area gate와 auto-track value를 다르게 만든다 | PASS — 고정 CSS/runtime fixture는 inline style을 사용하고 Stylo snapshot으로 실제 computed pair를 같이 검사한다. |
| 18 | zoom 또는 computed percentage normalization이 25% basis를 변경한다 | PASS — definite root width/height와 axis별 computed value, used track geometry를 Chrome 및 runtime에서 분리 비교한다. |
| 19 | new parser가 auto-track list 길이·기존 10,000-track quota를 우회한다 | PASS — 기존 `MAX_COMPUTED_GRID_TRACKS`, reservation 및 layout `GRID_MAX_TRACKS` gate를 그대로 유지한다. |
| 20 | cascade PASS를 app runtime이나 공개 CSS 지원 완료로 과장한다 | PASS — Stylo, projection, layout, simulator를 각각 검증하며 WPT, public support, C11 completion은 제외한다. |

이 재검토에서 발견한 source gate는 계획에 반영했다. 이후 구현/runtime 검토와 PR diff 검토는 독립된 새 표로 남긴다.
