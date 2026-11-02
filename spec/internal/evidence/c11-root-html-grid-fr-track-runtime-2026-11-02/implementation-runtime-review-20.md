# C11.1 Root HTML `fr` 구현·runtime 실패 관점 검토 20개

**검토 대상:** `RuntimeRootHtmlTextPaintV1` typed Grid projection, fraction-axis validation, parsed Root HTML runtime tests, Taffy `find_size_of_fr` accumulator change.

**검증 근거:** runtime C11.1 Root HTML regression 10/10, style Grid projection tests 47/47, vendored Taffy library tests 176/176, scoped Clippy, rustfmt와 `git diff --check` 통과. Taffy test build는 변경하지 않은 `compute/grid/mod.rs`의 기존 unused warning 6개를 출력했다.

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 변경이 일반 Grid fixture profile까지 새 fraction track을 열 수 있음 | **PASS** — Root HTML validator와 fraction-axis gate는 `RuntimeRootHtmlTextPaintV1` 분기 안에서만 호출된다. 다른 `RuntimeBlockFlexGrid*` profile은 변경하지 않았다. |
| 2 | 음수·NaN·무한 fraction이 Taffy까지 전달될 수 있음 | **PASS** — Root HTML direct-track validation은 유한한 non-negative px/fr만 허용한다. projection tests가 음수·양/음 무한·NaN과 non-direct shape를 확인한다. |
| 3 | `minmax(..., fr)`가 direct `fr`로 잘못 인정될 수 있음 | **PASS** — direct variant만 매치한다. 실제 Root HTML CSS cascade에서 `minmax(0px,1fr)`는 `grid-template-columns` 오류로 fail-closed되고 scene은 없다. |
| 4 | percentage 또는 auto width가 column flex basis로 쓰일 수 있음 | **PASS** — column `fr`는 같은 computed style의 유한 non-negative `LengthPx` width만 받는다. runtime negative matrix가 percentage와 indefinite axis를 확인한다. |
| 5 | `calc()` width가 computed `LengthPx`로 정규화된 뒤 허용될 수 있음 | **PASS** — `layout_math_values`의 width/height key도 확인한다. `calc()` width negative control은 scene을 만들지 않는다. |
| 6 | `min-width`가 track 최소 크기를 바꿔 flex 비율을 왜곡할 수 있음 | **PASS** — 해당 축의 computed min-size는 `auto`여야 한다. `min-width:0` runtime negative control은 property `min-width`로 거부된다. |
| 7 | `max-width`가 flex track 결과를 clamp할 수 있음 | **PASS** — 해당 축의 max-size는 `none`이어야 한다. `max-width` override negative control은 실패하고 partial scene이 없다. |
| 8 | `aspect-ratio`가 반대 축으로 container 크기를 결정할 수 있음 | **PASS** — fraction 어느 축이든 aspect ratio가 설정되면 거부한다. 현재 cascade 경계가 내는 `aspect-ratio` 오류도 테스트에서 구분한다. |
| 9 | columns와 rows의 basis 검사가 서로 바뀌거나 한쪽을 놓칠 수 있음 | **PASS** — columns는 width/min-width/max-width, rows는 height/min-height/max-height를 각각 검사한다. fixed row fraction positive와 indefinite row negative를 실행했다. |
| 10 | 명시 px track이 fraction 활성화 때 손상되거나 순서가 바뀔 수 있음 | **PASS** — 혼합 `30px 1fr 2fr` projection/runtime case가 track 순서와 시작 좌표를 확인한다. |
| 11 | `grid-template-areas` 경로가 fraction으로 넓어질 수 있음 | **PASS** — 기존 named-area validation은 고정 px track만 유지한다. 120×70 기존 area 형태에 fraction을 넣은 runtime negative control은 `grid-template-areas`로 거부된다. |
| 12 | gap이 flex free space에서 빠지지 않거나 두 번 차감될 수 있음 | **PASS** — fixed px gap은 기존 projection을 재사용한다. `0 10px`, `10px 0` positive geometry와 root/container boundary를 검사한다. |
| 13 | factor 합이 1보다 작을 때 표준상 1로 취급되지 않을 수 있음 | **PASS** — Taffy 0.25+0.25 test가 120px에서 각각 30px track을 확인하고 runtime marker origins가 25px/10px gap 조건을 확인한다. |
| 14 | `0fr`를 생략하거나 다음 item을 잘못 배치할 수 있음 | **PASS** — zero factor track geometry와 다음 item의 x=0 위치를 확인한다. 0px item은 paint 목록에서 생략될 수 있어 computed background에 해당하는 layout frame으로 별도 확인한다. |
| 15 | f32 factor 합산이 `1e38fr` 세 factor의 유효 비율도 망가뜨릴 수 있음 | **PASS** — f32에서 유한한 3-factor 합과 `[40,40,40]`을 Taffy 단위 및 Root HTML runtime에서 확인했다. |
| 16 | 네 개 `1e38fr` 합산 overflow가 zero tracks로 전파될 수 있음 | **PASS** — 누산과 divisor만 f64로 올리고 결과를 layout f32 경계로 변환한다. Taffy 및 Root HTML runtime은 각각 `[30,30,30,30]`을 확인한다. Chrome 156의 0px 관측은 명세 불일치로 별도 기록되어 있고 기대값으로 사용하지 않는다. |
| 17 | 큰 수 수정이 일반 ratio·subunit·zero factor까지 회귀시킬 수 있음 | **PASS** — Taffy unit cases와 전체 176개 library tests가 통과했다. Taffy test 컴파일은 변경하지 않은 모듈 warning 6개를 보였다. |
| 18 | 검증이 class 이름·NodeId·DOM 순서에 묶일 수 있음 | **PASS** — positive control은 semantic class를 바꾸고 source order를 역전하며, computed fraction tracks와 marker origin을 검사한다. production layout은 색/ID로 scene을 고르지 않는다. |
| 19 | 고정 marker 크기가 예상 flex track 크기를 강제로 만들 수 있음 | **PASS** — runtime marker는 의도한 track보다 작게 설정한다. defininte root boundary, gap, 뒤 marker의 누적 start 좌표와 마지막 남는 공간으로 used sizing을 판정한다. 이는 Chrome auto-stretch item의 전체 frame parity 주장이 아니다. |
| 20 | 실패한 입력이 partial frame 또는 이전 revision 결과를 노출할 수 있음 | **PASS** — 각 negative control은 `completed == None`을 확인하고, positive source test는 style/layout key 일치를 확인한다. 검사는 기본 parsed Root HTML entry를 사용한다. |

Android/iOS simulator도 같은 source hash로 실행해 `status=0`, `layout=ready`, 5 boxes와 draw/present 로그 및 화면을 확인했다. 화면은 C04.12 native demo surface scaling을 포함하므로 CSS-pixel geometry의 대체 근거가 아니다. PixelCopy/Metal drawable readback, Android hardware renderer/GPU, 전체 CSS Grid 또는 C11 완료를 대신하지 않는다.
