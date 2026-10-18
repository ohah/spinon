# C04.12 기본 root HTML runtime 구현 실패 관점 검토 · 20개 · 2026-10-19

이 검토는 계획 review와 별도로 현재 구현 코드·회귀 입력·실행 결과를 대상으로 했다. 아래 20개는 서로 다른 입력·수명·변환·게시 실패 경로다.

## 발견 및 수정

`display:none` 부모 아래 `p { word-break: keep-all }`을 둔 입력에서 숨긴 자식의 스타일도 문단 스타일로 변환되어 `unsupported_root_html_default_render_style`로 문서 전체가 `Failed`가 됐다. `validate_visible_tree`가 반환하는 visible element 집합만 text 스타일로 변환하고, 숨김 자식은 레이아웃 입력에서도 제외하도록 고쳤다. 새 회귀 테스트와 기존 기본 runtime 테스트가 함께 통과한다.

## 검토 결과

| # | 실패 관점 | 확인 및 판정 |
| --- | --- | --- |
| 1 | C ABI에서 null, 빈 입력, 1 MiB 초과, malformed UTF-8을 잘못 파싱하거나 slice를 만들 수 있다. | `runtime_gpu/ffi.rs`에서 출력 인자와 source를 검증한 다음 `from_raw_parts`를 호출한다. 바이트 길이 상한과 UTF-8 오류를 생성 실패로 돌려보내며, FFI 경계 회귀 테스트 2개가 이 네 경우를 확인한다. |
| 2 | FFI source buffer가 host 수명 동안 보존된다고 잘못 가정하거나 비동기 사용 뒤 dangling될 수 있다. | host 생성 중 동기적으로 `ParsedRootHtml`로 파싱하고 기존 session 초기 문서로 소유시킨다. Android `byte[]`, iOS `NSData`는 호출 동안 살아 있다. |
| 3 | 앱의 일반 host API가 fixture API로 잘못 라우팅되거나 기본 제품 ABI에서 빠질 수 있다. | `RuntimeGpuHost::new_with_root_html`은 일반 `RuntimeSession::new_runtime_gpu_with_root_html`을 부른다. product archive에 일반 symbol이 있고 fixture symbol은 없다. |
| 4 | text layout 입력이 V8 HostDocument와 다른 두 번째 parse 결과나 다른 root generation을 사용할 수 있다. | cascade·layout·paint가 요청 snapshot의 root와 같은 source revision을 사용한다. `prepare`와 `attach`에서 generation/document/render-tree revision을 확인한다. |
| 5 | HTML quirks 문서나 비 HTML `documentElement`가 제한 renderer에서 정상 입력으로 오인될 수 있다. | parser의 no-quirks 생성과 HTML `html` root 검증이 별도로 존재한다. 실패는 scene publication 이전에 반환된다. |
| 6 | head/meta/style/script 또는 hidden subtree가 visible content로 그려질 수 있다. | UA stylesheet와 tree 검증이 비표시 요소를 분류하고 `display:none` 하위 트리에서 layout/text 결과를 만들지 않는다. |
| 7 | 숨긴 자식의 미지원 text property 하나가 visible 형제까지 모두 실패시킬 수 있다. | 실제로 재현해 수정했다. visible element ID만 paragraph/inline style로 변환한다. `c04_12_root_html_default_runtime` 2개가 통과한다. |
| 8 | visible SVG·외부 namespace·미지원 inline tag가 빈 텍스트나 엉뚱한 Block으로 조용히 바뀔 수 있다. | visible tree를 namespace/tag/display allowlist로 fail-closed한다. `span`, `strong`, `b`, `em`, `i` 외 visible inline tag는 node ID와 함께 거부한다. |
| 9 | display, color, font, line-height 같은 필수 computed property가 누락되어 기본값으로 조용히 대체될 수 있다. | `property`와 style projector가 누락·비지원 값을 구체 오류로 돌린다. 지원하지 않는 computed value는 fallback으로 바꾸지 않는다. |
| 10 | inline 자손의 font weight/style이 parent 값으로 덮이거나 중첩 run color가 잘못 상속될 수 있다. | inline ancestor 순서를 적용하고 text source range마다 computed run style을 만든다. 각 shaped run의 모든 source color가 같지 않으면 거부한다. |
| 11 | whitespace collapse, `nowrap`, `pre-line`, 방향, word-break의 computed 값을 다른 정책으로 그릴 수 있다. | 지원되는 collapse/wrap 조합만 enum으로 매핑하고 방향·word-break를 검증한다. 이 adapter가 지원하지 않는 값은 실패한다. |
| 12 | `line-break:auto`를 Parley가 노출하지 않는 모드로 가정해 코드상 범위를 숨길 수 있다. | 현재 slice는 `auto`를 고정 Strict로 매핑한다는 제한을 코드 주석과 계약 0081에 명시했다. full browser policy 동등성은 주장하지 않는다. |
| 13 | system font list, weight, italic/oblique가 Ahem fixture나 고정 폰트로 대체될 수 있다. | runtime engine은 Android/Apple system discovery를 켜고 fixture engine은 별도 생성자를 유지한다. product/positive-control archive에서 Ahem payload 경계를 다시 확인했다. |
| 14 | `line-height:normal`을 고정 배수로 계산해 font ascent/descent와 baseline이 어긋날 수 있다. | shaped run의 실제 font metrics로 normal line box를 구성한다. Simulator 화면과 draw log에서 text batch를 재확인했다. |
| 15 | NaN/무한 좌표, 과대 scale, 커다란 glyph bitmap이 raster나 scene에서 overflow될 수 있다. | scale·glyph 크기·bitmap/snapshot byte limit, 유한 좌표, glyph ID, source range 검증이 raster 경계에 있다. 실패는 `Result`로 전파된다. |
| 16 | surrogate/UTF-16 HostDocument source와 UTF-8 shaping range가 다른 글자를 가리킬 수 있다. | 각 text source를 HostDocument UTF-16에서 복원해 snapshot source text와 대조한다. source node 종류와 layout revision도 검증한다. |
| 17 | 이전 viewport/style/environment의 text snapshot을 새 scene key에 붙일 수 있다. | scene key와 computed style revision, environment revision, viewport bits, layout source revision, font registry token을 확인한다. |
| 18 | inline layout frame이 box painter에 들어가거나 hidden frame이 background box로 그려질 수 있다. | frame key가 HostDocument에 속하는지 확인하고 box paint 노드만 `layout.frames`에 남긴다. 기존 direct-text fixture positive-control도 통과한다. |
| 19 | text glyph가 owner box의 paint order·clip 밖에 추가되거나 일부 text만 남을 수 있다. | owner의 DrawBox 직후 DrawText를 붙이고 동일 clip을 적용·대조한다. source 누락, owner box 누락, attach 오류는 scene 전체 실패다. |
| 20 | Android/iOS가 다른 source, scheme, viewport를 쓰거나 빌드 전용 경로만 검증될 수 있다. | 동일 1,298-byte bundle fixture를 양 플랫폼에 넣어 수정 후 다시 빌드·실행했다. AVD는 ANGLE/SwiftShader이고 iOS Simulator는 Metal drawable이며 실기기·hardware GPU·compositor 후 픽셀 검증은 아니다. |

## 실행 근거

- 기본 runtime: `mise exec -- cargo test --locked -p spinon-runtime c04_12_root_html_default_runtime` · 2 passed, 185 filtered
- fixture positive-control: `mise exec -- cargo test --locked -p spinon-runtime --features c04-12-root-html-text-paint-fixture-test c04_12_root_html_text_paint_fixture_uses_parsed_text_and_owner_clips` · 1 passed, 190 filtered
- FFI 입력 거부: `mise exec -- cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test root_html_constructor_rejects` · 2 passed, 46 filtered
- `mise exec -- cargo check --locked -p spinon-ffi --features c04-runtime-gpu` · 통과
- Android API 37 AVD 및 iPhone 17 Pro / iOS 26.2 Simulator 재빌드·light/dark 실행·화면은 [README](./README.md), [log](./android/logcat.txt), [iOS log](./ios/unified-log.txt)에 있다.
- 기본/fixture-test archive hash, symbol, exact Ahem payload는 [artifact-boundary.txt](./artifact-boundary.txt)에 있다.

이 검토는 지원 subset을 검증한다. 물리 기기, Android hardware GPU, iOS compositor 이후 픽셀, 전체 HTML/CSS 및 Chromium/WPT 적합성의 증거가 아니다.

## 마지막 코드 델타 재확인

- Parley `FontData`의 Blob ID는 별도 shaping pass마다 달라질 수 있으므로 `TextRun::PartialEq`는 모든 visible run field에 더해 선택 face의 font bytes와 collection index를 비교한다. Clone 동등성만 유지하고 서로 다른 font resource를 같다고 보지 않는다.
- 이 수정 뒤 `spinon-text-layout` 전체 37개와 관련 layout/render/style/runtime/FFI package tests가 통과했다. 두 엔진에서 같은 입력으로 만든 snapshot equality와 glyph raster 결과를 함께 확인했다.
- 손상 font index의 assertion은 해당 입력이 실제 요청한 family인 `Roboto`를 기대하도록 정정했다. Android/iOS Simulator 앱도 최종 소스에서 다시 빌드·실행했다.
