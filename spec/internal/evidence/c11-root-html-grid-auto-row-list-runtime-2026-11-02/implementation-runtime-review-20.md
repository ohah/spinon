# 구현 코드·런타임 실패 관점 검토 20개

계획 review를 재사용하지 않고, `1d838ba44bd635bb5c572dc950b9313f1863434a` 위의 실제 구현과 latest Android/iOS Simulator 산출물을 별도 실패 경계로 점검했다.

| # | 확인한 실패 경로 | 판정과 근거 |
|---:|---|---|
| 1 | cascade 허용값이 모든 Runtime profile로 확장되는가 | 새 option은 `RuntimeRootHtmlTextPaintV1`에서만 켜진다. `element.rs`의 profile gate와 cascade tests를 확인했다. 통과. |
| 2 | 새 `grid-auto-rows` 규칙이 `grid-auto-columns`에도 적용되는가 | Root HTML `grid-auto-columns:12px 18px`는 `Unsupported`를 유지한다. `c11_1_root_html_auto_row_list.rs`의 positive row/negative column assertion 통과. |
| 3 | 기존 scalar px `grid-auto-rows` 계산이 목록 표현으로 바뀌는가 | scalar는 기존 `LengthPx` variant와 repeat 경로를 유지한다. Root HTML scalar one/zero/multiple regression과 runtime matrix 통과. |
| 4 | auto-track cascade가 list order를 바꾸거나 필드를 누락하는가 | computed list는 각 track을 원 순서대로 보존하고 projection에서 확인된다. cascade 3/3, projection 11/11. |
| 5 | 빈 list가 유효한 pattern이 되는가 | cascade/projection 및 layout admission에서 empty list를 거부한다. projection invalid-list matrix와 layout validation을 확인했다. 통과. |
| 6 | percentage·fraction·intrinsic track이 px 목록으로 잘못 투영되는가 | cascade `track_size(..., false)` 결과 중 `LengthPx`만 복사하고 projection도 px-only를 검사한다. mixed `%`, `fr`, `min-content` rejection 통과. |
| 7 | 음수·NaN·infinity가 projection 경계를 통과하는가 | list 원소의 finite·non-negative 검증을 확인했다. projection matrix에 음수·NaN·infinity를 넣어 모두 거부했다. |
| 8 | 추가 implicit row가 없어 layout core가 음수·비유한 list를 받아들이는가 | 최초 검토에서 발견했다. `validate_root_html_implicit_rows_contract`에서 확장 여부와 무관하게 값을 확인하도록 수정했다. no-implicit-row direct `LayoutInput` test가 음수·NaN·infinity를 모두 거부한다. |
| 9 | invalid list가 row allocation 뒤에야 검증되는가 | Root HTML contract validation이 expansion 전에 값을 확인한다. invalid/no-row test와 resource-limit test 통과. |
| 10 | 첫 implicit row가 explicit row list의 첫 값으로 대체되는가 | explicit row 20px 다음 y=25px row가 pattern 첫 15px track을 사용한다. runtime exact frame test 통과. |
| 11 | pattern이 한 번만 적용되고 두 번째 이후 row에 반복되지 않는가 | 네 DOM row가 `[15,25,15]px`로 확장되고 네 번째 item row의 frame도 확인한다. projection/runtime tests 통과. |
| 12 | explicit row 개수가 pattern phase를 소비하는가 | explicit 20px row는 유지되고 첫 implicit row가 pattern index 0으로 시작한다. layout·runtime geometry가 일치한다. |
| 13 | 0px pattern entry가 invalid 처리되거나 gap을 없애는가 | 0px implicit track은 크기 0으로 두고 5px row gap은 유지한다. `root_html_fixed_px_implicit_row_list_accepts_zero_and_cycles_after_it` 통과. |
| 14 | 다중 열 sparse placement에서 row starts/column offsets가 바뀌는가 | 각 두 item row의 x=`0,45` 및 row y=`25,45,75`를 독립 기대값과 대조했다. 통과. |
| 15 | 비기본 alignment 또는 named area가 목록 admission으로 우회되는가 | list+`align-items:start`는 cascade에서 `cascade_failed`; named-area list는 projection에서 거부된다. 관련 runtime/projection negative test 통과. |
| 16 | implicit row 또는 auto-row list limit을 넘길 때 확장된 Vec가 먼저 만들어지는가 | 10,001 implicit rows는 `GridImplicitTrackRequired`, 10,001개 auto-row list는 projection 및 layout admission에서 각각 cap으로 거부한다. 두 no-implicit-row/list-cap tests와 expansion cap test 통과. |
| 17 | computed track list reserve 실패가 property/error context를 잃는가 | cascade `AutoRows` failpoint는 `GridAllocationFailure { property: "grid-auto-rows" }`를 반환하고 partial snapshot을 발행하지 않는다. |
| 18 | layout `ExpandedRows` reservation 실패 뒤 같은 worker가 손상되는가 | injected failpoint에서 error를 반환하고 같은 worker 재시도에서 정상 row list로 복구한다. |
| 19 | negative runtime input이 stale/partial scene을 남기는가 | unsupported list, flow, area, placement, text와 alignment 입력은 failed state 및 `completed=None`을 유지한다. Root HTML runtime 20/20. |
| 20 | simulator 검증이 opt-in fixture scene이나 성공하지 않은 probe를 통과시키는가 | 최신 앱은 default `index.html`, C04 runtime GPU 외 fixture flags off로 실행했다. corrected HostDocument probe, `layout=ready boxes=11`, 11-box draw/present 확인 뒤 앱을 종료했다. Simulator screenshots는 pixel readback이 아니다. |

**판정:** 최초 검토에서 발견한 no-implicit-row invalid-list validation 공백을 수정하고 Rust·Android·iOS 검증을 최신 코드로 재실행했다. 이 검토 범위에서 미해결 결함은 없다. GPU surface pixel equality, 실기기, hardware GPU, iOS compositor 이후 pixels, WPT 및 일반 CSS Grid admission은 범위 밖이다.
