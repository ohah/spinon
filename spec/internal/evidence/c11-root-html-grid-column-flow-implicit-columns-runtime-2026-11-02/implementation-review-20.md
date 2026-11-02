# Root HTML column-flow implicit columns 구현/runtime 실패 관점 20개

**대상:** branch `feat/c11-root-html-grid-column-flow-implicit-columns-20261102`의 고정 px implicit-column 구현 후보. 계획 검토 20개와 별도로 새로 살폈다. 구현 source fingerprint는 [source-revision.json](source-revision.json)의 `fdb020def0805d984ded56fbc9b84cc5cc279c4679a913ca6c80c4290515e862`다. 검토는 코드·기존 회귀 검사·이번 실행 로그를 대조했으며 아래에 별도 실행하지 않은 동작을 통과했다고 주장하지 않는다.

| # | 실패 관점 | 검토 결과와 근거 |
|---:|---|---|
| 1 | 새 track 허용이 Root HTML column-flow 밖의 profile에도 적용되는가 | cascade list 허용 flag와 typed projection은 Root HTML profile에 한정된다. layout expansion은 `RootHtmlColumnFlowV1` mode에서만 켜진다. **문제 없음.** |
| 2 | row-flow가 fixed `grid-auto-columns`를 새로 받는가 | column-flow 전용 cascade admission과 projection gate가 row-flow scalar를 거부한다. `root_html_column_flow_rejects_unsupported_auto_columns_and_row_flow_sizing` 확인. **문제 없음.** |
| 3 | explicit cells만으로 충분한데 `auto` implicit sizing이 차단되는가 | scalar/list/`auto`가 허용되고 auto track expansion은 필요할 때만 요구된다. `root_html_column_flow_keeps_implicit_sizing_when_explicit_cells_are_sufficient` 확인. **문제 없음.** |
| 4 | `auto`인데 item이 explicit cell을 넘을 때 빈 implicit track이 생성되는가 | 후보 검증은 필요한 column 수가 explicit count보다 크면 `auto`를 거부한다. Layout regression `root_html_column_flow_rejects_implicit_column_overflow_before_placement` 및 runtime 거부 행렬이 확인한다. **문제 없음.** |
| 5 | px scalar가 implicit column마다 동일하게 적용되는가 | scalar를 각 추가 column breadth로 확장한다. `root_html_column_flow_expands_fixed_px_scalar_for_each_implicit_column`이 위치를 확인한다. **문제 없음.** |
| 6 | px list가 첫 값만 반복되거나 index가 어긋나는가 | 비어 있지 않은 목록을 authored order대로 modulo 순환하며 normalized expansion과 다시 비교한다. `root_html_column_flow_repeats_fixed_px_implicit_column_list`가 두 번째 항목과 반복을 확인한다. **문제 없음.** |
| 7 | 빈 list에서 `% 0` 또는 숨은 default가 발생하는가 | cascade/projection/layout 각 경계에서 empty list를 거부하며, layout test가 property별 오류를 확인한다. **문제 없음.** |
| 8 | 10,001개 auto-column 입력이 확장 전 할당되는가 | projection과 layout validation이 `GRID_MAX_TRACKS`를 먼저 확인한다. layout 검사 `root_html_column_flow_rejects_invalid_auto_column_tracks`가 10,001개 목록을 거부한다. **문제 없음.** |
| 9 | explicit와 implicit column 합계가 10,000 cap을 넘는가 | required count를 배치 전에 계산하고 column 축의 공통 quota에 비교한다. `root_html_column_flow_rejects_required_implicit_columns_over_track_limit`가 첫 초과값을 확인한다. **문제 없음.** |
| 10 | NaN, infinity, 음수 px가 정상 폭으로 들어가는가 | finite/nonnegative 검사를 cascade output, style projection, layout contract에서 각각 적용한다. projection의 synthetic values와 layout error 유형을 확인한다. **문제 없음.** |
| 11 | `%`, `fr`, `minmax()`, `fit-content()` 또는 아직 계산 안 된 `calc()`가 px list로 오인되는가 | typed value admission은 direct finite px만 받는다. percentage-dependent `calc()`는 cascade 검사에서 `Unsupported`로 남고 projection에서 거부된다. **문제 없음.** |
| 12 | 순수 length `calc()`가 계산 가능한데 거부되거나 계산 중 NaN이 폭으로 들어가는가 | cascade는 `calc(10px + 5px)`를 `15px`로 정규화하고, 같은 profile의 test가 실제 computed value를 확인한다. 비유한 합성 typed value는 projection/layout gate에서 차단한다. **문제 없음.** |
| 13 | 행 수가 0일 때 required-column 나눗셈이 panic 나는가 | profile validator가 empty explicit rows를 거부한 뒤에만 `div_ceil(rows.len())`을 호출한다. layout validator도 동일하게 empty rows를 먼저 거부한다. **문제 없음.** |
| 14 | hidden children이 implicit column 수를 늘리거나 source-order 슬롯을 소비하는가 | root projection은 visible item owner를 세고 placement input은 hidden map을 사용한다. `c11_1_root_html_sparse_column_flow_ignores_hidden_items_without_using_extra_cells` 확인. **문제 없음.** |
| 15 | sparse column flow가 row 밖 implicit row를 조용히 생성하는가 | 필요한 column만 계산하고 `expanded_rows`는 거부하며 각 item row 위치가 explicit row 범위인지 재검증한다. **문제 없음.** |
| 16 | dense flow, authored placement, line/span, repeat, line name, areas가 새 mode에 섞이는가 | candidate validator가 flow, placement shorthand, template metadata와 repeat/area 형태를 fail-closed 처리한다. projection의 dense/repeat 거부 회귀가 통과한다. **문제 없음.** |
| 17 | item 순서·크기·DOM shape나 writing direction이 계산과 다르게 배치되는가 | 기존 direct empty `div`, fixed-size, `order:0`, LTR/horizontal-tb 제약을 유지한다. runtime unsupported-shape 행렬과 document-order control을 확인했다. **문제 없음.** |
| 18 | row/column gap이 두 축에 중복 적용되거나 누락되는가 | Taffy track sizing의 축별 gutter 경계를 유지한다. layout `root_html_column_flow_applies_each_gap_once_on_its_own_axis`와 runtime의 12개 frame 비교가 통과했다. **문제 없음.** |
| 19 | auto-column 예약 실패가 cascade/layout 일부를 publish하거나 다음 호출을 오염시키는가 | cascade 및 expanded-column reservation failpoint 각각에서 구체 오류와 동일 worker/engine 복구를 검사한다. Scene test는 complete snapshot만 노출하며 Android/iOS log도 최종 `layout=ready`를 확인한다. **문제 없음.** |
| 20 | normalized column 수가 독립 계산과 다르거나 screenshot이 geometry oracle로 오인되는가 | Rust runtime은 root와 12 item frame을 직접 비교한다. Chrome 156은 독립 기대값과 맞고 simulator는 marker·draw/present 및 시각 배열만 근거로 남긴다. PixelCopy/Metal readback이라고 확대 해석하지 않았다. **문제 없음.** |

이번 검토에서 blocking defect는 찾지 못했다. 전체 style-to-layout suite는 308/309이며 무관한 C07 aspect-ratio computed-string 기대값 차이 1건이 있다. 전체 suite green으로 보고하지 않는다. 실행 근거와 raw logs는 [runtime README](README.md)에 있다.
