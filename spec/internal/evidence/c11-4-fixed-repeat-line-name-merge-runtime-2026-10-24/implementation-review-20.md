# C11.4 fixed-repeat named-line merge · 구현 실패 관점 검토 20개

**대상:** `computed_auto_repeat_tracks`의 fixed-repeat name expansion/budgeting, cascade/layout named placement, opt-in Android/iOS scene adapter와 artifact boundary.
**한계:** 구현 후보에 대한 자체 20관점 검토다. PR diff review 30개와 GitHub CI/merge 검증을 대신하지 않는다.

| # | 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | parser의 outer `line_names` 길이가 `values + 1`이 아니어 index가 어긋남 | 함수 진입에서 checked `values.len() + 1` 비교로 fail-closed한다. 이후 expansion도 각 outer group과 마지막 group의 존재를 확인한다. |
| 2 | auto-repeat이 없거나 두 개여서 repeat 경계를 하나로 정할 수 없음 | 정확히 하나의 `auto-fill`/`auto-fit`만 수용한다. 단일 auto-repeat과 이전 회귀 16/16을 확인했다. |
| 3 | fixed repeat count가 0, 음수 또는 `usize` 변환 불가 | positive integer만 저장하고 그 외는 `UnsupportedComputedGridValue`로 거부한다. 기존 invalid-count 회귀를 유지한다. |
| 4 | fixed pattern이 빈 값, 2 track 초과 또는 line group 수 불일치 | 1–2개의 track과 `track_sizes + 1` name groups만 허용한다. 모든 track은 유한·0 이상 computed `LengthPx`로 제한한다. |
| 5 | repeat count × track 수나 aggregate track 수가 overflow/10,000 초과 | checked multiplication/addition 및 기존 `MAX_COMPUTED_GRID_TRACKS` filter로 값 생성 전에 거부한다. 기존 boundary tests를 유지한다. |
| 6 | fixed repeat 내부 이름이 iteration마다 빠지거나 순서가 뒤바뀜 | 각 iteration의 inner groups를 authored order로 append한다. cascade assertion은 2-track pattern과 다중 iteration 결과를 exact 비교한다. |
| 7 | 반복 끝과 다음 iteration 시작의 같은 physical line을 두 줄로 취급 | iteration 끝 group을 두 번 증가시키지 않아 다음 leading group과 같은 slot에 합친다. layout의 seam placement geometry가 통과했다. |
| 8 | outer names와 repeat의 leading/trailing names가 seam에서 누락 | repeat 전 outer group, repeat inner groups, repeat 뒤 outer group을 같은 output group에 authored order로 합친다. fixed/ordinary, fixed/fixed assertion이 이를 고정한다. |
| 9 | 동일한 name token을 seam에서 미리 deduplicate해 occurrence를 바꿈 | metadata에는 원래 token과 중복을 보존한다. layout test는 duplicate `join` group을 보존하고 named occurrence는 physical line 기준으로 확인한다. |
| 10 | auto-repeat inner names가 outer line vector에 잘못 확장됨 | auto-repeat의 `N+1` inner groups는 별도 `ComputedCssGridAutoRepeat.line_names`로 복사하고 output outer vector에는 component 전후 경계만 둔다. |
| 11 | auto-repeat 전후 fixed repeat이나 ordinary track의 경계가 밀림 | columns의 fixed/auto/fixed seam 및 ordinary/auto seam expected groups와 named placement frame을 확인했다. |
| 12 | 양의 named integer occurrence가 잘못된 seam을 선택 | `start`, `join`, `seam`, `mid`, `tail`, `repeat-end` placement의 고정 expected rectangles가 style-to-layout 4/4에서 통과했다. |
| 13 | 음수 occurrence가 explicit grid 끝을 잘못 기준으로 삼음 | 음수 `seam` occurrence와 음수 row-axis `tail` placement를 고정 geometry와 비교했다. |
| 14 | named span 검색이 duplicate 또는 repeat 경계에서 잘못 셈 | `span 2 mid` expected frame 및 Chrome 사전 비교의 physical-line occurrence validator를 확인했다. |
| 15 | `auto-fit`이 named placement보다 먼저 empty track을 접음 | item을 repeat-end named line으로 배치한 뒤 남은 5개 repeated tracks의 collapse flags와 item frame을 Android/iOS에서 확인했다. |
| 16 | row 축이 column 전용 로직을 쓰거나 pixel samples가 축을 잘못 판정 | row scene은 40×300 root와 y=96 item을 검사한다. 양 플랫폼 fixture adapter와 네 경계 표본도 row coordinate를 사용한다. |
| 17 | 1 MiB metadata 계산에서 group header, String header 또는 UTF-8 bytes를 누락 | outer/auto group vector headers, 모든 `String` headers와 각 반복 token의 UTF-8 bytes를 checked arithmetic으로 합산한다. 정확히 1 MiB 통과, 1 byte 초과 거부 test가 통과했다. |
| 18 | metadata budget 계산 또는 repeated-name 곱셈이 integer overflow | checked multiplication/addition 실패는 node/property 문맥의 unsupported-value 오류가 된다. 10,000-track 상한과 별도 적용한다. |
| 19 | 중첩 Vec/String 할당 실패에서 panic, 부분 snapshot 또는 이후 호출 오염 | outer group, inner names, string text 각 예약을 fallible하게 하고 injection 지점 5개에서 contextual allocation error 후 같은 fixture의 다음 cascade 복구를 확인했다. vendored Taffy의 내부 infallible allocation에 대한 process-wide OOM 주장은 하지 않는다. |
| 20 | fixture가 기본 제품 artifact에 들어가거나 제한 fixture를 공개 CSS 지원으로 표시 | default release archive marker 0/3·ABI/Rust runtime symbol 없음, fixture-test positive control marker 3/3·ABI/Rust symbol 있음이다. 신규 진입은 기존 opt-in profile이며 공개 admission·C11 완료 상태는 추가하지 않는다. |

## 실행에서 수정해 재검증한 항목

- 최초 iOS simulator capture는 Metal 표본 feature flag가 꺼져 있어 실패했다. flag를 켜고 앱을 다시 빌드한 뒤 iOS scene 3/3, Metal 표본 12/12 exact로 통과했다. 제품 코드 문제로 분류하지 않았다.
- 이 evidence 디렉터리의 focused-check runner는 첫 작성에서 repository root 탐색을 한 단계 벗어나 실패했다. root 인덱스를 수정하고 같은 runner를 재실행해 Rust·fixture-boundary·format 검사를 모두 통과했다. 이 수정은 검증 도구에만 해당한다.
