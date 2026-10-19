# C11.4 cyclic percentage/calc gap PR diff 적대 검토 · 20개 관점

**검토 범위:** 이 PR의 Taffy gutter intrinsic-basis 변경, root Grid max-content projection 허용, 여섯 장면의 Rust/FFI·Android/iOS 검증 경로, 계획·상태·계약·실행 근거 변경.
**검토 결과:** 아래 20개 실패 경로에서 미해결 P1/P2 결함을 찾지 못했다. 통과 근거는 제한된 고정 fixture에만 적용된다. C11.4 완료 판정은 바꾸지 않는다.

| # | 적대 관점 | 공격 경로 | 판정과 근거 |
| ---: | --- | --- | --- |
| 1 | definite percentage gap | 고정 `100×40px`에서 퍼센트 gap이 viewport나 min-size를 기준으로 잘못 resolve될 수 있음 | Chrome 155·Rust·두 simulator의 root/frame 및 gap이 `10/10px`로 일치한다. |
| 2 | cyclic percentage intrinsic sizing | intrinsic stage가 최종 `%`를 먼저 더해 root를 부풀릴 수 있음 | gutter만 임시 0 basis를 사용한다. max-content percent scene은 `100×20px`, 최종 gap은 `10/5px`다. |
| 3 | mixed `calc()` components | 0 percentage basis 처리 중 `calc()`의 5px absolute 항까지 사라질 수 있음 | definite·cyclic `calc()`가 각각 Chrome 기대값 `100×40`, `105×25` 및 gap `15/15`, `15.5/11.25px`에 맞는다. |
| 4 | definite minimum leakage | `min-width`/`min-height`를 cyclic gap intrinsic basis로 재사용할 수 있음 | true inner size와 일반 percentage basis를 분리한다. min-basis cases는 root `120×30px`, final gap `12/7.5` 및 `17/12.5px`다. |
| 5 | final resolve 누락 | intrinsic stage의 임시 0이 final used gap이나 trace에 남을 수 있음 | six Rust case가 item offset, used gap 및 마지막 active-gutter trace를 각각 확인한다. |
| 6 | regular track scope bleed | gutter 처리 분기가 일반 track의 percent·`fit-content()` 한도까지 바꿀 수 있음 | 분기는 `GridTrackKind::Gutter` 초기화에 한정하고 fit-content limit은 true inner size를 사용한다. workspace regression이 통과했다. |
| 7 | row/column basis swap | column-gap percentage를 height에, row-gap을 width에 대입할 수 있음 | 서로 다른 10%/25% row·column 값을 여섯 case의 x/y offset과 별도 trace로 검증했다. |
| 8 | min-size overflow masking | grid root를 min-size로 clamp한 뒤 item overflow가 누락될 수 있음 | cyclic calc min-basis의 아래 overflow `2.5px`를 frame assertion과 Android/iOS 표본으로 보존했다. |
| 9 | wrong grid-item placement | 두 번째 column/row offset이 맞아도 다른 node frame을 대조할 수 있음 | 네 item ID와 paint order를 고정하고 Android/iOS summary의 다섯 node frame을 검사한다. |
| 10 | projection profile expansion | 새 `max-content` 허용이 다른 runtime profile에도 노출될 수 있음 | `RuntimeBlockFlexGridTrackSizingV1`에서만 허용하며 다른 profile negative assertion이 통과했다. |
| 11 | root-only exception escape | nested Grid 또는 block root가 같은 우회 경로로 통과할 수 있음 | 허용 조건은 root handle과 Grid display를 모두 요구한다. non-Grid root 거부 회귀가 통과한다. |
| 12 | unsupported intrinsic keywords | `min-content`나 `fit-content()`가 새 예외에 딸려 들어올 수 있음 | width/height에서 MinContent·FitContent를 배제하고 기존 fail-closed 경로를 유지한다. |
| 13 | fixture JavaScript injection | scene/node ID가 따옴표 없는 JS identifier나 실행 가능한 source로 들어갈 수 있음 | source는 고정 scene allowlist를 사용하고 Rust `{:?}` string serialization 및 생성 결과 assertion을 둔다. |
| 14 | unbounded scene source | 임의 scene 입력이 대형 JS payload나 메모리 사용으로 이어질 수 있음 | 알 수 없는 ID는 거부하고 새 scene source는 4 KiB 상한을 검사한다. |
| 15 | scene expectation drift | FFI 장면, frame assertion, scene count가 서로 다른 세트를 사용할 수 있음 | Android/iOS 각각 정확한 여섯 scene allowlist, root/item/node frames와 `boxes=5`를 검사하고 6/6 실행했다. |
| 16 | Android sample false positive | gap 배경점이 item 색 또는 오래된 node에서 읽혀 통과할 수 있음 | JSON target node ID·좌표를 scene expectation과 맞추고 gap 배경 RGBA `[16,24,32,255]`를 비교한다. PixelCopy 29/29. |
| 17 | iOS sample/parser mismatch | Metal parser allowlist 또는 sample node/coordinate가 scene과 다를 수 있음 | 신규 scene 여섯 개를 allowlist와 정확한 RGBA 표에 연결하고 run/scene ID를 확인한다. Metal drawable 29/29. |
| 18 | offscreen overflow pixel omitted | 화면이 보여도 grid 바깥의 overflow가 clip될 수 있음 | percent, calc 및 calc-min-basis right/bottom 표본이 red item 영역에 놓이며 양 플랫폼에서 모두 일치한다. |
| 19 | wrong runtime target / leaked app | 자동 ADB 선택이 물리 단말을 쓰거나 실패 뒤 app process가 남을 수 있음 | script는 `emulator-5554`, API 37 ARM64 AVD 이름과 iPhone 17 Pro/iOS 26.2 UDID를 고정하고 종료 trap을 둔다. 실행 후 두 app은 종료됐다. |
| 20 | evidence/status overclaim | screenshot이나 simulator 결과를 전체 Grid 또는 device/compositor 지원으로 과장할 수 있음 | V8/app artifact hash, run ID, 12 screenshots와 limitation을 기록한다. Android는 software AVD, iOS는 compositor 전이며 C11.4 checkbox·완료 수는 유지한다. |

## 검증 기록

- `CARGO_INCREMENTAL=0 cargo test --locked --workspace`: 1,126 passed, 0 failed, 4 ignored.
- workspace all-target Clippy `-D warnings` 및 `cargo fmt --all -- --check`: 통과.
- Android API 37 ARM64 AVD: frame 6/6, PixelCopy 29/29; iPhone 17 Pro/iOS 26.2 Simulator: frame 6/6, Metal drawable 29/29.
- 전체 log와 screenshot은 [`runs/c114-gap-20261019T181753Z`](./runs/c114-gap-20261019T181753Z/)에 있다. 두 platform screenshot은 simulator 결과이며 physical device, Android hardware GPU, iOS compositor-after parity, WPT 또는 전체 C11.4를 입증하지 않는다.
