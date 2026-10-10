# C08 Block 흐름·기본 페인트 구현 실패 경로 검토

검토 대상은 `RuntimeBlockPaintV1` cascade profile, Taffy Block 투영, runtime render scene, V8 fixture, C ABI, Android JNI, iOS Objective-C++/Swift 경로, 고정 Chromium 기준이다. 계획 검토와 다른 입력·실패 경계를 사용했다.

| # | 공격 관점 | 확인 및 조치 |
| --- | --- | --- |
| 1 | C08 profile이 기존 Flex paint profile의 계산 의미를 바꾸는가 | 별도 `RuntimeBlockPaintV1`과 계산기 선택으로 분리했다. 예전 profile 경로와 기존 회귀 테스트를 유지한다. |
| 2 | 다른 runtime session의 profile 결과가 C08 캐시에 섞이는가 | C08 전용 cascade coordinator를 세션 생성 시 지정한다. incremental cache는 profile이 같지 않으면 재사용을 거부한다. |
| 3 | 지원 CSS allowlist가 문서와 실제 author 입력 검사에서 다른가 | inline·stylesheet 경계에서 `display`, `box-sizing`, `width`, `height`, `background-color`, `color`, `font-size`, `font-family`만 author 값으로 허용한다. |
| 4 | 지원 가능한 author stylesheet까지 거부되는가 | `.painted` 규칙의 높이·배경색이 computed style과 GPU scene에 도달하는 runtime test를 추가해 통과했다. |
| 5 | stylesheet shorthand가 longhand 확장으로 allowlist를 우회하는가 | `border` 선언이 확장된 unsupported property로 거부되는 test를 추가해 통과했다. |
| 6 | stylesheet parse diagnostic 뒤 기본값으로 부분 장면을 그리는가 | 잘못된 `background-color` 입력에서 `cascade_diagnostics`를 반환하고 scene을 공개하지 않는 test를 추가해 통과했다. |
| 7 | inline shorthand로 미지원 border paint를 요청하면 선을 그리거나 무시하는가 | `border` inline 입력은 `unsupported_inline_property`로 거부된다. C07.2의 border width는 layout 값이며 painter는 선을 그리지 않는다. |
| 8 | `display:flex` 등 Block 외 display가 조용히 Taffy Block으로 바뀌는가 | 모든 computed element display를 검사해 `block`·`none` 이외 값을 Taffy 투영 전에 거부한다. `flex` negative test가 통과했다. |
| 9 | HTML `div`의 기본 UA display가 runtime과 Chromium에서 달라지는가 | fixture는 root에 display를 지정하지 않으며 고정 Chrome reference와 computed `block` 문자열이 일치한다. |
| 10 | auto width, nested auto height, source order가 Taffy에서 바뀌는가 | 고정 reference의 9개 node 각 frame을 비교한다. supported frame은 field별 오차 `0.5 CSS px` 안에 있고 DPR 1·2 결과가 같다. |
| 11 | layout 순서와 paint order가 서로 달라지는가 | scene box node 순서가 DOM preorder이며 실제 포함 box에만 연속 순번을 준다. 통합 runtime test가 7개 node 순서와 paint order를 확인한다. |
| 12 | 숨겨진 조상의 element descendant가 화면에 남는가 | `display:none` root와 nested subtree의 frame이 모두 0이고 scene에서 빠지는 test가 통과했다. |
| 13 | hidden subtree text가 보이는 text와 같은 오류를 내거나 잘못 paint되는가 | hidden text는 layout 입력에서 제외되고, visible text는 runtime layout 실패로 구분한다. 두 경로를 한 test에서 확인한다. |
| 14 | 숨겨진 root를 실패·빈 장면으로 혼동하는가 | hidden root는 `layout=ready`인 유효한 empty scene으로 처리한다. 별도 test에서 전 descendant frame 0과 box 0을 확인한다. |
| 15 | 폭 또는 높이가 0인 box가 보이는 작은 사각형으로 바뀌는가 | zero-width layout은 성공하지만 paint box는 생략하는 test를 추가해 통과했다. |
| 16 | transparent background가 아래 paint를 덮거나 layout box 자체를 없애는가 | `RuntimePaint::None` box는 DOM·layout 순서를 유지하고 GPU draw만 생략한다. 고정 fixture의 transparent group 이후 frame과 paint를 비교한다. |
| 17 | 불투명 RGB가 바뀌거나 partial alpha가 opaque로 둔갑하는가 | 6개 단색 palette와 alpha `0.5`를 검사한다. RGB는 typed paint로 보존하고 부분 alpha에서는 scene을 게시하지 않는다. |
| 18 | `color`·font computed 값이 glyph를 실제 그렸다는 의미로 잘못 처리되는가 | color/font-size/font-family 상속 문자열을 확인하지만 C08 scene은 배경 box만 그린다. 문서와 앱 설명은 glyph paint를 주장하지 않는다. |
| 19 | stale document/style/environment key의 계산 결과가 scene으로 통과하는가 | C08 전용 stale style revision test를 추가해 mismatch를 거부한다. layout/render 공통 admission도 document·tree·style·environment revision tuple을 검사한다. |
| 20 | 실제 V8·Android/iOS surface·근거 범위를 잘못 연결하거나 과장하는가 | API 37 Android emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 checked-in JS fixture, `layout=ready`, 7 boxes, WGPU presented를 확인했다. Android는 ANGLE/SwiftShader software backend다. screenshot 색상 표본은 paint 전달 근거이고 자식별 모바일 geometry oracle이나 hardware GPU 성능 자료는 아니다. |

## 검증 실행

- `cargo fmt --all -- --check`: 통과.
- `cargo test --locked -p spinon-runtime -p spinon-style -p spinon-style-to-layout -p spinon-style-to-render -p spinon-ffi`: 대상 5개 crate 통과.
- `node --test tools/css-reference/c08-block-flow.test.mjs`: 1/1 통과. Chromium·fixture·runtime JS·capture tool hash와 DPR 1·2 결과를 확인했다.
- Android/iOS 화면, 실행 로그와 screenshot SHA-256은 [시뮬레이터 근거](c08-block-flow-simulators-2026-10-10.md)에 있다.

검토 중 빠져 있던 성공 stylesheet, 거부 stylesheet, parse diagnostic, visible/hidden text, zero-area box 검사를 추가했다. 수정 뒤 관련 runtime test를 다시 실행해 통과했다.
