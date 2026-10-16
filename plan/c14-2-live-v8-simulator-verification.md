# C14.2 · live V8 앱의 이미지 비율 sizing 검증 계획

## 목표와 경계

C14.2에서 이미 구현한 HTML img 자연 비율·지정 축·CSS 2.2 min/max·box-sizing 계산을 실제 Android 및 iOS 앱의 opt-in fixture 경로에서 재검증한다. V8이 만든 연결 문서 snapshot에서 이미지 요청을 구성하고, 고정 PNG bytes를 기존 decoder와 resource snapshot에 전달한 뒤, HostDocument → Stylo → Taffy 결과를 고정 Chrome 155 oracle과 비교한다.

이번 작업은 기존 C14.2 계산기를 다시 구현하거나 제품 이미지 loader를 추가하는 작업이 아니다. 합성 Rust fixture로 확인했던 계산 행렬이 live V8 문서와 앱 FFI를 통과하는지 확인하는 제한 검증이다. 네트워크 fetch, cache/retry, async resource replacement, 제품 stale scheduling, image pixel paint, WPT 전체 적합성, 공개 CSS 지원은 포함하지 않는다. 기본 제품 artifact에서는 fixture ABI와 고정 입력을 제외한다.

## 현재 공백과 사전 판정

- C14.2 구현 PR #156과 기존 Rust oracle은 Chrome 155 일반 흐름 21 case, Flex 8 case, Rust 회귀와 Android API 37 ARM64 AVD Rust test executable을 확인했다. Block inventory에는 natural PNG row가 22개 있지만 `aspect-ratio-auto-override`는 `aspect-ratio:auto <ratio>`를 사용한다. C14.2 원 계획과 구현은 이를 지원하지 않는 입력으로 명시했으므로 21-case 구현 행렬에서 제외한다([분모 정정 근거](../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/README.md)). 기존 AVD 증거는 앱/V8 통합을 실행하지 않았고, 현재 main에는 C14.2 앱 fixture route가 없다.
- Block 기준은 [replaced-ratio-sizing-precomparison.html](../tests/fixtures/css/c14/replaced-ratio-sizing-precomparison.html)과 [replaced-ratio-sizing-inventory.json](../tests/fixtures/css/c14/replaced-ratio-sizing-inventory.json), Flex 기준은 [replaced-ratio-sizing-flex-precomparison.html](../tests/fixtures/css/c14/replaced-ratio-sizing-flex-precomparison.html)과 [replaced-ratio-sizing-flex-inventory.json](../tests/fixtures/css/c14/replaced-ratio-sizing-flex-inventory.json)이다.
- 고정 비교 결과는 [Chrome 155 관측과 조건](../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/README.md), [Block observation](../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/chrome155-observation.json), [Flex observation](../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/chrome155-flex-observation.json)에 둔다. 기준 브라우저는 HeadlessChrome 155.0.8059.40, viewport 1280×633 CSS px, DPR 1이다.
- 완료 집합은 C14.2 구현·Rust oracle과 같은 Block 21 case 및 Flex 8 case, 총 29개다. Block inventory의 22번째 PNG row `aspect-ratio-auto-override`는 `aspect-ratio:auto <ratio>`라는 명시적 미지원 입력이라 분모에서 제외하며 inventory의 `layoutStage`에도 표시한다. 각 포함 case는 ID와 frame을 일대일 비교하고, 기본 tolerance는 frame 필드마다 0.05 CSS px다. inventory가 명시한 case별 tolerance를 우선한다. max-width 20px 분기의 원시 oracle 값 `19.984375×13.328125`는 그대로 report에 보존하고 정수 px로 먼저 반올림하지 않는다. 명시한 0.05 tolerance 안의 차이는 통과하므로 실제 `20px`도 허용될 수 있으며, 이는 oracle 기대값 자체를 `20px`로 덮어쓰는 것과 구분한다.
- 공통 PNG는 tests/fixtures/resources/c14-1-png-basic.png이며 24×16 pixel, SHA-256 5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2다. src는 고정 URL https://spinon.invalid/c14-2-png-basic.png만 허용한다. 모든 case에서 natural size 24×16 CSS px를 확인한다.
- 기존 Block observation의 image x/y/width/height와 Flex observation의 image 전체 frame 및 부모 크기/`align-items`를 비교한다. 고정 Flex observation은 부모 x/y를 기록하지 않으므로 실제 부모 전체 frame은 report에 보존하되 oracle 대조는 width/height로 한정한다. child image의 x/y까지 대조해 부모 배치 좌표가 달라지는 결과는 잡는다.

## fixture 계약

1. 전용 Cargo feature c14-2-app-fixture-test, native define SPINON_ENABLE_C14_2_APP_FIXTURE_TEST, Android/iOS build flag와 C ABI를 같은 기본-off 상태로 연결한다. Cargo feature는 C14.1 actor-owned live-V8 snapshot capability를 재사용하되 전용 C14.2 fixture symbol/payload만 추가한다. 전용 앱 실행 argument만 route를 연다. 잘못된 flag와 다른 fixture argument와의 충돌은 오류로 닫는다.
2. 고정 JavaScript fixture가 독립적인 RuntimeSession::eval 문서 2개를 만든다. Block 문서는 원본 fixture에서 C14.2가 지원하는 21개 img를 main 아래 직접 연결하고, Flex 문서는 하나의 main 아래 원본 8개 flex parent와 각 단일 img를 둔다. PNG inventory row인 `aspect-ratio-auto-override`는 양성 실행에 포함하지 않는다. 양쪽 문서는 text-free이고 각 포함 case ID·DOM 순서·PNG src를 고정 oracle/inventory와 대조한다. 스타일은 앱 adapter가 동일한 pinned stylesheet source로 전달하며 V8이 style element parsing을 지원한다고 가정하지 않는다.
3. 각 img의 ImageRequest는 해당 live snapshot에서 만든다. 각 문서의 decoded completion은 node·src·generation·document/render revision을 검증하고 문서별 증가된 IntrinsicSizeRevision을 가진 독립 resource snapshot으로 묶는다. C14.1/C14.2 style-layout adapter를 그대로 사용한다. 문서별 source stamp를 하나로 합치지 않는다.
4. 보고서는 포함한 29 case 각각의 oracle ID, DocumentGeneration/DocumentRevision/RenderTreeRevision, StyleRevision, EnvironmentRevision, IntrinsicSizeRevision, natural size, 해당 case의 frame, expected frame, tolerance, match/error를 기록한다. Chrome Block observation에는 parent frame이 없으므로 Block image full frame을 비교한다. Chrome Flex observation은 parent width/height와 `align-items`, image x/y/width/height를 제공하므로 이 값을 비교한다. 실제 Flex parent frame은 report에 보존하지만 oracle이 제공하지 않는 parent x/y는 독립 비교하지 않는다. status=ok는 모든 case가 통과하고 count가 29/29이며 각 문서의 resource/layout source stamp와 style/environment/intrinsic revision이 일치할 때만 반환한다. 누락·중복 case, frame 누락, report truncation은 성공으로 기록하지 않는다.
5. 출력 화면은 29/29 summary와 주요 경계 case를 읽을 수 있게 보여주고 전체 case 결과는 원본 report/log로 보존한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 최신 앱을 실행하고 screenshot을 따로 보존한다. 물리 기기는 연결·조회하지 않는다.
6. default Android APK/native library와 iOS linked app에서 C14.2 전용 FFI symbol·고유 JS/CSS/PNG payload가 없는지 확인한다. opt-in artifact는 같은 symbol/payload를 포함하는 positive control이다. 이 판정은 검사한 artifact에만 적용하며 release 전체나 제품 image loader 경계로 일반화하지 않는다.

## 포함/제외 case

- 21개 Block PNG case는 natural auto size, 한 축 지정과 ratio transfer, min/max 제약, CSS 2.2 10개 조합, min/max conflict, 부분 auto 제약, border-box edge, authored numeric ratio와 aspect-ratio:auto를 포함한다.
- Flex 8개 case는 지정 main/cross 축의 min/max, natural flex auto-basis, definite cross-axis stretch와 max-height, stretch가 없는 definite cross-axis를 포함한다.
- SVG ratio-only 관찰 2개는 PNG decoder adapter 대상이 아니므로 완료 분모에서 제외한다. SVG no-dimensions의 300×150 fallback은 synthetic metadata fixture가 소유한다. missing-src와 broken-PNG는 성공 metadata와 구분하는 실패 대조이며 29개 양성 분모에 넣지 않는다. 이들을 빈 metadata entry로 바꾸거나 자연 크기를 합성해 양성 case로 만들지 않는다.
- Grid, flex grow/shrink 이후 cross-axis 재측정, percentage/calc sizing, CSS aspect-ratio와 min/max 조합, decoder/network/cache/replacement, image paint는 기존 C14.2 범위를 확장하지 않으며 이 작업에서도 구현하지 않는다.

## 실행 순서와 통과 기준

1. 이 계획을 구현 전 별도 20개 실패 관점으로 검토하고 P1/P2를 반영한다. 분모 정정 뒤에는 [범위 정정 계획 재검토 20개](../spec/internal/evidence/c14-2-live-v8-scope-correction-plan-review-20-2026-10-16.md)처럼 독립 재검토를 추가한다. 계획 검토는 구현·PR 검토와 독립 기록한다.
2. 기존 Chrome 155 fixture/inventory/observation/PNG hash를 입력으로 고정하고 구현 행렬과 같은 21+8 외 행이 포함되지 않는지 확인한다. oracle 결과를 새로 측정할 경우 Agent Browser를 사용하고 terminal-browser는 사용하지 않는다.
3. opt-in 앱 fixture와 V8 document, fixed stylesheet, C14.2 FFI route를 추가한다. fixture 전용 소스와 payload는 Cargo default feature에서 분리한다.
4. Android API 37 ARM64 AVD와 iOS 26.2 Simulator에서 각 29/29 비교, natural size, full frame, revisions, error/status를 확인한다. 한 플랫폼을 다른 플랫폼의 증거로 대신하지 않는다.
5. default/opt-in Android 및 iOS artifact 경계를 각각 검사하고, 전용 FFI symbol·고유 payload의 negative/positive controls를 기록한다.
6. workspace tests, Clippy, rustfmt, diff/native build-script checks와 변경된 fixture-boundary checks를 실행한다. 실패는 원인을 보존하고 최종 통과 결과를 별도로 남긴다.
7. 계획 표와 다른 독립 구현 실패 관점 20개를 적용해 결함을 수정하고 영향을 다시 확인한다. PR 변경 diff에는 별도의 적대적 관점 30개를 기록한다. 리뷰에서 미해결 P1/P2를 허용하지 않는다.
8. 모든 원래 C14.2 완료 기준과 29-case simulator gate가 충족된 경우에만 C14.2 행을 완료로 갱신한다. C14.1, 상위 C14, U05, S02 제품 runtime owner, X16은 자동 완료하지 않는다. 미완료 게이트가 남으면 체크를 유지하고 구체적인 원인을 기록한다.

## 산출물

- 구현 전 oracle와 계획 검토 20개
- opt-in Android/iOS live V8 runtime route 및 default/positive artifact boundary evidence
- 플랫폼별 29-case report, raw logs, screenshots, device/simulator metadata와 artifact hashes
- 계획과 다른 구현 검토 20개 및 별도 PR 변경 검토 30개
- 원래 완료 기준을 충족한 경우에 한해 C14.2 상태·계약·내부 evidence index 갱신

## 실행 결과 (2026-10-16)

- iPhone 17 Pro / iOS 26.2 Simulator와 `spinon_api37_1_compat16k` API 37 ARM64 AVD에서 각 Block 21/21, Flex 8/8, 총 29/29 live V8 case가 통과했다. 고정 PNG natural size는 두 실행 모두 24×16 CSS px이며 report에 전체 frame·source stamp·revision을 보존했다.
- 두 앱 모두 report text를 표시한다. C14.2 `content-box-edges`와 `aspect-ratio-border-box`는 border/padding이 반영된 CSS frame을 Chrome oracle과 비교한다. native image paint와 border 바깥 pixel overflow/clip은 검사하지 않는다.
- Default Android APK/native library와 iOS Simulator static/app executable에서 C14.2 FFI symbol·고정 case label/ID·PNG bytes가 없었다. Opt-in build에서는 같은 payload가 있었고 default route argument는 양 플랫폼에서 `SPINON_C142_DISABLED`로 거부됐다. 자세한 hash·원본 log·화면은 [실행 근거](../spec/internal/evidence/c14-2-live-v8-runtime-2026-10-16/README.md)와 [artifact 경계](../spec/internal/evidence/c14-2-live-v8-runtime-2026-10-16/artifact-boundary/README.md)에 둔다.
- 오류 실행에서 발견한 PNG metadata JSON 객체 파싱과 C14.2 `aspect-ratio` 계산 profile을 수정했다. 제한 V8 runtime은 HTML UA stylesheet를 로드하지 않으므로 `<main>`의 Chrome UA `display:block` 기본값을 fixture cascade에 명시했다.
- 이 gate는 고정 PNG와 sizing geometry를 검증한다. 제품 resource loader·network/cache/async replacement, image pixel paint/clip, 실제 기기와 WPT 전체 적합성은 증명하지 않는다.

## 분모 정정 검토

inventory에는 browser row 27개와 natural PNG row 22개가 있지만, C14.2 구현/Rust oracle의 Block 지원 집합은 21개다. `aspect-ratio-auto-override`는 원 계획에서 fail-closed로 정한 `aspect-ratio:auto <ratio>`이므로 21개 구현 행렬에서 제외한다. inventory marker, Chrome observation, 기존 21+8 case를 대조한 별도 [계획 재검토 20개](../spec/internal/evidence/c14-2-live-v8-scope-correction-plan-review-20-2026-10-16.md)와 [PR 변경 검토 30개](../spec/internal/evidence/c14-2-live-v8-scope-correction-pr-delta-review-30-2026-10-16.md)를 남긴다.
