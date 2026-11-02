# C11.1 percentage tracks PR diff 적대 검토 30개

대상은 `feat/c11-root-html-grid-percentage-tracks`의 현재 staged diff 전체다. 항목은 계획의 20개 검토와 구현/runtime 20개 검토를 반복하지 않고, 파일 경계·호출 연결·회귀·근거·계약 동기화를 PR 변경분에서 따로 살폈다. 결과는 로컬 candidate의 diff review이며 외부 승인 리뷰나 CI를 뜻하지 않는다.

| # | diff failure vector | 확인 결과 |
|---:|---|---|
| 1 | 이전 `root_html_grid_fr` validator module을 지우며 선언만 남기거나 새 module을 중복 연결하는가 | `style_values.rs`의 선언과 호출이 `root_html_grid_tracks` 한 경로에 연결되고, 이전 파일의 사용처는 남지 않는다. |
| 2 | `root_html_fr_track_tests` 이동 중 기존 `fr` unit test를 잃는가 | Git rename 대상에 fraction positive/rejection tests가 유지되고 새 percentage tests가 추가됐다. |
| 3 | Root HTML validator 허용값을 일반 Grid profile까지 확장하는가 | gate는 `RuntimeRootHtmlTextPaintV1` 경로에서만 실행되고 generic converter만 기존 typed percentage를 전달한다. |
| 4 | `%`를 조기 CSS px 환산해 style snapshot의 authored computed value를 잃는가 | projection test에서 `GridTrackSize::Percentage`를 유지하고 runtime은 definite axis에 맡긴다. |
| 5 | horizontal axis와 width/min-width/max-width를 엇갈리게 묶는가 | `AxisConstraints` width mapping을 call-site와 test matrix에서 확인했다. |
| 6 | vertical axis와 height/min-height/max-height를 엇갈리게 묶는가 | `AxisConstraints` height mapping을 call-site와 indefinite-row rejection에서 확인했다. |
| 7 | validator 구조 개선 후 필드 하나가 빠지거나 잘못 전달되는가 | Clippy의 too-many-arguments 수정 뒤 projection 4/4, runtime 16/16, scoped Clippy가 통과했다. |
| 8 | 해당 axis의 `min-size:auto` 계약을 생략하는가 | width/height min 조건이 axis value와 함께 검사되고 min override rejection이 있다. |
| 9 | 해당 axis의 `max-size:none` 계약을 생략하는가 | width/height max 조건이 axis value와 함께 검사되고 max override rejection이 있다. |
| 10 | `aspect-ratio` 또는 math-backed basis가 definite length 검사를 우회하는가 | aspect ratio와 `layout_math_values`를 별도 diagnostic으로 거부한다. |
| 11 | percentage admission 때문에 fixed-px-only gap contract도 함께 넓어지는가 | 기존 gap validator는 그대로 fixed finite non-negative px subset을 요구하고 `%` gap rejection이 유지된다. |
| 12 | gap이 percentage basis에서 빠져 Chrome/규범 expected origin과 달라지는가 | precomparison과 `% + fr` runtime expected values가 basis `[60,120,20]` 및 origin 70을 고정한다. |
| 13 | 합산을 f32로 먼저 수행해 overflow 검사 전에 값을 wrap/non-finite로 만드는가 | resolved percentage와 aggregate는 f64로 계산한 뒤 `f32::MAX`와 비교한다. |
| 14 | individual percentage resolve는 안전하지만 여러 fixed/percentage track 합은 overflow하는가 | fixed aggregate와 200개 거대 percentage 누산 rejection이 있고 partial completion도 확인한다. |
| 15 | gap count가 track 수와 다르거나 gap을 두 번 더하는가 | 누산은 `track_count - 1` gaps만 한 번 포함하고 `% + fr` positive case는 free-space 결과를 별도 고정한다. |
| 16 | `fr`을 fixed aggregate로 더해 정상 mixed axis를 오거부하는가 | fraction 값은 aggregate에 더하지 않고 기존 Taffy flex algorithm에 전달한다. |
| 17 | invalid direct 값과 unsupported nested syntax가 서로의 진단을 가리는가 | negative/non-finite direct percentage는 track validator, nested/unsupported form은 기존 projection validator가 원 property를 보고한다. |
| 18 | named-area fixed-px 예외에 percentage track을 실수로 허용하는가 | area template gate 및 percentage + areas runtime rejection이 유지된다. |
| 19 | new CSS fixture에 browser-only negative/spacer scene이 simulator로 유입되는가 | simulator fixture는 두 positive Grid만 갖고 negative matrix는 Rust tests에 남는다. |
| 20 | fixture CSS의 보조 색상/item 변경이 기존 fr/area test 의미를 바꾸는가 | 새 marker color는 overflow scene 식별용이며 existing positive/rejection matrix가 회귀로 통과한다. |
| 21 | root HTML integration이 fixture-only FFI constructor나 scene ID를 추가하는가 | 변경은 typed style gate/test path이며 app은 기본 `index.html` route를 사용한다. |
| 22 | V8 probe가 test-only element tree 또는 semantic child order를 강제하는가 | probe는 root, 두 Grid, item class 구조만 확인하고 geometry assertions는 Rust runtime에서 semantic paint/frame으로 한다. |
| 23 | source reordering이나 selector renaming에서 assertion이 NodeId·class index에 묶이는가 | Rust positive matrix는 semantic colors와 reversed-source-order case를 사용한다. |
| 24 | expected track/frame 값의 fixture와 reference hash가 어긋나는가 | precomparison SHA manifest, Chrome observation, expected JSON 및 runtime source는 같은 고정 fixture를 가리킨다. |
| 25 | Android APK가 같은 이름의 다른 bootstrap asset을 읽는가 | `assets/index.html`을 fixture와 byte-compare하고 asset SHA를 runtime evidence에 기록한다. |
| 26 | iOS app bundle이 stale generated bootstrap asset을 읽는가 | built `.app/index.html`을 같은 fixture와 byte-compare하고 SHA가 Android와 일치한다. |
| 27 | 플랫폼 capture에서 task fixture flags가 켜져 default route를 오염시키는가 | Android/iOS build logs에서 `c04_runtime_gpu_fixture_test=0`, C11.2.3 fixture-test flag 0을 확인했다. |
| 28 | viewport/DPR 차이를 pixel parity로 과장하는가 | README는 CSS frame/present와 시각 screenshot만 보고하고 PixelCopy·Metal readback은 하지 않았다고 제한한다. |
| 29 | capture 실패·재시도 뒤 임시 app source, 앱 process 또는 Agent Browser가 남는가 | shell trap은 app source/generated asset을 복구하고 Android 최초 PID 조회 공백을 재시도한다. iOS screenshot 명령 출력을 보존하며 양 app process와 Agent Browser process가 캡처 뒤 비어 있다. |
| 30 | 계획/계약 변경을 공식 C11 완료나 새 인터페이스 버전으로 오집계하는가 | `spec/STATUS.md` checkbox는 수정하지 않았고 plan/contract/evidence는 C11 미완료·`72/213`·`0.1.0`을 유지한다. |

Diff review에서 수정이 필요한 결함은 발견되지 않았다. 로컬 verification은 projection 4/4, layout 1/1, Root HTML runtime 16/16, scoped Clippy, rustfmt, source/evidence checksum, Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator build/runtime이다. PR status checks, merge와 Tailnet publication은 이 문서의 검토 범위에 포함하지 않는다.
