# C11.4 explicit `align-content:stretch` 구현/runtime 실패 경로 검토 20개

검토 대상은 candidate diff, Chrome 157 precomparison fixture/expected 값, default `RuntimeSession` 실행, opt-in track trace, Android API 37 AVD와 iOS 26.2 Simulator 원본 로그 및 restoration record다. 각 항목은 별도의 오류 경계를 본다.

| # | 독립 실패 경로 | 검토 및 판정 |
|---:|---|---|
| 1 | 규범 값 선택 | CSS Grid §11.8의 `normal`·`stretch`만 허용하는 대상과 일치한다. `normal` control 및 `start` control을 Chrome 157에서 함께 고정했다. |
| 2 | authored shorthand가 scope 밖으로 새는 경우 | projection은 `align-content`만 새로 허용하고, `place-content`가 바꾸는 `justify-content`는 기존 초기값 gate에 남긴다. Shorthand 전체 지원을 주장하지 않는다. |
| 3 | computed `align-content`가 아닌 authored text를 판정하는 경우 | admission은 Stylo computed property map의 exact `normal|stretch`를 쓴다. 직접 parser token 비교를 추가하지 않는다. |
| 4 | candidate grid와 value gate가 어긋나는 경우 | exact `RootHtmlAutoRowStretchV1` candidate가 alignment/value 조건을 같이 확인한다. layout mode 선택도 같은 candidate set으로 제한한다. |
| 5 | projection 뒤 공통 alignment gate가 재거부하는 경우 | `reject_grid_alignment` 및 Root HTML scope gate 양쪽에 candidate-only 예외를 전달한다. Grid container/property/value/profile 네 조건을 모두 묶었다. |
| 6 | grid가 아닌 element에 `stretch`가 허용되는 경우 | 예외 인자는 `root_html_auto_row_stretch_nodes` membership에서만 설정된다. 다른 node의 기존 default rejection 조건은 변하지 않는다. |
| 7 | unsupported container alignment가 통과하는 경우 | `start`, `center`, `end`, `space-between`, `space-around`, `space-evenly` RuntimeSession mutation을 각각 실행해 layout과 completed snapshot 모두 없는 것을 확인했다. |
| 8 | other alignment property가 같이 풀리는 경우 | `align-items`, `justify-content`, `justify-items`는 여전히 `normal`을 요구한다. `place-content` shorthand가 두 축을 함께 바꾸는 경우도 scope 밖이다. |
| 9 | altered child/text/순서가 candidate로 오인되는 경우 | 잘못된 첫/둘째 text, direct text 순서 변경, 추가 item의 rejection tests를 실행했다. |
| 10 | legacy intrinsic track profile이 candidate allowance를 물려받는 경우 | `RootHtmlIntrinsicTrackSizingV1`의 `grid-auto-rows:auto` + stretch 입력 rejection regression이 통과했다. |
| 11 | one-axis shared placement contract가 일반 mode까지 넓어지는 경우 | normalizer 예외는 mode가 정확히 `RootHtmlAutoRowStretchV1`일 때만 작동하도록 분리했다. |
| 12 | fixed row 또는 explicit track가 stretch 대상이 되는 경우 | 10px explicit row는 `[10,15,15]px`에서 계속 10px이며, opt-in trace의 implicit count는 2다. |
| 13 | 5px 기대값이 구현 상수로 주입되는 경우 | layout test는 기존 track-sizing pass를 호출해 `normal`·`stretch` 모두 같은 5px 배분을 얻는다. 신규 sizing arithmetic/constant는 추가되지 않았다. |
| 14 | Chrome/독립 기대값 순서가 반전되는 경우 | independent `expected.json`의 세 item frame과 Root Grid 40×40 frame을 RuntimeSession 결과와 index/order별 비교한다. |
| 15 | scene painting identity가 layout frame 대조를 우회하는 경우 | normal/stretch painted tuples에 NodeId와 frame을 넣어 비교하고, 두 text draw 명령 및 source order를 검사한다. |
| 16 | style/layout revision mismatch snapshot이 성공 처리되는 경우 | 두 alignment scene 모두 completed layout key가 cascade key와 같은지 검사한다. 각 text source revision도 completed key와 대조한다. |
| 17 | unsupported request가 partial geometry를 남기는 경우 | rejection controls는 `completed.is_none()`까지 assert한다. |
| 18 | 같은 RuntimeSession이 rejection 후 복구하지 못하는 경우 | media query로 `stretch→start` 실패를 만든 뒤 같은 session에 `stretch` 환경을 복원해 `[10,15,15]px` scene 성공을 확인한다. |
| 19 | Simulator가 다른 fixture나 stale app asset을 실행하는 경우 | Android APK assets, iOS app `index.html`, source fixture SHA-256이 모두 `44a3423c…e7308`이다. Runtime boot marker와 `layout=ready`를 각 프로세스 PID로 필터링했다. |
| 20 | 시각 smoke를 pixel parity·복원 성공으로 과장하는 경우 | screenshot은 2개 text draw의 시각 증거로만 제한한다. `environment.txt`는 `asset_and_simulator_restoration=PASS`와 설치 APK/executable 이전·이후 hash equality를 기록한다. iOS duplicate `RawWindowMetalLayer` linker warning 및 readback/WPT/physical-device 비검증을 문서화했다. |

초기 review에서 `stretch`가 projection·layout contract·shared placement validator의 독립 세 gate에서 차례로 막히는 것을 확인했다. 세 경계를 exact candidate에 한정해 수정했고, workspace Clippy 첫 실행에서 발견한 두 `nonminimal_bool` 경고도 허용값 predicate를 간결하게 묶어 해결했다. 검토 후 미해결 실패 경로는 없다. 이 review는 계획 검토 20개를 재사용하지 않는다.
