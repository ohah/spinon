# C11.1 percentage gap 구현/runtime 적대 검토 20개

검토 대상은 구현 commit `f92352d97eddfa5e6142d9db78d5a1900879587b`, 현재 Rust delta, 동일 commit의 Android/iOS Simulator 실행 로그와 캡처다. 별도 계획 검토 20개를 재사용하지 않았고 PR 최종 변경분 검토와도 분리한다.

| # | 별도 실패 경로·불변 조건 | 대조 근거와 결과 |
|---:|---|---|
| 1 | `row-gap`이 width, `column-gap`이 height를 basis로 잘못 선택 | [percentage geometry tests](./style-focused-pass02.log)의 축별 10% 사례에서 column=20px, row=10px와 item 좌표를 각각 확인했다. 통과. |
| 2 | `gap: row column` 두 값의 순서를 뒤집음 | `gap:5% 10%` expected frames에서 row=5px, column=20px를 확인했다. 통과. |
| 3 | legacy `grid-gap`이 새 percentage 경로를 우회하거나 빠짐 | 동일한 5%/10% computed longhand와 `gap` geometry를 비교했다. CSS Gaps §2.4의 alias 계약과 일치한다. 통과. |
| 4 | `0%`를 `normal` 또는 unsupported로 취급 | percentage zero 장면에서 computed zero와 기존 item origin을 별도로 확인했다. 통과. |
| 5 | definite 0px axis basis에서 75%가 NaN·미정으로 처리 | width=0px의 75% column gap이 0px used value와 x=0 origin을 유지한다. 통과. |
| 6 | 100% 초과 percentage gap을 100%로 clamp | width=100px, column-gap=120%의 두 번째 item x=130px를 Rust와 Chrome 기준에서 확인했다. 통과. |
| 7 | invalid negative 선언이 앞선 유효 cascade를 제거 | `column-gap:5px` 뒤의 `column-gap:-10px`에서 computed 5px와 x=15px가 유지된다. 통과. |
| 8 | valid typed `calc()`를 bare percentage와 같은 방식으로 admission | `calc(5% + 2px)`는 `column-gap`으로 명시해 거부하고 layout output을 만들지 않는다. 통과. |
| 9 | 대응 축이 indefinite인데 computed percentage만으로 layout 진행 | indefinite width 사례는 해당 axis/property로 fail-closed한다. 통과. |
| 10 | percentage axis의 min/max constraint가 basis를 바꾸는데도 승인 | projection의 min-width 거부와 direct layout의 min-height/max-height 거부를 확인했다. 통과. |
| 11 | percentage×basis 계산이 f32 한계를 넘은 뒤 변환 | resolver는 f64 곱셈 후 유한 f32 범위를 검사하며 f32 초과 사례를 거부한다. [gap helper tests](./style-gap-helper-pass01.log) 통과. |
| 12 | 개별 gap은 유한하지만 전체 gutter 누적과 fixed track이 f32 한계를 넘음 | direct row/column-flow aggregate overflow negative controls가 error를 반환하고 partial layout을 내지 않는다. [layout tests](./layout-focused-pass02.log) 통과. |
| 13 | implicit row 확장 시 row-gap을 빼거나 잘못된 basis 사용 | definite 100px height, 25% auto row, 10% gap에서 두 번째 row y=30px를 projection과 direct adapter에서 확인했다. 통과. |
| 14 | column-flow implicit column의 percentage gap 누락 또는 placement 순서 오류 | 50% auto column과 10% gap에서 세 번째 item x=60px 및 aggregate rejection을 확인했다. 통과. |
| 15 | `display:none` 자식이 required row/gutter 수를 늘려 정상 입력을 거부 | 숨긴 세 번째 item과 1px height에서 큰 단일 gap을 써도 두 visible item만 남아 layout이 성공한다. [hidden-child regression](./layout-focused-pass02.log) 통과. |
| 16 | named-area spanning item에 gutter 폭을 누락하거나 기존 px route 회귀 | fixed 2×2 named area의 percentage gap에서 header width=132px, lower row y=27px, second column x=52px를 확인했고 px control도 통과했다. |
| 17 | 새 gap admission이 Flex나 다른 runtime profile로 누출 | validator 호출은 `RuntimeRootHtmlTextPaintV1`의 인식된 Grid container만 대상으로 하며 새 feature/profile/FFI는 추가하지 않았다. diff와 profile dispatch 경계를 확인했다. 통과. |
| 18 | invalid gap 또는 extent 실패 시 partial `StyleLayoutOutput` 발행 | projection/layout은 `Result::Err`로 반환되고 focused rejection cases에서 성공 output이 생성되지 않는다. 통과. |
| 19 | desktop Rust 결과만 성공하고 앱 Root HTML/V8 경로가 fixture를 읽지 못함 | Android/iOS 양쪽에서 `source=index.html`, `root_app_marker=ready`, `layout=ready boxes=7`, `presented boxes=7`을 로그에서 확인했다. |
| 20 | root 밖 넘침이 clip되거나 screenshot을 표면 pixel parity로 과장 | 양쪽 screenshot에서 200px root 밖 두 번째 열을 확인했고 Rust exact frame은 x=260px이다. 캡처는 visual smoke로만 분류했으며 PixelCopy/Metal readback은 수행하지 않았다. |

검토 중 숨김 item이 required row count에 미치는 영향의 전용 regression assertion이 없음을 발견해 테스트를 추가했다. 구현은 이미 visible child만 세고 있었으며 새 검사는 그 경계를 증명한다. 이 검토에서 추가 구현 차단 결함은 발견하지 못했다.

범위 한계: 전체 `spinon-style-to-layout`와 `spinon-layout --lib` suite는 각각 2개·1개의 실패가 있어 전체 green으로 보고하지 않는다. 전체 결과와 원문은 [style package log](./style-package-full-pass01.log), [layout package log](./layout-package-full-pass01.log)에 있다. scoped focused tests, Clippy, rustfmt, diff check와 두 Simulator 실행은 별도 결과다.
