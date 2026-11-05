# C11.1 intrinsic Grid percentage-gap 구현·runtime 적대 검토 20개

**검토 범위:** `5d47b06cc` 기준의 현재 변경 source, 9-scene Chrome 156 독립 expected, Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 실행, layout/runtime tests와 원본 logs. 기존 계획 검토 20개 및 계획 PR diff 검토 30개를 재사용하지 않았다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | percentage gap 허용이 모든 Grid 또는 Flex profile로 새어 나가는가 | Admission은 `RootHtmlIntrinsicTrackSizingV1` 경로의 `root_html_intrinsic_gap_is_supported`에서만 넓혔다. 다른 profile의 기존 zero-gap predicate는 그대로다. **통과**. |
| 2 | 기존 fixed-px gap이 basis가 없다는 이유로 깨지는가 | fixed `LengthPx`는 기존 값 자체를 used gap으로 검사하고 basis에 의존하지 않는다. fixed zero/control/row/column/both runtime 및 layout cases가 통과했다. **통과**. |
| 3 | row-gap과 column-gap basis가 교차되는가 | row는 container height, column은 width를 넘기고 120×80px 혼합 case에서 25% row=20px, 10% column=12px 및 item origins를 비교했다. **통과**. |
| 4 | border/padding/margin 때문에 fixed dimension이 content-box가 아닌가 | 해당 intrinsic profile은 양쪽 box-edge와 margin을 0으로 제한한다. fixture도 0이며 CSS basis와 고정 dimension이 같다. **통과**. |
| 5 | 퍼센트가 100 단위(% 값)와 fraction(0–1) 사이에서 100배 틀리는가 | typed `Percentage`는 Taffy percent fraction 그대로 전달한다. 10%, 25%, 120% 결과가 Chrome 독립 기대값과 맞는다. **통과**. |
| 6 | percentage 0이 기본 zero-gap geometry를 바꾸는가 | `0%` positive scene의 두 track 축과 item frames를 no-gap control과 비교했다. **통과**. |
| 7 | shorthand의 첫째/둘째 값 순서가 바뀌는가 | `gap:25% 10%`의 row/column basis가 각각 20px/12px로 기대값과 일치한다. **통과**. |
| 8 | `grid-gap` alias가 별도 resolver를 타는가 | alias scene과 shorthand mixed-basis scene에서 최종 frame이 동일한 독립 expected를 만족한다. **통과**. |
| 9 | 100% 초과 gap이 clamp되거나 지원 밖으로 잘못 분류되는가 | row와 column 각각 120%를 positive case로 확인했다. 두 번째 fixed item의 origin과 visible overflow가 보존된다. **통과**. |
| 10 | gutter가 track 개수만큼 생기거나 마지막에 추가되는가 | Taffy trace와 expected track/frame을 대조했다. 전체 gutter 검사는 `(explicit count - 1) × used gap`을 쓴다. **통과**. |
| 11 | fraction 사용 공간 계산에서 gutter가 빠지거나 두 번 빠지는가 | 100×100px axes의 min-content 40/20px과 10px gutter에서 flexible track 50/70px 및 item origin을 trace와 대조했다. **통과**. |
| 12 | resolved used gap이 f32 CSS px로 표현되지 않아 Taffy에 무한대가 들어가는가 | finite f32 percentage와 fixed basis의 product를 f64로 계산하고 used gap 자체가 f32 범위 안인지 검사한다. 단위/aggregate negative cases가 property-specific error로 끝났다. **통과**. |
| 13 | 개별 gap은 유한하지만 여러 gutter의 합이 overflow하는가 | 2·3 track typed percentage cases에서 aggregate overflow를 layout 전에 거부했다. runtime fixed-px overflow regression도 통과했다. **통과**. |
| 14 | 유한 gap 뒤 intrinsic track와 합쳐져 Taffy line position이 non-finite가 되는가 | 계산 후 frame collect 전에 해당 축의 detailed track positions를 검사하며 row/column 각각 track-plus-gutter negative를 실행했다. **통과**. |
| 15 | 음수/NaN/infinity typed 값이 zero나 성공으로 취급되는가 | fixed와 percentage typed negative/NaN/infinity는 `InvalidStyle`로 거부되고 Taffy 계산으로 가지 않는다. **통과**. |
| 16 | CSS가 표현할 수 없는 큰 percentage의 projection 거부가 CSS parse 오류나 layout 실패로 잘못 표시되는가 | `3e38%` mutation은 cascade `Ready`, layout 실패, `unsupported_computed_value`와 `column-gap` property, 미완성 output으로 분류됐다. **통과**. |
| 17 | CSS-valid `calc()` gap이 실수로 percentage support에 섞이는가 | 계산식은 cascade computed value로 남고 제한 profile이 `column-gap` diagnostic으로 atomic reject한다. **통과**. |
| 18 | CSS-invalid negative declaration이 unsupported computed value로 잘못 올라오는가 | `gap:-1px`에서 cascade fallback을 확인했고 기존 zero-gap runtime matrix가 유지된다. **통과**. |
| 19 | 실패 Grid가 같은 Root HTML 문서의 다른 성공 scene frame/paint를 남기는가 | oversized percentage/calc/extent rejection은 `completed=None`을 요구한다. Positive 9 scene에서는 27개 item이 모두 layout과 render snapshot에 있어야 한다. **통과**. |
| 20 | simulator 화면/soft GPU를 pixel parity나 실기기 증거로 과장하는가 | 양쪽 동일 source hash, `layout=ready boxes=48`, draw/present와 OS screenshot만 기록했다. AVD는 ANGLE/SwiftShader이고 PixelCopy·Metal drawable readback·WPT는 실행하지 않았다고 분리했다. **통과**. |

## 결과

20개의 서로 다른 구현/runtime 실패 관점을 대조했고 추가 수정이 필요한 percentage-gap 기능 결함은 찾지 못했다. Focused layout 8/8, runtime 11/11, Clippy, FFI check, fixture boundary 33/33 및 rustfmt가 통과했다. 전체 `spinon-layout --lib`는 450개 중 449개가 통과했다. 유일한 실패 C15 `host_document_collects_direct_text_and_inherited_spans_in_source_order`는 변경 전 fixed-px baseline의 448개 중 447개 run에서도 실패했다([baseline log](../c11-root-html-intrinsic-gap-runtime-2026-11-05/layout-full-suite.log)). fixture-test feature의 전체 runtime suite는 candidate 276 passed/8 failed/3 ignored, 기준 `5d47b06cc` 275 passed/8 failed/3 ignored이며 실패 테스트 집합이 동일하다([candidate log](./runtime-suite.log), [baseline log](./baseline-runtime-suite.log)). 새 broad-suite 실패는 없지만 두 전체 crate suite 모두 green은 아니다.

C11.1/C11/C11.4, 일반 CSS admission, WPT, Android 실기기/hardware GPU 및 iOS compositor 이후 pixel parity는 미완료다.
