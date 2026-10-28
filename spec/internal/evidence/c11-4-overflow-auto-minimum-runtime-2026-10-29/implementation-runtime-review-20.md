# C11.4 overflow automatic minimum 구현/runtime 실패 관점 20개

검토 범위는 single-track 14-scene 경로의 cascade, host tree 검증, intrinsic input, Taffy trace, GPU draw/present, feature 경계다. 병렬 agent 검토가 아니라 같은 checkout에서 순차로 한 코드 검토다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 일반 빌드에 fixture catalog가 들어갈 수 있음 | fixture catalog와 runtime route는 `c11-4-grid-track-sizing-fixture-test` cfg 아래 있고 FFI는 `c04-runtime-gpu-fixture-test` opt-in으로 연결된다. 일반 `c04-runtime-gpu` archive에서는 신규 root ID와 image URL이 없다. PASS |
| 2 | 임의 scene ID가 고정 검증을 통과할 수 있음 | runtime catalog와 FFI scene catalog는 정확히 14개 ID를 고정한다. 등록되지 않은 prefix root와 FFI scene은 fail-closed다. PASS |
| 3 | 다른 profile이나 문서 root가 이 route에 진입할 수 있음 | HTML `div`, document root parent, `RuntimeBlockFlexGridTrackSizingV1`를 함께 확인한다. PASS |
| 4 | item이 빠지거나 중첩·추가되어 trace owner가 바뀔 수 있음 | Grid root의 direct child 하나, parent identity, 고정 item ID/tag와 leaf/text child 구조를 검사한다. PASS |
| 5 | 텍스트의 글꼴·내용 변화가 min-content 기대값을 오염할 수 있음 | 시스템 폰트 없는 `TextLayoutEngine`, 고정 Ahem, 10px 글꼴, 정확한 `XXXXXXXXXX` text와 horizontal writing mode를 검사한다. PASS |
| 6 | replaced image의 URL·자연 크기가 달라질 수 있음 | URL과 포함 PNG bytes를 고정하고 decode 결과가 10×10인지 확인한다. FFI image adapter는 row 7개 ID만 허용한다. PASS |
| 7 | axis authored overflow를 computed overflow와 혼동할 수 있음 | 각 case가 authored x/y와 computed x/y를 따로 보관한다. scrollable target 축의 반대축 `visible → auto`와 `clip`의 반대축 `visible`을 Android/iOS trace에서 대조한다. PASS |
| 8 | 지정 item minimum control을 automatic minimum 성공으로 셀 수 있음 | `min-width/height:0` control은 `SpecifiedMinimum(0)` source/value를 요구한다. PASS |
| 9 | 0 track minimum control이 item minimum 계산을 우회할 수 있음 | `minmax(0,1fr)` control은 target pass의 item Minimum query 부재를 요구한다. PASS |
| 10 | overflow 허용이 일반 CSS admission으로 새어 나갈 수 있음 | `overflow-x/y`는 C11.4 전용 fixture cascade override에만 추가되고 기본 track-sizing profile은 계속 거부한다. 기존 경계 테스트 통과. PASS |
| 11 | property 이름만으로 미지원 `visibility:collapse`가 허용될 수 있음 | 허용 값 검사를 inline과 author stylesheet preflight에 추가했다. 기존 inline negative test와 새 stylesheet negative test가 통과한다. PASS |
| 12 | `align-items:stretch`가 다른 profile에 전파될 수 있음 | projection option 기본값은 false이며 stretch 예외는 C11.4 profile과 전용 overflow fixture option이 함께 맞을 때만 적용된다. PASS |
| 13 | CSS computed Grid와 typed layout input이 어긋날 수 있음 | root style과 `GridContainerInput`에서 axis track, `minmax(auto|0,1fr)`, cross track, auto-flow, line names, repeat, template areas를 각각 고정 검증한다. PASS |
| 14 | image intrinsic revision이 layout과 render snapshot에서 달라질 수 있음 | row PNG decode가 만든 `IntrinsicSizeRevision`을 `CurrentLayoutInputs`에도 전달한다. 이전 AVD retry의 `LayoutIntrinsicSizeRevision` mismatch를 수정했고 최신 14-scene 행렬에서 통과한다. PASS |
| 15 | 최종 frame만 맞고 minimum source가 틀릴 수 있음 | visible/clip의 content-based, scrollable의 overflow automatic, specified control source/value와 zero-track query 부재를 trace에서 검증한다. PASS |
| 16 | flexible maximum이 예상하지 않은 intrinsic query를 할 수 있음 | target pass contribution은 고정 item·span 1·Minimum·flexible track 통과만 허용한다. MaxContent/MinContent와 다른 item 기여는 거부한다. PASS |
| 17 | trace에 축 누락·순서 오류·추가 subpass가 있을 수 있음 | 정확히 하나의 Grid trace, 두 축 존재, Columns 시작, contiguous sequence를 요구한다. 첫 검토에서 pass 수를 정확히 2로 제한했으나 `column-hidden-zero-minimum`의 정상 subpass를 거부했다. 실제 출력에 맞춰 반복 축 subpass는 허용하고 각 pass의 기여 계약을 모두 확인하도록 수정했다. Android/iOS 전체 행렬 통과. PASS |
| 18 | 추가·중복 frame 또는 오래된 revision이 최신 결과처럼 보일 수 있음 | frame count와 고유 node count가 모두 2인지 검사하고 style/document/render-tree, environment, intrinsic revision을 layout key와 연결한다. PASS |
| 19 | image asset이 scene에만 있고 실제 draw 경로에는 빠질 수 있음 | row 7개 scene의 node, URL, dimensions, frame과 resource revision을 검사해 `RuntimeRenderImage`를 붙인다. PixelCopy/Metal 로그가 image count와 draw count 각 1을 기록한다. PASS |
| 20 | 경계 밖 pixel 또는 simulator 결과를 과장할 수 있음 | Android PixelCopy와 iOS 제출 전 Metal drawable의 고정 좌표를 검사한다. visible row scene의 root 아래 red/black 표본을 확인하며 iOS post-compositor·실기기·clip/scroll 증거로 주장하지 않는다. PASS |

## 실행 근거

- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test`: 291 passed, 3 ignored, 0 failed.
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test runtime_gpu::fixtures::c11_4::overflow_auto_minimum`: 2 passed.
- `cargo clippy --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test --lib -- -D warnings`와 `cargo fmt --all -- --check` 통과.
- Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator가 각각 14/14 scene을 통과한다. 최신 원본은 이 디렉터리의 `android-avd-c114-overflow-auto-minimum-android-avd-20261028T163438739Z-7b4f0097` 및 `ios-simulator-c114-overflow-auto-minimum-ios-simulator-20261028T163550852Z-0719d120`다.
- Product/opt-in FFI archive marker 경계는 [`artifact-boundary.md`](./artifact-boundary.md)에 기록했다.
- C11.4 공식 항목은 미완료로 유지하며 공식 완료 수는 `72/213`이다. 이 runtime slice는 일반 CSS admission, clipping·scroll 동작 또는 전체 C11.4를 완료했다고 주장하지 않는다.
