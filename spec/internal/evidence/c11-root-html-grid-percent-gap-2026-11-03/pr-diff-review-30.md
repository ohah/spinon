# C11.1 percentage gap PR 변경분 적대 검토 30개

검토 대상은 `origin/main...9450daaf` 전체 diff다. 계획 검토 20개와 구현/runtime 검토 20개의 표를 재사용하지 않고, 이번 PR에서 바뀐 Rust 모듈 경계·호출 순서·adapter·fixture 연결을 별도 관점으로 확인했다.

| # | 이번 변경에서 확인한 실패 경로·불변 조건 | 검토 결과 |
|---:|---|---|
| 1 | 새 `root_html_gap` 파일이 모듈 선언에서 빠져 실제 빌드에 연결되지 않음 | `grid.rs`의 private `mod root_html_gap` 및 projection 모듈 등록을 확인했다. Clippy와 focused test 로그에 새 모듈이 포함된다. 통과. |
| 2 | percentage resolver가 알 수 없는 property를 column 기준으로 오인 | resolver의 property 분기는 `row-gap`/`column-gap`만 허용하고 그 외에는 `InvalidGridTrack`을 반환한다. 통과. |
| 3 | resolver가 row percentage에 width basis를 연결 | `row-gap` 분기는 `style.height`와 vertical min/max를 묶어 선택한다. projection 호출도 height를 넘긴다. 통과. |
| 4 | resolver가 column percentage에 height basis를 연결 | `column-gap` 분기는 `style.width`와 horizontal min/max를 묶어 선택한다. projection 호출도 width를 넘긴다. 통과. |
| 5 | 새 helper 추출로 기존 fixed px gap이 percentage basis 검사를 받거나 값이 바뀜 | `LengthPx` 분기는 유한·비음수만 확인하고 원래 값을 반환한다. 기존 px named-area control과 implicit-row 회귀가 유지된다. 통과. |
| 6 | percentage gap에서 `Auto` 또는 비유한 dimension이 definite basis인 것처럼 취급 | 직접 layout resolver는 `LayoutDimension::Fixed`와 finite non-negative를 요구한다. projection helper도 `LengthPx`만 resolve한다. 통과. |
| 7 | percentage gap에 min/max가 함께 있을 때 preflight와 direct layout 간 허용 범위가 달라짐 | computed projection은 `layout_dimensions` min/max를 확인하고 direct resolver는 대응 `LayoutStyle` constraint를 확인한다. 각각 fail-closed한다. 통과. |
| 8 | typed math metadata가 일반 percentage 값으로 낮아져 admission을 통과 | `validate_root_html_grid_box_inputs`가 `layout_math_values`를 gap resolver보다 먼저 검사한다. 기존 typed `calc()` negative scene은 partial layout 없이 실패한다. 통과. |
| 9 | negative, NaN, infinity, `auto`, `unsupported` gap이 resolver match의 wildcard로 허용 | projection과 direct resolver 모두 유효한 px/percentage 또는 `normal`만 받으며 나머지는 오류다. 통과. |
| 10 | 큰 ratio와 basis 곱을 f32로 먼저 계산해 overflow가 무한대나 잘못된 유한 값으로 숨음 | percentage와 basis를 f64로 곱한 다음 유한성 및 f32 최대값을 확인한다. helper 경계 테스트 2/2가 통과했다. |
| 11 | zero-width definite basis에서 positive percentage가 reject되거나 0이 아닌 값으로 변환 | 0은 유효한 fixed basis이며 곱 결과 `Some(0.0)`/`0.0`을 보존한다. zero-basis geometry test가 통과했다. |
| 12 | 1보다 큰 ratio를 1로 cap하는 별도 변환이 생김 | resolver는 입력 ratio를 직접 basis에 곱한다. `120%` fixture의 두 번째 column 원점은 130px다. 통과. |
| 13 | 명시적 column 수보다 많은 visible item을 implicit row extent에서 누락 | direct grid validator는 visible direct child 수를 explicit column 수로 올림 나눗셈하고 explicit row 수와 비교해 큰 값을 사용한다. 통과. |
| 14 | malformed zero-column container가 `div_ceil(0)`에서 panic | `validate_root_html_implicit_rows_contract`가 Grid validation 초기에 empty columns를 거부한 뒤에만 새 required-row 계산이 도달 가능하다. 통과. |
| 15 | `display:none` child가 새 required-row 계산에서 gutter를 늘림 | `hidden` map을 만든 뒤 hidden grid를 건너뛰고 visible direct child만 센다. 별도 hidden-child regression이 layout focused log에서 통과했다. |
| 16 | explicit rows가 item 수보다 많을 때 마지막 explicit track 또는 그 사이 gutter를 extent에서 제외 | required rows는 explicit row count 이상이고 row extent는 `required_rows - 1` gutter와 모든 explicit row track을 합산한다. 통과. |
| 17 | implicit row의 반복 auto-row list가 첫 항목에서만 확장되거나 index가 explicit row 수부터 시작 | implicit extent loop의 index는 implicit 시작점을 0으로 두고 `tracks[index % tracks.len()]`으로 순환한다. 기존 cycling test와 percentage runtime test가 통과했다. |
| 18 | `grid-auto-rows: length`가 새 TrackList 변환에서 누락 | 단일 px auto-row를 `GridTrackSize::LengthPx`로 변환하고 동일한 extent 검사에 전달한다. 기존 fixed-row 회귀가 유지된다. 통과. |
| 19 | explicit percentage row/column track의 aggregate 검사가 gap 변경 뒤 잘못된 축 basis를 씀 | `validate_axis_track_range`는 columns에 width, rows에 height를 선택하고 percentage gap도 aggregate에 포함한다. projection focused test가 통과했다. |
| 20 | percentage gap만 있고 percentage track은 없는 축에서 aggregate 검사를 건너뜀 | early return 조건이 percentage track과 percentage gap을 모두 검사한다. gap-only fixed-track scene에서도 basis와 합산 경로가 실행된다. 통과. |
| 21 | gutter 개수가 track 수와 달리 계산되어 끝에 불필요한 gap 하나를 붙임 | explicit track list는 `len - 1`; implicit-row/column은 실제 required count에서 `saturating_sub(1)`을 사용한다. 고정 geometry scene이 이 경계를 확인한다. 통과. |
| 22 | 큰 gutter 개수 곱에서 usize underflow 또는 중간 f32 overflow | count 감소는 saturating subtraction이고 곱·누적은 f64로 수행한 뒤 매 단계 finite f32 extent를 검증한다. direct overflow negative tests가 통과했다. |
| 23 | column-flow sparse placement가 percentage column gap을 resolver 이전에 px-only로 거부 | `validate_root_html_column_flow_container_style`와 sparse expanded-column 검사 모두 공용 resolver를 사용한다. direct column-flow percentage test가 통과했다. |
| 24 | column-flow required column 수가 row count와 배치 방향을 반대로 사용 | column-flow는 visible item을 explicit row count로 올림 나눗셈하고 기존 explicit column 수보다 작은 경우를 막는다. explicit/implicit column geometry가 통과했다. |
| 25 | relative implicit column extent helper 이동 중 기존 call site가 구 모듈을 계속 참조하거나 gap 계산을 생략 | 유일한 projection call site가 `root_html_grid_gap::validate_implicit_column_relative_extent`로 이동했고 함수는 resolved percentage gap을 합산한다. 통과. |
| 26 | fraction auto-column의 기존 bounded extent 처리가 새 percentage-gap 확장에서 고정 길이로 오인 | fraction branch는 기존처럼 content-basis 잔여 분배 영역을 고정 총합에 더하지 않는다. fraction factor 유효성은 기존 validator가 별도로 유지한다. 통과. |
| 27 | named-area candidate gate가 layout projection과 다른 unit 또는 axis를 씀 | named-area 경로는 computed row height/column width를 공용 resolver에 넘긴다. spanning header와 lower/right item geometry의 percentage scene이 통과했다. |
| 28 | named-area에서 신규 px/percentage admission이 unsupported gap 형태까지 넓어짐 | named-area는 resolver가 성공한 유한·비음수 px/percentage/normal만 받으며 기존 unsupported placement와 track gate는 그대로다. 기존 px control 및 unsupported negative scene이 통과했다. |
| 29 | helper 이동/추가가 unrelated Grid/Flex, 공개 API, FFI 또는 Cargo feature로 새어 나감 | 전체 diff에서 변경은 layout/style projection, 테스트, 계획 및 evidence에 한정된다. public export, FFI symbol, feature, Flex dispatch 변경은 없다. 통과. |
| 30 | 계획·실행 자료가 서로 다른 implementation commit 또는 임시 bootstrap 입력을 가리킴 | simulator source revision은 `f92352d9` 구현 commit, fixture/probe hash, 두 artifact hash를 기록하고 복원된 bootstrap hash 목록도 연결한다. README는 시각 smoke와 pixel readback 한계를 구분한다. 통과. |

검토 결론: PR 변경 범위에서 추가 수정이 필요한 차단 결함을 찾지 못했다. focused 검증과 시뮬레이터 근거는 [실행 README](./README.md)에 있고, broad package suites는 실패 항목을 그대로 명시한다. C11.1/C11 및 공식 완료 `72/213`은 이 변경으로 완료 처리하지 않는다.
