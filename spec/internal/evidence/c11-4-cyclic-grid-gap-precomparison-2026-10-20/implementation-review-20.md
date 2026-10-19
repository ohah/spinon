# C11.4 percentage/calc gap 구현 적대 검토 · 20개 관점

**검토 대상:** cyclic Grid percentage/calc gap Taffy 수정, typed layout 회귀, opt-in runtime scene, Android/iOS fixture route와 실행 증거.
**결론:** 구현·통합 중 확인한 layout, projection, fixture routing, platform sampling 결함을 수정했다. Chrome 155와 Rust 6-case geometry 대조, workspace test·Clippy·format, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 frame 6/6 및 표면 표본 각 29/29가 통과했다. 구현은 제한된 C11.4 candidate이며 C11.4 전체 완료를 뜻하지 않는다.

| # | 적대 관점 | 공격·실패 경로 | 판정과 근거 |
| ---: | --- | --- | --- |
| 1 | definite percentage 기준 | fixed `100×40` Grid의 `%` gap이 min-content나 viewport 기준으로 계산될 수 있음 | Chrome 155와 Rust에서 최종 gap `10/10px`, root `100×40px` 일치. |
| 2 | cyclic percentage intrinsic 누출 | max-content 크기 결정 중 최종 10%/25%를 미리 반영하면 tracks 뒤로 root가 커짐 | intrinsic 기여에서는 0%, 최종 layout은 `100×20px` root에 `10/5px` gap. |
| 3 | definite `calc()` 길이 소실 | percentage basis를 0으로 바꾸면서 `5px`도 없어질 수 있음 | definite `calc(5px + %)` 결과 `15/15px`, root `100×40px` 일치. |
| 4 | cyclic `calc()` 고정 항 소실 | 전체 `calc()`를 미정 값으로 처리하면 max-content contribution이 0이 됨 | zero basis에서 고정 `5px`를 보존해 `105×25px`; 최종 gap `15.5/11.25px`. |
| 5 | min-size와 cyclic percentage 혼동 | definite `min-width/min-height`를 cyclic gap intrinsic basis로 쓰면 퍼센트가 조기 반영됨 | Chrome 기준 `120×30px`, gap `12/7.5px`; Rust 6-case test 통과. |
| 6 | min-size와 cyclic `calc()` 혼동 | 5px 절대항과 min-size percentage를 합쳐 intrinsic size를 부풀릴 수 있음 | 최초 검토에서 row root가 `32.5px`로 잘못 커지는 결함을 재현. definite min-size와 true inner size를 분리해 intrinsic gap만 zero-basis로 바꾼 뒤 Chrome의 `30px` 및 아래 overflow `2.5px`와 일치. |
| 7 | 최종 basis 재사용 누락 | intrinsic stage의 임시 gutter 크기가 최종 frame·trace에 남을 수 있음 | 여섯 Rust case가 root 크기와 최종 used gap을 각각 검사; 모든 기대값 통과. |
| 8 | row/column basis 뒤바뀜 | 높이 percentage가 width를, 또는 width percentage가 height를 사용함 | 2×2 geometry에서 행·열 second-item offset과 각 최종 gutter를 따로 비교. |
| 9 | gap 위치 오프셋 오류 | gap 크기만 맞고 두 번째 열·행 시작 위치가 다를 수 있음 | 각 case에서 second-column x 및 second-row y를 Chrome 값과 `0.001px` 허용차로 비교. |
| 10 | overflow를 root 안으로 clamp | Grid 외곽을 넘는 item이 숨거나 root 크기에 합산될 수 있음 | percent/calc definite·cyclic 사례의 오른쪽·아래 item overflow를 별도 assertion. |
| 11 | 모든 track에 임시 basis 적용 | intrinsic regular track의 `%`/`calc()` 함수까지 0 basis로 바뀔 수 있음 | Taffy 수정은 `GridTrackKind::Gutter`에만 분기하고 track input regression은 workspace test로 확인. |
| 12 | fit-content·fixed sizing 회귀 | gutter 계산 변경이 일반 track의 definite value나 fit-content limit에 영향 | 일반 track은 기존 `percentage_basis`를 유지하고 fit-content limit은 `inner_node_size`를 유지하는 코드 경계를 확인; workspace tests 통과. |
| 13 | 여러 gutter·collapsed track 혼입 | 최종 trace에서 gap 하나만 찾는 테스트가 다중/빈 gutter를 잘못 해석할 수 있음 | 이 fixture는 2×2 explicit track으로 축마다 active gutter 하나를 고정하고 final pass/stage의 active count와 크기를 검사; auto-repeat/collapse는 별도 범위로 남김. |
| 14 | `calc()` property identity 손실 | row/column CSS math ID가 뒤바뀌거나 동일 expression이 잘못 resolve됨 | Rust fixture가 서로 다른 `LayoutCssMathProperty` ID를 만들고 행·열 gap/offset을 각각 비교. |
| 15 | 동적 scene 입력 오염 | 임의 scene ID가 raw JS 생성 경로로 들어가거나 payload가 무제한 증가함 | 고정 scene allowlist와 4 KiB source upper bound를 fixture test로 검사; 알 수 없는 ID 거부. |
| 16 | 기본 제품 ABI에 시험 경로 노출 | 앱 검증용 scene/FFI가 기본 feature에 들어가 제품 계약을 바꿀 수 있음 | default archive에서 여섯 scene ID·두 fixture symbol이 빠졌고 opt-in `c04-runtime-gpu-fixture-test` archive에서 모두 확인됐다. artifact SHA-256은 [경계 검사 log](./artifact-boundary.log)에 있다. |
| 17 | Android geometry·색 기대치 불일치 | 5개 box frame은 맞아도 PixelCopy gap sample이 다른 색으로 통과하거나 실패할 수 있음 | `gap-background` RGBA `[16,24,32,255]`를 연결했다. API 37 AVD에서 6 scene frame과 29/29 PixelCopy 표본이 모두 일치했다. |
| 18 | iOS fixture route·Metal sample 불일치 | scene allowlist와 frame·표본 node/coordinate가 서로 다른 이름을 쓸 수 있음 | 신규 여섯 scene을 iOS Metal parser allowlist에 추가했다. iPhone 17 Pro/iOS 26.2 Simulator에서 6 scene frame 및 compositor 전 Metal 표본 29/29가 통과했다. |
| 19 | 실제 기기 오접속·실패 뒤 앱 잔류 | 기본 ADB 선택으로 물리 단말을 쓰거나 중간 실패 시 앱이 남을 수 있음 | script는 `adb -s emulator-5554`, API/ABI/AVD 검사를 고정하고 iPhone UDID를 확인하며 `trap`에서 테스트 앱만 종료한다. 지정된 AVD와 Simulator 실행이 완료됐고 실기기 ADB는 사용하지 않았다. |
| 20 | 브라우저/산출물 근거 혼동 | screenshot만 남기거나 Terminal Browser를 쓰고 전용 browser 프로세스를 남길 수 있음 | Chrome 155 DOM JSON·화면은 Agent Browser로 수집하고 해당 session 및 전용 Chrome/daemon 프로세스를 종료했다. PR에 연결할 두 플랫폼 화면과 원본 runtime log는 [run evidence](./runs/c114-gap-20261019T181753Z/)에 보존했다. |

## 수정 중 검토 결함

- Android `c114GridExpectedRgb()`에 새 `gap-background` sample ID가 없었다. RGB 검사를 무효화하지 않도록 정확한 stage 배경색을 추가했다.
- Taffy 초기 gutter basis가 `inner_node_size.or(axis_min_size)` 하나만 받아 definite minimum을 cyclic intrinsic basis로 오인했다. true inner size와 일반 percentage basis를 별도로 전달하도록 고쳤다. Chrome min-size/calc scene과 Rust 회귀가 이를 확인한다.
- final trace selector의 `.last()`에 Clippy `double_ended_iterator_last` 경고가 있었고, `.rfind()`로 바꿨다.
- Android app CSS에서 지원하지 않는 `justify-items:start`를 제거해 장면 검증을 사용 가능한 선언으로 제한했다.
- FFI scene 생성기가 item node ID를 JS 문자열이 아닌 식별자로 출력해 `ReferenceError`가 발생했다. JSON 문자열 placeholder로 고치고 generated source assertion을 추가했다.
- iOS Metal 표본 parser가 여섯 신규 scene ID를 허용하지 않아 실행 인수가 거부됐다. parser allowlist에 정확한 ID를 추가하고 각 scene의 Metal 표본 실행으로 확인했다.
- style projection은 C11.4 `RuntimeBlockFlexGridTrackSizingV1`의 root Grid에서만 `max-content`를 허용하도록 좁게 확장했다. 다른 profile, non-Grid root, `min-content`/`fit-content()`는 계속 거부하는 negative tests를 추가했다.

## 범위 한계

이 검토는 horizontal writing mode의 explicit 2×2 fixed-track, `%`/`calc()` gap 사례를 다룬다. 전체 Grid contribution ordering, auto-repeat interaction, nested/anonymous item, 다른 writing mode, WPT, 물리 기기, Android hardware GPU 및 OS compositor 이후 픽셀을 입증하지 않는다. 이 slice는 C11.4를 완료로 만들지 않는다.
