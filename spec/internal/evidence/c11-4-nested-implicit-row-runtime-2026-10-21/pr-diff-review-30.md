# PR #448 변경분 실패 관점 검토 30개

검토 대상은 PR #448의 코드, scene fixture, 실행 증거와 상태/계획 문서다. 구현/runtime 20관점 검토와 구분해 diff 전체에서 별도의 계약 경계·실패 경로를 확인했다.

| # | 실패 관점 | 판정과 근거 |
| --- | --- | --- |
| 1 | 신규 scene 외 입력이 fixture FFI source에 섞이는가 | 통과: FFI adapter는 두 고정 ID만 source로 변환하고 미등록 ID는 `None`이다. `nested_grid_implicit_rows.rs`의 positive/negative test가 각각 확인한다. |
| 2 | generated JS source의 문자열 보간이 임의 입력을 실행하는가 | 통과: source 템플릿의 값은 컴파일타임 scene table에서 온 두 상수만 보간한다. arbitrary scene ID를 script에 끼워 넣는 경로가 없다. |
| 3 | 새 고정 PNG scene payload가 기본 제품 archive에 포함되는가 | 통과: default static archive 검사에서 두 scene ID와 PNG payload가 없고 opt-in archive positive control에서 모두 있다. |
| 4 | C11.4 runtime adapter가 fixture-test feature 밖에서 컴파일·실행되는가 | 통과: adapter와 runtime test module 선언은 `c11-4-grid-track-sizing-fixture-test`에 gated되어 있다. |
| 5 | 다른 runtime style profile의 문서가 해당 adapter에서 부분 처리되는가 | 통과: adapter는 `RuntimeBlockFlexGridTrackSizingV1`만 받아 다른 profile을 명시 거부한다. |
| 6 | HTML namespace, root tag, root ownership 변형을 허용하는가 | 통과: exact XHTML namespace, root `div`, `HostParent::Root`를 검증한다. |
| 7 | sibling Grid/item이 추가된 root에서 fixture가 계속 처리되는가 | 통과: outer root child 수가 정확히 1이어야 한다. |
| 8 | nested Grid의 node ID, tag, parent owner가 뒤바뀌는가 | 통과: exact scene-derived ID, `div`, outer-root parent를 함께 검사한다. |
| 9 | inner child 누락·중복·순서 변경이 intrinsic source 선택을 바꾸는가 | 통과: 정확히 세 child와 `[explicit, first implicit, image]` 순서를 고정한다. |
| 10 | auto-row pattern 외 track grammar나 named line/area/repeat가 조용히 통과하는가 | 통과: exact rows/auto_rows를 요구하고 auto-repeat, template areas, line names, auto-columns 변형을 거부한다. |
| 11 | outer 또는 inner Grid width/height/edge/gap/profile 변형이 기대값을 오염시키는가 | 통과: 24px width, auto height, 4px row gap, zero column gap/edges/border 및 min/max subset을 검사한다. |
| 12 | nested Grid item alignment 차이가 intrinsic sizing 결과에 새 영향을 주는가 | 통과: inner grid의 `align-items:start`와 `justify-items:normal`을 요구한다. |
| 13 | span-two와 one-row scene의 outer row end line이 혼동되는가 | 통과: scene ID별 `1/3` 또는 `1/2` end line을 adapter 및 platform expectation에서 분리한다. |
| 14 | `grid-row:3/4`가 실제 두 positive implicit rows를 만들지 않는가 | 통과: image placement 3/4와 explicit row 뒤의 fixed child placement 1/2, 2/3을 모두 확인한다. |
| 15 | auto-row list가 뒤 track에서 순환하면서 min/max order가 바뀌는가 | 통과: Chrome 재관찰, direct layout과 두 runtime trace 모두 `[8,8,16]` CSS px를 기록한다. |
| 16 | inner row gaps가 track base size에 섞이거나 contribution에서 누락되는가 | 통과: 두 4px gutter는 trace의 gutter slot이고 40px contribution은 `8+4+8+4+16`이다. |
| 17 | replaced item이 다른 node/source owner 또는 잘못된 URL을 사용해 outer contribution을 중복시키는가 | 통과: image leaf/URL/owner를 검사하고 intrinsic image input은 nested grid ID 및 source order 2에 귀속된다. |
| 18 | image request/decode 실패 또는 자연 크기 변조가 부분 layout으로 진행되는가 | 통과: request/decode errors가 반환되고 decoded PNG natural size가 정확히 24×16이 아니면 adapter가 거부한다. |
| 19 | stale/default intrinsic revision으로 layout snapshot이 생성되는가 | 통과: checked increment로 revision 1을 사용하고 Android/iOS report 검사도 revision 1을 요구한다. |
| 20 | outer Grid contribution의 node/kind/value/span이 nested item과 분리되지 않는가 | 통과: direct/runtime/app trace는 nested node 2, `MinContent`, 40px, span 2 또는 1을 확인한다. |
| 21 | two-row contribution이 outer gap을 포함한 채 두 track으로 분배되는가 | 통과: span-two 기대 Rows `[18,18]`, root/nested height 40px가 독립 계산과 Chrome observation 및 두 simulator에서 일치한다. |
| 22 | one-row scene의 unused second row 또는 row-gap slot이 사라지는가 | 통과: track bases `[40,0]`과 별도 4px gutter, 44px root height를 확인한다. |
| 23 | scene에서 직접 node frame 하나가 잘못되거나 JSON result가 box count만 확인하는가 | 통과: 다섯 node의 x/y/width/height를 scene별 예상 문자열과 비교한다. |
| 24 | Android 진단 JSON의 뒤쪽 log text가 JSON parse를 깨거나 일부 source만 검사되는가 | 통과: `JSONTokener`가 diagnostics object 하나를 읽고 intrinsic revision·source id/order/natural size를 검사한다. |
| 25 | iOS 진단 JSON parser가 quoted brace/escape를 object 종료로 오인하는가 | 통과: parser는 quoted string과 backslash escape를 추적한 뒤 balanced object만 JSONSerialization에 보낸다. |
| 26 | 두 platform validators가 trace의 한 축이나 한 Grid만 검사하는가 | 통과: 각 scene에서 outer/nested Grid 두 개, Columns/Rows 두 pass, row/column gutter layout을 모두 확인한다. |
| 27 | trace chunk truncation이 정상 trace로 오인되는가 | 통과: 최종 results는 Android scene당 8 chunks, iOS scene당 16 chunks이며 두 Grid trace가 끝까지 존재한다. |
| 28 | superseded layout, 미완료 draw 또는 present 오류가 scene 성공으로 집계되는가 | 통과: final result의 두 scene 모두 retry 0, `drawPresented=true`; runtime summary는 `layout=ready boxes=5`를 기록한다. |
| 29 | 최신 Android source와 설치 APK가 다른 상태의 결과가 제출되는가 | 통과: AVD build/install 이후 local 및 installed APK SHA-256가 `48739e…b81c1`로 일치한다. iOS Simulator app SHA-256도 source revision manifest에 연결했다. |
| 30 | 문서/PR이 고정 fixture를 일반 CSS 지원, PNG paint, border overflow, 실기기 또는 완료 수 증가로 확대 해석하는가 | 통과: 내부 계약·STATUS·계획·공개 roadmap과 PR 본문 모두 고정 2-scene 범위, 미지원 사항, `72/213` 유지 및 empty CI rollup 경계를 명시한다. PR 본문 Android/iOS 네 image는 Agent Browser에서 모두 `complete=true`, naturalWidth가 1080/1206이고 화면 크기가 0보다 큰 것을 확인했다. |

## 발견 사항과 재검증

- 사전 비교 HTML을 runtime-supported style subset에 맞춘 뒤 기존 Chrome observation에 source hash binding이 없는 것을 발견했다. Agent Browser 0.31.1 / HeadlessChrome 156으로 현재 HTML을 재실행해 fixture hash, used rows, image natural sizes, 모든 DOM frame을 새 `chrome156-runtime-profile-observation.json`에 고정했다. 관측 geometry는 기존 독립 계산과 같았고 screenshot SHA-256도 일치했다.
- Simulator capture 중 stale APK allowlist, row gutter trace expectation, intrinsic-only PNG draw-count requirement를 발견해 수정했다. 영향받은 platform validators/runner와 Android/iOS 최종 2/2 결과를 다시 확인했다. 실패 원본은 evidence에 남아 있다.

이 검토는 구현/runtime 20관점 review와 별개다. 두 fixed scene을 넘어서는 arbitrary CSS behavior, PixelCopy/Metal surface sampling, physical Android, WPT와 전체 C11.4 적합성을 검증한다고 주장하지 않는다.
