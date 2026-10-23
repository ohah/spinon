# C11.4 definite percentage auto-repeat maximum PR diff 적대 검토 30개

검토 대상은 `origin/main` 대비 구현 PR #568의 구현·fixture·시뮬레이터 route·상태/로드맵 변경이다. 구현/runtime 검토 20개와 겹치지 않도록 PR diff 연결, 검증 데이터, 패키징, 범위 표현의 별도 실패 경로를 확인했다. 생성 뒤 GitHub 최종 diff와 화면 이미지를 확인했다.

| # | 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | Cascade가 maximum percentage를 string 재파싱 없이 typed breadth로 전달하는가? | 통과: Stylo computed value 경로에서 nonnegative percentage를 보존하고 layout이 별도 검증한다. |
| 2 | unsupported max-content 계열 값이 auto-repeat admission을 우회하는가? | 통과: cascade와 layout 모두 auto/min-content/max-content를 닫힌 match로 거부한다. |
| 3 | 기존 고정 px/flex maximum 회귀가 생겼는가? | 통과: 기존 auto-repeat sizing 경로를 재사용하고 fixed control scene/test와 workspace 회귀를 통과했다. |
| 4 | 이번 수정이 vendored Taffy algorithm을 불필요하게 fork/변경하는가? | 통과: Taffy의 fixed component, definite maximum 선택, minimum flooring, gap 계산을 그대로 사용한다. |
| 5 | columns percentage가 row height 또는 viewport를 basis로 쓰는가? | 통과: layout gate는 columns width, rows height를 축별로 연결한다. |
| 6 | BorderBox outer dimension을 percentage basis로 전달하는가? | 통과: fixed padding과 border를 빼고 content axis를 사용하며 전용 회귀가 있다. |
| 7 | max constraint가 있는 축의 불명확한 percentage basis를 허용하는가? | 통과: min-size 0/max-size auto 외의 이 제한 slice를 fail-closed한다. |
| 8 | calc maximum이 다른 node 또는 axis에서 온 AST를 소비하는가? | 통과: id, node, GridTemplateColumns/Rows property가 함께 검증되고 mismatch 테스트가 있다. |
| 9 | 최대 calc가 arbitrary expression tree를 통과시키는가? | 통과: 정확히 두 항 length/percentage Sum으로 제한하고 max(), 음수 항 및 세 항 Sum을 거부한다. |
| 10 | direct percentage 곱셈 overflow가 Taffy에 전달되는가? | 통과: 곱셈 유한성 gate와 `f32::MAX` 회귀가 있다. |
| 11 | maximum below minimum 또는 zero maximum이 track minimum을 낮추는가? | 통과: 두 축과 반복 방식에서 fixed control flooring 결과가 일치한다. |
| 12 | auto-repeat count에서 gap 또는 첫 repeat special case가 빠지는가? | 통과: 8px gap과 3/4 repeat boundary를 가진 고정 fixture가 Taffy trace/frame과 맞는다. |
| 13 | `auto-fill`과 `auto-fit` 빈 track 결과가 같은 것으로 처리되는가? | 통과: fixture와 layout tests가 fill 보존 및 fit post-placement collapse를 각각 고정한다. |
| 14 | columns/rows, fill/fit, simultaneous axes 중 route가 누락되는가? | 통과: 28개 고유 ID와 양 축 동시 case를 Android/iOS에서 각각 모두 실행했다. |
| 15 | fixture Rust module 또는 JSON payload가 기본 feature에 들어가는가? | 통과: 전용 `c04-runtime-gpu-fixture-test` cfg와 `include_str!` 모듈 경계를 검사했다. |
| 16 | fixture host가 목록에 없는 scene을 실행하는가? | 통과: exact ID catalog와 unknown ID 거부를 Rust FFI test가 확인한다. |
| 17 | scene JSON을 JavaScript source에 끼워 넣을 때 escape/injection이 가능한가? | 통과: 전체 scene을 serde_json으로 serialize한 문자열을 JS JSON.parse 인자로 전달한다. |
| 18 | catalog에 중복 ID, expected item 누락 또는 track/collapse 길이 불일치가 있는가? | 통과: unique ID, item/frame cardinality, root 크기와 per-axis track/collapse 길이를 검증한다. |
| 19 | capture script가 전체 simulator catalog를 오래된 수로 제한하는가? | 통과: 전체 392 scene, percentage maximum 28 scene, 선택 ID 집합을 테스트한다. |
| 20 | capture script 선택 결과가 Android/iOS allowlist와 어긋나는가? | 통과: 각 28 ID가 양 플랫폼 route와 simulator catalog에 존재함을 Node 검사와 실제 실행으로 확인했다. |
| 21 | 예상 node frame이 fixture 기대 geometry에서 drift하는가? | 통과: JSON expected frame, host expected frame, Rust observed frame을 28 scene에서 비교했다. |
| 22 | pixel sample이 경계·잘못된 node·중복 지점을 검증하는가? | 통과: allowlist, target node, interior center, sample id/cardinality와 expected color를 고정한다. |
| 23 | Android 표본 요약이 일부 성공 scene만으로 78/78을 주장하는가? | 통과: 원본 runtime log에서 28 matched summaries와 78 exact PixelCopy sample을 독립 집계했다. |
| 24 | iOS 표본이 compositor 이후 결과로 과장되는가? | 통과: Metal drawable 표본 78/78 exact의 경계는 compositor 제출 전이라고 기록한다. |
| 25 | Android AVD의 software renderer를 hardware GPU 결과로 표시하는가? | 통과: ANGLE/SwiftShader 제한을 README·계획·roadmap에 표시한다. |
| 26 | screenshot이 실제 실행 scene과 다른 앱/build의 이미지인가? | 통과: Android/iOS run ID별 build/runtime log, result JSON, 28 screenshots를 연결한다. |
| 27 | 기본 product archive의 fixture 부재를 최종 APK/IPA 전체 제외로 넓히는가? | 통과: archive evidence에 static archive 한계를 명시하고 app bundle exclusion은 주장하지 않는다. |
| 28 | 테스트/Clippy 성공이 오래된 로그만 가리키는가? | 통과: 구현 검토 보완 뒤 workspace, focused layout/style/FFI, module-boundary 로그를 새로 저장했다. |
| 29 | 상태 대장이나 계획이 C11.4 완료/공식 집계 증가를 암시하는가? | 통과: checkbox와 `72/213`을 유지하고 이 구현을 PR 전 후보로 표시한다. |
| 30 | 공개 roadmap이 C13을 활성으로 보이게 하거나 C11.4 slice를 일반 CSS 완료로 보이는가? | 통과: C11.4 active와 제한 profile 경계를 쓰고 C13은 incomplete/inactive, 부분 wheel 및 미구현 CSSOM을 명시한다. |

**결과:** 미해결 P1/P2는 찾지 못했다. C11.4는 미완료다. PR #568의 GitHub diff는 로컬 기준 243개 파일과 일치했고 base는 `main`, head는 `7ed088a5b08a847627f7fab5ac4ce0d14d5fc61a`였다. 본문 이미지 두 장은 Agent Browser에서 각각 `complete=true`, natural size 1080×2400 및 1206×2622였다. PR은 2026-10-23T14:16:01Z Rebase and merge됐고 merge SHA는 `c436a38388fe8a191fd374495c516c88d1e3cd54`다. GitHub check rollup은 비어 있고 `main` required status checks도 설정되지 않았다. 후속 merge 기록으로 계획·상태 대장·로드맵을 갱신하되 C11.4 checkbox와 `72/213`은 유지한다.
