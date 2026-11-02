# C11.1 percentage tracks implementation/runtime 실패 관점 검토 20개

대상은 현재 `feat/c11-root-html-grid-percentage-tracks` working tree의 Root HTML style validation, typed Grid projection, runtime matrix와 양 플랫폼 기본 app 경로다. 이 검토는 계획의 20개 관점 및 계획 PR diff의 30개 관점과 별개다. 아래 각 항목은 서로 다른 실패 경계를 확인하며, candidate는 아직 미병합이다.

| # | 실패 관점 | 확인과 결과 |
|---:|---|---|
| 1 | column percentage가 definite width 대신 auto/intrinsic width를 받아도 성공하는가 | `percentage width basis` rejection은 `grid-template-columns` property로 layout 전에 실패한다. |
| 2 | row percentage가 definite height 대신 auto/intrinsic height를 받아도 성공하는가 | `indefinite percentage row axis` rejection은 `height` 원인으로 실패한다. |
| 3 | width의 `calc()` computed 결과를 확정 px basis로 잘못 취급하는가 | `layout_math_values`를 확인해 `width`를 거부한다. 계산 후 값만 검사하는 경로가 아니다. |
| 4 | width/height의 퍼센트 basis가 container의 다른 축과 섞이는가 | both-axis 양성 장면에서 columns `[60,120,20]`, rows `[30,60,10]`을 독립 definite basis에 대조한다. |
| 5 | 해당 축의 `min-size` override가 percentage track을 clamp하는가 | `min-width:0` rejection은 percentage track과 함께 실패하며 partial layout은 없다. |
| 6 | 해당 축의 `max-size` override가 percentage track을 clamp하는가 | `max-width:200px` rejection은 percentage track과 함께 실패하며 partial layout은 없다. |
| 7 | aspect-ratio가 반대 축 basis를 유도해 definite axis gate를 우회하는가 | `%` 축에서 `aspect-ratio`를 명시적으로 거부하고 typed projection에 전달하지 않는다. |
| 8 | `0%` basis나 `100%` 경계를 음수·clamp 값으로 해석하는가 | zero/full positive case는 `[0,200]`을 보존한다. |
| 9 | definite width가 0일 때 nonzero percentage가 auto로 승격되는가 | zero-basis positive case에서 `[0,0]`을 layout/runtime frame으로 확인한다. |
| 10 | fixed-px gap을 percentage basis에서 먼저 빼는가 | both-axis expected tracks와 gap origins를 비교하며 basis를 줄이지 않는다. |
| 11 | 같은 축 `% + fr`에서 gap을 flex free space에서 두 번 빼는가 | `25% 1fr`와 10px gap은 `[60,170]`, 두 번째 시작 x=70으로 확인한다. |
| 12 | 정상적인 100% 초과 track이 100%로 clamp되는가 | `120% 10%`은 `[240,20]`이며 second origin x=248이다. |
| 13 | track/root 밖의 visible item이 root edge에서 clip되는가 | Rust paint clip assertion과 Android/iOS screenshot에서 보라색 8px marker가 root edge 오른쪽에 남는다. |
| 14 | percentage projection이 computed typed value를 px로 조기 환산하는가 | projection test는 양 축 `GridTrackSize::Percentage` 값 자체가 유지되는지 확인한다. |
| 15 | 음수·NaN·±Infinity direct percentage가 projection gate를 통과하는가 | `root_html_profile_rejects_invalid_or_nondirect_percentage_tracks`가 모두 거부한다. |
| 16 | `calc()`, `minmax()`, named line 또는 repeat가 direct percentage처럼 처리되는가 | authored rejection matrix가 각 syntax를 `grid-template-columns` 진단으로 거부한다. |
| 17 | percentage gap이 basis 또는 fixed-px gap gate로 잘못 허용되는가 | `percentage gap remains unsupported`는 `column-gap` 진단을 받고 scene을 publish하지 않는다. |
| 18 | percentage와 named area 전용 profile의 조건이 섞이는가 | `percentage tracks remain unsupported with named areas`는 area 결합을 거부한다. |
| 19 | item 수가 explicit track 수보다 많아 implicit track이 생기는데 성공하는가 | `percentage implicit item remains unsupported`는 `grid-auto-rows` 경계에서 실패한다. |
| 20 | 단일 resolved track 또는 누산 합 overflow 뒤 partial frame이 남는가 | resolved percentage, fixed aggregate, gap aggregate, 200개 거대 percentage 누산 사례는 모두 실패하고 `completed=None`을 확인한다. |

실행 결과: `cargo test -p spinon-style-to-layout root_html_profile -- --nocapture` 4/4, `cargo test -p spinon-layout percentage_tracks_use_the_definite_grid_content_axes -- --nocapture` 1/1, `cargo test -p spinon-runtime root_html_grid -- --nocapture` 16/16, scoped Clippy `-D warnings`, `cargo fmt --all -- --check`, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 route 실행이 통과했다. Simulator screenshots는 시각 확인용이며 PixelCopy·Metal readback은 하지 않았다.

검토 범위 밖은 full workspace suite, WPT conformance, physical device/hardware GPU, iOS compositor 이후 pixel equality와 C11.1/C11 전체 완료다.
